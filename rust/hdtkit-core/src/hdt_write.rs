//! Phase 4: HDT write path — `ttl2hdt`.
//!
//! The Phase 0 ADR assumed the `hdt` crate was strictly read-only and planned a
//! hand-rolled dictionary/bitmap encoder for this phase. Closer reading found the
//! crate does have a real writer behind its `nt` feature: `Hdt::from_triples`
//! builds a proper compressed FourSectionDictionary + BitmapTriples HDT from
//! in-memory triples, and `Hdt::write` serializes the whole file. That's reused
//! here instead — no hand-rolled binary encoding needed after all.
//!
//! `Hdt::from_triples` wants each term as one string in HDT's dictionary format:
//! IRIs *without* angle brackets, blank nodes as `_:id`, and literals exactly as
//! NTriples prints them (quoted, `@lang` or `^^<datatype>` suffix). `oxrdf`'s
//! `Display` for `Literal`/`BlankNode` already produces that; only `NamedNode`
//! needs unwrapping (`Display` adds angle brackets we must drop).

use hdt::Hdt;
use oxrdf::{NamedOrBlankNode, Term, Triple};
use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

fn subject_to_hdt_string(s: &NamedOrBlankNode) -> String {
    match s {
        NamedOrBlankNode::NamedNode(n) => n.as_str().to_owned(),
        NamedOrBlankNode::BlankNode(b) => b.to_string(),
    }
}

fn term_to_hdt_string(t: &Term) -> Result<String, Box<dyn Error>> {
    match t {
        Term::NamedNode(n) => Ok(n.as_str().to_owned()),
        Term::BlankNode(b) => Ok(b.to_string()),
        Term::Literal(l) => Ok(l.to_string()),
        #[allow(unreachable_patterns)] // RDF-star triple terms, not enabled/produced by our TTL parser
        other => Err(format!("unsupported term for HDT object: {other:?}").into()),
    }
}

fn triple_to_hdt_strings(t: &Triple) -> Result<[String; 3], Box<dyn Error>> {
    Ok([
        subject_to_hdt_string(&t.subject),
        t.predicate.as_str().to_owned(),
        term_to_hdt_string(&t.object)?,
    ])
}

/// `base_iri` resolves relative IRIs while parsing the input `.ttl` (absent if
/// the file has none). This is unrelated to the HDT dataset identifier that
/// `Hdt::from_triples` also wants for its header — we synthesize that
/// separately from the output path since we don't expose it to Python.
pub fn ttl_to_hdt(input: &Path, output: &Path, base_iri: Option<&str>) -> Result<(), Box<dyn Error>> {
    let triples = crate::ttl::parse_ttl(input, base_iri)?;
    let hdt_triples =
        triples.iter().map(triple_to_hdt_strings).collect::<Result<Vec<_>, _>>()?;
    let dataset_iri = format!("file://{}", output.display());
    let hdt = Hdt::from_triples(hdt_triples, &dataset_iri)?;
    let mut writer = BufWriter::new(File::create(output)?);
    hdt.write(&mut writer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use hdt::sophia::api::graph::Graph;

    #[test]
    fn round_trips_ttl_to_hdt_to_ttl() {
        let dir = std::env::temp_dir();
        let ttl_in = dir.join("hdtkit_test_roundtrip_in.ttl");
        let hdt_out = dir.join("hdtkit_test_roundtrip.hdt");

        std::fs::write(
            &ttl_in,
            "@prefix ex: <http://example.org/> .\n\
             ex:alice ex:knows ex:bob .\n\
             ex:alice ex:name \"Alice\"@en .\n\
             ex:bob ex:age \"42\"^^<http://www.w3.org/2001/XMLSchema#integer> .\n",
        )
        .unwrap();

        ttl_to_hdt(&ttl_in, &hdt_out, None).unwrap();

        let hdt = Hdt::read(std::io::BufReader::new(File::open(&hdt_out).unwrap())).unwrap();
        assert_eq!(hdt.triples().count(), 3);

        std::fs::remove_file(&ttl_in).ok();
        std::fs::remove_file(&hdt_out).ok();
    }
}
