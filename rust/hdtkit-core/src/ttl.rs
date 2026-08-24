//! Phase 2: TTL parsing. Internal only — no Python-visible behavior yet.
//! Reuses `oxrdf::Triple` directly rather than a custom triple type (nothing
//! about our needs requires a different shape).

use oxttl::TurtleParser;
use std::io::Read;
use std::path::Path;

pub fn parse_ttl_reader<R: Read>(reader: R) -> Result<Vec<oxrdf::Triple>, Box<dyn std::error::Error>> {
    TurtleParser::new()
        .for_reader(reader)
        .map(|r| r.map_err(Into::into))
        .collect()
}

pub fn parse_ttl(path: &Path) -> Result<Vec<oxrdf::Triple>, Box<dyn std::error::Error>> {
    parse_ttl_reader(std::io::BufReader::new(std::fs::File::open(path)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_triples() {
        let ttl = b"@prefix ex: <http://example.org/> .\nex:a ex:knows ex:b .\n";
        let triples = parse_ttl_reader(&ttl[..]).unwrap();
        assert_eq!(triples.len(), 1);
        assert_eq!(triples[0].subject.to_string(), "<http://example.org/a>");
    }
}
