# goslop

goslop is a Rust-based static analyzer for Go repositories that focuses on signals commonly associated with low-context AI-generated code. It currently scans a repository, parses Go files with tree-sitter-go, extracts structural fingerprints for each function, builds a lightweight local package index, runs early heuristic checks, and can benchmark the pipeline against real Go repositories.

## Overview

The current implementation is optimized around a fast full-repository pass:

- walk the target tree with `.gitignore` awareness
- skip common generated-code inputs and `vendor/` paths
- parse Go files with tree-sitter-go
- fingerprint functions and methods with lightweight structural metrics
- flag generic naming and weak typing patterns
- use a local package index to catch some unresolved repository-local calls
- benchmark discovery, parse, index, heuristic, and total runtime stages

## Commands

Run a scan against a target path:

```bash
cargo run -- scan /path/to/go-repo
```

Run the same scan with JSON output:

```bash
cargo run -- scan --json /path/to/go-repo
```

Run a scan without `.gitignore` filtering:

```bash
cargo run -- scan --no-ignore /path/to/go-repo
```

Benchmark the current pipeline against a real local Go repository:

```bash
cargo run -- bench /path/to/go-repo
```

Benchmark with explicit repeats and warmups:

```bash
cargo run -- bench --warmups 2 --repeats 5 /path/to/go-repo
```

Benchmark with JSON output:

```bash
cargo run -- bench --json /path/to/go-repo
```

## Development

Run the test suite:

```bash
cargo test
```

For a detailed architecture and roadmap guide, see `guides/implementation-guide.md`.
