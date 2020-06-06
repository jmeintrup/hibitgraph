//! # hibitgraph
//
// Provides a very fast and space-efficient graph data structure for specific use cases.
// When to use:
//  - You know the maximum size your graph can take
//  - You have at most `mem::size_of::<usize>.pow(4)` vertices
//  - Your graph is undirected and has no values/weights associated with vertices or edges
//
// Provided Functionality:
//  - Constant time adding/removing edges
//  - Fast DFS Iteration
//  - Fast edge contractions
//
// Internally the graph stores a vector containing multiple [hibitset::BitSet](https://docs.rs/hibitset/0.6.3/hibitset/struct.BitSet.html)

use bit_set;
use hibitset;
use hibitset::{BitIter, BitSetLike, DrainableBitSet};
use std::mem;

const MAX_CAPACITY: usize = mem::size_of::<usize>()
    * mem::size_of::<usize>()
    * mem::size_of::<usize>()
    * mem::size_of::<usize>();

/// A `BitGraph` is an undirected graph data structure
/// Its capacity is limited to `mem::size_of::<usize>.pow(4)`
#[derive(Debug, Clone)]
pub struct BitGraph {
    m_data: Vec<hibitset::BitSet>,
    m_degrees: Vec<u32>,
    m_order: u32,
}

impl BitGraph {
    /// Creates a new BitGraph preallocated with up to `capacity` vertices
    /// It is not possible later add vertices >= `capacity`
    pub fn with_capacity(capacity: u32) -> BitGraph {
        Self::check_capacity(capacity);
        BitGraph {
            m_data: vec![hibitset::BitSet::with_capacity(capacity); capacity as usize],
            m_degrees: vec![0; capacity as usize],
            m_order: 0,
        }
    }

    #[inline]
    fn check_capacity(capacity: u32) {
        if capacity > MAX_CAPACITY as u32 {
            panic!(
                "Out of bounds. Given: {}, Allowed: {}",
                capacity, MAX_CAPACITY
            )
        }
    }

    #[inline]
    fn check_bounds(&self, idx: u32) {
        if idx >= self.m_degrees.len() as u32 {
            panic!(
                "Out of bounds. Given: {}, Allowed: {}",
                idx,
                self.m_degrees.len()
            )
        }
    }

    #[inline]
    fn check_is_same(&self, u: u32, v: u32) {
        if u == v {
            panic!("Edge needs two distinct endpoints, given: {} {}", u, v)
        }
    }

    /// Creates a new BitGraph with `capacity` vertices, with all vertices connected to each other.
    /// It is not possible later add vertices >= `capacity`
    pub fn complete(capacity: u32) -> BitGraph {
        Self::check_capacity(capacity);
        let mut m_data = vec![hibitset::BitSet::with_capacity(capacity); capacity as usize];
        for bs in m_data.iter_mut() {
            for idx in 0..capacity {
                bs.add(idx);
            }
        }
        BitGraph {
            m_data,
            m_degrees: vec![capacity; capacity as usize],
            m_order: capacity,
        }
    }

    /// Adds a new undirected edge from `u` to `v`
    /// If the edge already exists, the graph is not updated
    /// It is not possible to add edges with endpoints >= `capacity`
    pub fn add_edge(&mut self, u: u32, v: u32) {
        self.check_bounds(v);
        self.check_bounds(u);
        self.check_is_same(u, v);
        self.add_endpoint(u, v);
        self.add_endpoint(v, u);
    }

    /// Same as `add_edge` except that no boundary checks are performed.
    /// Can corrupt the underlying data
    pub fn add_edge_unchecked(&mut self, u: u32, v: u32) {
        self.add_endpoint_unchecked(u, v);
        self.add_endpoint_unchecked(v, u);
    }

    /// Removes the edge from `u` to `v` after performing boundary checks.
    /// If the edge is not present the graph is not updated
    pub fn remove_edge(&mut self, u: u32, v: u32) {
        self.check_bounds(v);
        self.check_bounds(u);
        self.check_is_same(u, v);
        self.remove_endpoint(u, v);
        self.remove_endpoint(v, u);
    }

