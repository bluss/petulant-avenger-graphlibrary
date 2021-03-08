use std::collections::HashSet;
use std::hash::Hash;

use petgraph::algo::{greedy_matching, maximum_matching};
use petgraph::prelude::*;

macro_rules! assert_one_of {
    ($actual:expr, [$($expected:expr),+]) => {
        let expected = &[$($expected),+];
        if !expected.iter().any(|expected| expected == &$actual) {
            let expected = expected.iter().map(|e| format!("\n{:?}", e)).collect::<Vec<_>>();
            let comma_separated = expected.join(", ");
            panic!("assertion failed: `actual does not equal to any of expected`\nactual:\n{:?}\nexpected:{}", $actual, comma_separated);
        }
    };
}

macro_rules! set {
    () => {
        HashSet::new()
    };
    ($($elem:expr),+) => {
        {
            let mut set = HashSet::new();
            $(
                set.insert($elem.into());
            )*
            set
        }
    };
}

// So we don't have to type `.collect::<HashSet<_>>`.
fn collect<'a, T: Copy + Eq + Hash + 'a>(iter: impl Iterator<Item = T>) -> HashSet<T> {
    iter.collect()
}

#[test]
fn greedy_empty() {
    let g: UnGraph<(), ()> = UnGraph::default();
    let m = greedy_matching(&g);
    assert_eq!(collect(m.edges()), set![]);
    assert_eq!(collect(m.nodes()), set![]);
}

#[test]
fn greedy_disjoint() {
    let g: UnGraph<(), ()> = UnGraph::from_edges(&[(0, 1), (2, 3)]);
    let m = greedy_matching(&g);
    assert_eq!(collect(m.edges()), set![0, 1]);
    assert_eq!(collect(m.nodes()), set![0, 1, 2, 3]);
}

#[test]
fn greedy_odd_path() {
    let g: UnGraph<(), ()> = UnGraph::from_edges(&[(0, 1), (1, 2), (2, 3)]);
    let m = greedy_matching(&g);
    assert_one_of!(collect(m.edges()), [set![0, 2], set![1]]);
    assert_one_of!(collect(m.nodes()), [set![0, 1, 2, 3], set![1, 2]]);
}

#[test]
fn greedy_star() {
    let g: UnGraph<(), ()> = UnGraph::from_edges(&[(0, 1), (0, 2), (0, 3)]);
    let m = greedy_matching(&g);
    assert_one_of!(collect(m.edges()), [set![0], set![1], set![2]]);
    assert_one_of!(collect(m.nodes()), [set![0, 1], set![0, 2], set![0, 3]]);
}

#[test]
fn maximum_empty() {
    let g: UnGraph<(), ()> = UnGraph::default();
    let m = maximum_matching(&g);
    assert_eq!(collect(m.edges()), set![]);
    assert_eq!(collect(m.nodes()), set![]);
}

#[test]
fn maximum_disjoint() {
    let g: UnGraph<(), ()> = UnGraph::from_edges(&[(0, 1), (2, 3)]);
    let m = maximum_matching(&g);
    assert_eq!(collect(m.edges()), set![0, 1]);
    assert_eq!(collect(m.nodes()), set![0, 1, 2, 3]);
}

#[test]
fn maximum_odd_path() {
    let g: UnGraph<(), ()> = UnGraph::from_edges(&[(0, 1), (1, 2), (2, 3)]);
    let m = maximum_matching(&g);
    assert_eq!(collect(m.edges()), set![0, 2]);
    assert_eq!(collect(m.nodes()), set![0, 1, 2, 3]);
}
