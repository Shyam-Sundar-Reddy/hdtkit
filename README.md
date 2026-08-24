# hdtkit

A small CLI/package to convert between RDF Turtle (`.ttl`) and HDT (`.hdt`):

- `.ttl` → `.hdt`
- `.hdt` → `.ttl`

## Install (dev)

```bash
pip install -e ".[dev]"
```

## Usage

```bash
hdtkit to-hdt input.ttl output.hdt
hdtkit to-ttl input.hdt output.ttl
```

## Status

Early scaffold — conversion logic is not implemented yet.
