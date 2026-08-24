//! Phase 5: `hdtcat` — combine 2+ HDT files into one, de-duplicating triples.
//!
//! Built directly on the Phase 3 read path and Phase 4 write path.
//! `hdt.triples_all()` already yields triples as `[Arc<str>; 3]` in exactly the
//! dictionary string format `Hdt::from_triples` wants — reused directly, no
//! detour through `oxrdf` terms and back like `hdt_read`/`hdt_write` need for
//! Turtle. A `BTreeSet` gives triple-level de-dup and a stable (sorted) input
//! order for free.

use hdt::Hdt;
use std::collections::BTreeSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub fn hdtcat(inputs: &[impl AsRef<Path>], output: &Path) -> Result<(), Box<dyn Error>> {
    if inputs.len() < 2 {
        return Err("hdtcat requires at least 2 input .hdt files".into());
    }
    let mut triples = BTreeSet::new();
    for input in inputs {
        let hdt = Hdt::read(BufReader::new(File::open(input)?))?;
        triples.extend(hdt.triples_all());
    }
    let dataset_iri = format!("file://{}", output.display());
    let hdt = Hdt::from_triples(triples, &dataset_iri)?;
    let mut writer = BufWriter::new(File::create(output)?);
    hdt.write(&mut writer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use hdt::sophia::api::graph::Graph;

    fn write_hdt(dir: &Path, name: &str, ttl: &str) -> std::path::PathBuf {
        let ttl_path = dir.join(format!("{name}.ttl"));
        let hdt_path = dir.join(format!("{name}.hdt"));
        std::fs::write(&ttl_path, ttl).unwrap();
        crate::hdt_write::ttl_to_hdt(&ttl_path, &hdt_path, None).unwrap();
        hdt_path
    }

    #[test]
    fn merges_and_dedups() {
        let dir = std::env::temp_dir();
        let a = write_hdt(&dir, "hdtkit_test_merge_a", "@prefix ex: <http://example.org/> .\nex:a ex:knows ex:b .\nex:shared ex:p ex:o .\n");
        let b = write_hdt(&dir, "hdtkit_test_merge_b", "@prefix ex: <http://example.org/> .\nex:c ex:knows ex:d .\nex:shared ex:p ex:o .\n");
        let out = dir.join("hdtkit_test_merge_out.hdt");

        hdtcat(&[&a, &b], &out).unwrap();

        let hdt = Hdt::read(BufReader::new(File::open(&out).unwrap())).unwrap();
        // 3 distinct triples: a-knows-b, c-knows-d, shared-p-o (deduped, not 4).
        assert_eq!(hdt.triples().count(), 3);

        for f in [&a, &b, &out] {
            std::fs::remove_file(f).ok();
        }
    }

    #[test]
    fn requires_at_least_two_inputs() {
        let dir = std::env::temp_dir();
        let a = write_hdt(&dir, "hdtkit_test_merge_single", "@prefix ex: <http://example.org/> .\nex:a ex:b ex:c .\n");
        let out = dir.join("hdtkit_test_merge_single_out.hdt");
        assert!(hdtcat(&[&a], &out).is_err());
        std::fs::remove_file(&a).ok();
    }
}
