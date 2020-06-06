# hibitgraph

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
bitgraph = "0.1"
```

## License

This library is licensed under the Apache License 2.0,
see [the LICENSE file][li] for more information.

[li]: LICENSE