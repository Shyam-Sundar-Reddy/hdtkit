# Phase 0 spike (throwaway)

Proves the two read-side building blocks decided in `phase.md` before the real
crate (`rust/hdtkit-core/`, Phase 1) gets written:

1. Read an existing `.hdt` file with the [`hdt`](https://crates.io/crates/hdt) crate.
2. Parse `.ttl` with [`oxttl`](https://crates.io/crates/oxttl).

Run: `cargo run` from this directory.

`snikmeta.hdt` is a test fixture copied from the `hdt` crate's own test suite
(MIT licensed, https://github.com/konradhoeffner/hdt), used here only to have a
real `.hdt` file to load without needing a working `ttl2hdt` yet.