    /// Same as `remove_edge` except that no boundary checks are performed
    /// Can corrupt the underlying data
    pub fn remove_edge_unchecked(&mut self, u: u32, v: u32) {
        self.remove_endpoint_unchecked(u, v);
        self.remove_endpoint_unchecked(v, u);
    }

    fn add_endpoint(&mut self, u: u32, v: u32) {
        if !self.m_data.get_mut(u as usize).unwrap().add(v) {
            if self.m_degrees[u as usize] == 0 {
                self.m_order += 1;
            }
            self.m_degrees[u as usize] += 1;
        }
    }

    fn add_endpoint_unchecked(&mut self, u: u32, v: u32) {
        self.m_data.get_mut(u as usize).unwrap().add(v);
        if self.m_degrees[u as usize] == 0 {
            self.m_order += 1;
        }
        self.m_degrees[u as usize] += 1;
    }

    fn remove_endpoint(&mut self, u: u32, v: u32) {
        if self.m_data.get_mut(u as usize).unwrap().remove(v) {
            self.m_degrees[v as usize] -= 1;
            if self.m_degrees[v as usize] == 0 {
                self.m_order -= 1;
            }
        }
    }

    fn remove_endpoint_unchecked(&mut self, u: u32, v: u32) {
        self.m_data.get_mut(u as usize).unwrap().remove(v);
        self.m_degrees[v as usize] -= 1;
        if self.m_degrees[v as usize] == 0 {
            self.m_order -= 1;
        }
    }

    /// Contracts the edge (target, source) by adding all neighbors
    /// of source to `target` and removing `source`
    pub fn contract_edge(&mut self, target: u32, source: u32) {
        self.check_bounds(target);
        self.check_bounds(source);
        self.check_is_same(target, source);
        let t_dat: &hibitset::BitSet = self.m_data.get(target as usize).unwrap();
        let s_dat: &hibitset::BitSet = self.m_data.get(source as usize).unwrap();
        if t_dat.contains(source) && s_dat.contains(target) {
            self.contract_edge_unchecked(target, source);
        } else {
            panic!(
                "Edge ({}, {}) does not exist. Can't contract!",
                target, source
            );
        }
    }

    /// Same as `contract_edge` without any boundary checks
    /// This is very unsafe, and can corrupt the entire graph if the function is called
    /// with invalid arguments
    pub fn contract_edge_unchecked(&mut self, target: u32, source: u32) {
        unsafe {
            let m_data_raw = self.m_data.as_mut_ptr();
            let source_dat = m_data_raw.add(source as usize);
            let target_dat = m_data_raw.add(target as usize);
            for w in (*source_dat).drain() {
                if w != target {
                    if !(*target_dat).add(w) {
                        self.m_degrees[target as usize] += 1;
                    }
                    let w_dat = m_data_raw.add(w as usize);
                    (*w_dat).remove(source);
                    if (*w_dat).add(target) {
                        self.m_degrees[w as usize] -= 1;
                    }
                }
            }
            (*target_dat).remove(source);
            self.m_degrees[target as usize] -= 1;
        }
        self.m_degrees[source as usize] = 0;
        self.m_order -= 1;
    }

    /// Returns an iterator over the neighborhood of vertex `v`
    pub fn neighbors(&self, v: u32) -> BitIter<&hibitset::BitSet> {
        self.m_data.get(v as usize).unwrap().iter()
    }

    /// Number of vertices in the graph
    pub fn order(&self) -> u32 {
        self.m_order
    }

    /// Number of neighbors of `v`
    pub fn degree(&self, v: u32) -> u32 {
        self.m_degrees[v as usize]
    }

    /// Returns a `DfsIterator` starting at vertex `v`
    pub fn dfs(&self, v: u32) -> DfsIterator {
        let mut stack: Vec<u32> = Vec::new();
        stack.push(v);
        DfsIterator {
            m_graph: self,
            m_visited: bit_set::BitSet::with_capacity(self.m_order as usize),
            m_stack: stack,
        }
    }
}

/// Iterator that performs a depths first search on a `BitGraph`
/// If the graph is fully-connected, all vertices are explored (spanning tree)
pub struct DfsIterator<'a> {
    m_visited: bit_set::BitSet,
    m_stack: Vec<u32>,
    m_graph: &'a BitGraph,
}

