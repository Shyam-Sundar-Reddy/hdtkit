# hdtkit

An importable Python package to convert between RDF Turtle (`.ttl`) and HDT (`.hdt`):

- `.ttl` → `.hdt`
- `.hdt` → `.ttl`
- combine two or more `.hdt` files into one

No CLI — `import hdtkit` is the interface.

## Install (dev)

```bash
pip install -e ".[dev]"
```

## Usage

```python
from hdtkit import ttl_to_hdt, hdt_to_ttl, merge_hdt

ttl_to_hdt("graph.ttl", "graph.hdt")
hdt_to_ttl("graph.hdt", "graph.ttl")
merge_hdt(["a.hdt", "b.hdt"], "combined.hdt")
```

## Status

Early scaffold. The public API shape above is stable, but conversion is not
implemented yet — each function currently raises `NotImplementedError`. The
conversion core is being built in Rust (exposed via PyO3); see `phase.md` for
the build plan (not tracked in this repo).
