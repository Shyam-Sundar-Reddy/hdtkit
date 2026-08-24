//! Phase 0 spike: prove the two read-side building blocks work before writing
//! any real crate code.
//!
//! 1. Read an existing .hdt file with the `hdt` crate and iterate triples.
//! 2. Parse a .ttl string with `oxttl` and iterate triples.
//!
//! Not shipped — this binary exists only to validate the Phase 0 decision in
//! `phase.md`. Run with: `cargo run` from `rust/spike/`.

use hdt::Hdt;
use oxttl::TurtleParser;
use std::fs::File;
use std::io::BufReader;

fn spike_hdt_read() {
    let file = File::open("snikmeta.hdt").expect("error opening snikmeta.hdt");
    let hdt = Hdt::read(BufReader::new(file)).expect("error loading HDT");

    let count = hdt.triples_all().count();
    println!("[hdt] loaded snikmeta.hdt, {count} triples total");

    for triple in hdt.triples_all().take(3) {
        println!("[hdt] {triple:?}");
    }
    assert!(count > 0, "expected at least one triple in snikmeta.hdt");
}

fn spike_ttl_parse() {
    let ttl = r#"
        @prefix ex: <http://example.org/> .
        ex:alice ex:knows ex:bob .
        ex:bob ex:knows ex:carol .
    "#;

    let mut count = 0;
    for result in TurtleParser::new().for_reader(ttl.as_bytes()) {
        let triple = result.expect("ttl parse error");
        println!("[ttl] {triple}");
        count += 1;
    }
    println!("[ttl] parsed {count} triples");
    assert_eq!(count, 2, "expected 2 triples from the inline ttl sample");
}

fn main() {
    spike_hdt_read();
    spike_ttl_parse();
    println!("\nPhase 0 spike OK: hdt-crate read works, oxttl parse works.");
    println!(
        "Confirmed from hdt crate docs: it is READ-ONLY (no HDT writing/serialization) — \
         see rust/hdtkit-core notes in phase.md for the write-path decision this implies."
    );
}
