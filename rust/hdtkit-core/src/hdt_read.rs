//! Phase 3: HDT read path — `hdt2ttl`.
//!
//! Reuses the `hdt` crate's typed Sophia terms (`Graph::triples()`, feature
//! `sophia` which is on by default) instead of hand-parsing HDT's raw
//! NTriples-style strings, and `oxttl`'s `TurtleSerializer` to write output.
//! `hdt` is read-only (see phase.md ADR) — this is the direction it covers.

use hdt::hdt_graph::HdtTerm;
use hdt::sophia::api::graph::Graph;
use hdt::Hdt;
use oxrdf::{BlankNode, Literal, NamedNode, NamedOrBlankNode, Term, Triple};
use oxttl::TurtleSerializer;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

fn to_term(t: &HdtTerm) -> Term {
    match t {
        HdtTerm::Iri(i) => NamedNode::new_unchecked(i.as_str()).into(),
        HdtTerm::BlankNode(b) => BlankNode::new_unchecked(b.as_str()).into(),
        HdtTerm::LiteralLanguage(lex, tag) => {
            Literal::new_language_tagged_literal_unchecked(lex.as_ref(), tag.as_str()).into()
        }
        HdtTerm::LiteralDatatype(lex, dt) => {
            Literal::new_typed_literal(lex.as_ref(), NamedNode::new_unchecked(dt.as_str())).into()
        }
    }
}

fn to_subject(t: &HdtTerm) -> Result<NamedOrBlankNode, Box<dyn Error>> {
    match to_term(t) {
        Term::NamedNode(n) => Ok(NamedOrBlankNode::NamedNode(n)),
        Term::BlankNode(b) => Ok(NamedOrBlankNode::BlankNode(b)),
        other => Err(format!("HDT subject must be an IRI or blank node, got {other}").into()),
    }
}

pub fn hdt_to_ttl(input: &Path, output: &Path) -> Result<(), Box<dyn Error>> {
    let hdt = Hdt::read(BufReader::new(File::open(input)?))?;
    let mut writer = TurtleSerializer::new().for_writer(BufWriter::new(File::create(output)?));
    for triple in hdt.triples() {
        let [s, p, o] = triple.expect("hdt triple iteration is infallible");
        let subject = to_subject(&s)?;
        let predicate = match to_term(&p) {
            Term::NamedNode(n) => n,
            other => return Err(format!("HDT predicate must be an IRI, got {other}").into()),
        };
        let object = to_term(&o);
        writer.serialize_triple(Triple::new(subject, predicate, object).as_ref())?;
    }
    writer.finish()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_snikmeta_hdt_to_ttl() {
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("../spike/snikmeta.hdt");
        let out = std::env::temp_dir().join("hdtkit_test_snikmeta.ttl");
        hdt_to_ttl(&fixture, &out).unwrap();
        let ttl = std::fs::read_to_string(&out).unwrap();
        assert!(!ttl.is_empty());
        // Round-trip: re-parse what we wrote and confirm the triple count matches.
        let reparsed = crate::ttl::parse_ttl(&out, None).unwrap();
        let hdt = Hdt::read(BufReader::new(File::open(&fixture).unwrap())).unwrap();
        assert_eq!(reparsed.len(), hdt.triples_all().count());
        std::fs::remove_file(&out).ok();
    }
}