impl<'a> Iterator for DfsIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(v) = self.m_stack.pop() {
            let mut r = None;
            if !self.m_visited.contains(v as usize) {
                self.m_visited.insert(v as usize);
                r = Some(v);
            } else {
                r = None;
            };
            self.m_graph.neighbors(v).for_each(|u| {
                if !self.m_visited.contains(u as usize) {
                    self.m_stack.push(u);
                }
            });
            if r != None {
                return r;
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{BitGraph, DfsIterator};
    use hibitset::BitSetLike;

    #[test]
    fn with_capacity() {
        let capacity: usize = 100;
        let mut c = BitGraph::with_capacity(capacity as u32);

        assert_eq!(c.m_degrees.len(), capacity);
        assert_eq!(c.order(), 0);
    }

    #[test]
    fn complete() {
        let capacity: usize = 100;
        let mut c = BitGraph::complete(capacity as u32);

        assert_eq!(c.m_degrees.len(), capacity);
        assert_eq!(c.order(), capacity as u32);
        for i in 0..capacity {
            assert_eq!(c.m_degrees.len(), capacity);
        }
    }

    #[test]
    fn dfs() {
        let capacity: usize = 10;
        let c = BitGraph::complete(capacity as u32);

        let mut bit_set = bit_set::BitSet::with_capacity(capacity);
        for i in c.dfs(0) {
            assert_eq!(bit_set.insert(i as usize), true);
        }
        for i in 0..capacity {
            assert_eq!(bit_set.contains(i as usize), true);
        }
    }

    #[test]
    fn add_edge() {
        let capacity: usize = 10;
        let mut c = BitGraph::with_capacity(capacity as u32);
        let u = 2;
        let v = 6;
        c.add_edge(u, v);
        assert_eq!(c.m_degrees[u as usize], 1);
        assert_eq!(c.m_degrees[v as usize], 1);
        assert_eq!(c.order(), 2);
        assert_eq!(c.m_data.get(u as usize).unwrap().contains(v), true);
        assert_eq!(c.m_data.get(v as usize).unwrap().contains(u), true);
    }

    #[test]
    fn remove_edge() {
        let capacity: usize = 10;
        let mut c = BitGraph::with_capacity(capacity as u32);
        let u = 0;
        let v = 1;
        c.add_edge(u, v);

        c.remove_edge(u, v);
        assert_eq!(c.m_degrees[u as usize], 0);
        assert_eq!(c.m_degrees[v as usize], 0);
        assert_eq!(c.order(), 0);
        assert_eq!(c.m_data.get(u as usize).unwrap().contains(v), false);
        assert_eq!(c.m_data.get(v as usize).unwrap().contains(u), false);
    }

    #[test]
    fn contract_edge() {
        let capacity: usize = 10;
        let mut c = BitGraph::with_capacity(capacity as u32);

        let u = 0;
        let v = 3;

        for i in 0..3 {
            c.add_edge(u, i + 1);
        }
        for i in 3..5 {
            c.add_edge(v, i + 1);
        }
        assert_eq!(c.m_degrees[u as usize], 3);
        assert_eq!(c.m_degrees[v as usize], 3);
        assert_eq!(c.order(), 6);

        c.contract_edge(u as u32, v as u32);

        assert_eq!(c.m_degrees[v as usize], 0);
        assert_eq!(c.m_degrees[u as usize], 4);
        assert_eq!(c.order(), 5);

        let tmp: Vec<u32> = c.m_data.get(u as usize).unwrap().iter().collect();
        assert!(tmp.iter().zip([1,2,4,5].iter()).all(|(a,b)| a == b));
        let tmp: Vec<u32> = c.m_data.get(v as usize).unwrap().iter().collect();
        assert_eq!(tmp.len(), 0);
        for i in [1u32,2,4,5].iter() {
            let tmp: Vec<u32> = c.m_data.get(*i as usize).unwrap().iter().collect();
            assert_eq!(tmp.len(), 1);
            assert!(tmp.iter().zip([0u32].iter()).all(|(a,b)| a == b));
        }
    }
}
