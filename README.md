# hibitgraph
[![Build Status](https://travis-ci.org/jmeintrup/hibitgraph.svg)](https://travis-ci.org/jmeintrup/hibitgraph)
[![Crates.io](https://img.shields.io/crates/v/hibitgraph)](https://crates.io/crates/hibitgraph)

Provides a very fast and space-efficient graph data structure for specific use cases.
When to use:
 - You know the maximum size your graph can take
 - You have at most `mem::size_of::<usize>.pow(4)` vertices
 - Your graph is undirected and has no values/weights associated with vertices or edges

Provided Functionality:
 - Constant time adding/removing edges
 - Fast DFS Iteration
 - Fast edge contractions
 
Internally the graph stores a vector containing multiple [hibitset::BitSet](https://docs.rs/hibitset/0.6.3/hibitset/struct.BitSet.html)

## Usage

Just add this to your `Cargo.toml`:

```toml
[dependencies]
hibitgraph = "0.1"
```

## License

This library is licensed under dual MIT/Apache License v2.0,
see the LICENSE files ([MIT][mit] and [Apache-v2.0][apache]) for more information.

[apache]: LICENSE-APACHE
[mit]: LICENSE-MIT