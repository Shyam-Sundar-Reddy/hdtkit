//! Phase 2: TTL parsing. Internal only — no Python-visible behavior yet.
//! Reuses `oxrdf::Triple` directly rather than a custom triple type (nothing
//! about our needs requires a different shape).

use oxttl::TurtleParser;
use std::io::Read;
use std::path::Path;

pub fn parse_ttl_reader<R: Read>(
    reader: R,
    base_iri: Option<&str>,
) -> Result<Vec<oxrdf::Triple>, Box<dyn std::error::Error>> {
    let mut parser = TurtleParser::new();
    if let Some(base_iri) = base_iri {
        parser = parser.with_base_iri(base_iri)?;
    }
    parser.for_reader(reader).map(|r| r.map_err(Into::into)).collect()
}

pub fn parse_ttl(
    path: &Path,
    base_iri: Option<&str>,
) -> Result<Vec<oxrdf::Triple>, Box<dyn std::error::Error>> {
    parse_ttl_reader(std::io::BufReader::new(std::fs::File::open(path)?), base_iri)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_triples() {
        let ttl = b"@prefix ex: <http://example.org/> .\nex:a ex:knows ex:b .\n";
        let triples = parse_ttl_reader(&ttl[..], None).unwrap();
        assert_eq!(triples.len(), 1);
        assert_eq!(triples[0].subject.to_string(), "<http://example.org/a>");
    }

    #[test]
    fn resolves_relative_iris_against_base() {
        let ttl = b"<a> <b> <c> .\n";
        let triples = parse_ttl_reader(&ttl[..], Some("http://example.org/")).unwrap();
        assert_eq!(triples[0].subject.to_string(), "<http://example.org/a>");
    }
}
