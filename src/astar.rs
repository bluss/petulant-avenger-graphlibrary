use std::collections::{
    HashMap,
    BinaryHeap,
};
use std::collections::hash_map::Entry::{
    Occupied,
    Vacant,
};

use std::hash::Hash;

use scored::MinScored;
use super::visit::{
    EdgeRef,
    GraphBase,
    IntoEdges,
    VisitMap,
    Visitable,
};

use algo::Measure;

/// [Generic] A* shortest path algorithm.
///
/// Compute the length of the shortest path from `start` to `finish`.
///
/// `finish` is implicitly given via the `is_goal` callback, which should return `true` if the
/// given node is the finish node.
///
/// The function `edge_cost` should return the cost for a particular edge, which is used to compute
/// path costs.
///
/// The function `estimate_cost` should return the estimated cost to the finish for a particular
/// node, also used to compute path costs.
///
/// ```
/// use petgraph::Graph;
/// use petgraph::algo::astar;
///
/// let mut g = Graph::new();
/// let a = g.add_node((0., 0.));
/// let b = g.add_node((2., 0.));
/// let c = g.add_node((1., 1.));
/// let d = g.add_node((0., 2.));
/// let e = g.add_node((3., 3.));
/// let f = g.add_node((4., 2.));
/// let ab = g.add_edge(a, b, 2);
/// let ad = g.add_edge(a, d, 4);
/// let bc = g.add_edge(b, c, 1);
/// let bf = g.add_edge(b, f, 7);
/// let ce = g.add_edge(c, e, 5);
/// let ef = g.add_edge(e, f, 1);
/// let de = g.add_edge(d, e, 1);
///
/// let path = astar(&g, a, |finish| finish == f, |e| *e.weight(), |_| 0);
/// assert_eq!(path, Some(vec![a, d, e, f]));
/// ```
///
/// The graph should be `Visitable` and implement `IntoEdges`. Edge costs must be non-negative.
///
/// Returns a Path of subsequent `NodeId` from start to finish, if one was found.
pub fn astar<G, F, H, K, IsGoal>(graph: G, start: G::NodeId, is_goal: IsGoal,
                                     mut edge_cost: F, mut estimate_cost: H)
    -> Option<Path<G>>
    where G: IntoEdges + Visitable,
          IsGoal: Fn(G::NodeId) -> bool,
          G::NodeId: Eq + Hash,
          F: FnMut(G::EdgeRef) -> K,
          H: FnMut(G::NodeId) -> K,
          K: Measure + Copy,
{
    let mut visited = graph.visit_map();
    let mut visit_next = BinaryHeap::new();
    let mut scores = HashMap::new();
    let mut path_tracker = PathTracker::<G>::new();

    let zero_score = K::default();
    scores.insert(start, zero_score);
    visit_next.push(MinScored(estimate_cost(start), start));

    while let Some(MinScored(_, node)) = visit_next.pop() {
        if is_goal(node) {
            return Some(path_tracker.reconstruct_path_to(node));
        }

        // Don't visit the same node several times, as the first time it was visited it was using
        // the shortest available path.
        if !visited.visit(node) {
            continue
        }

        // This lookup can be unwrapped without fear of panic since the node was necessarily scored
        // before adding him to `visit_next`.
        let node_score = *scores.get(&node).unwrap();

        for edge in graph.edges(node) {
            let next = edge.target();
            if visited.is_visited(&next) {
                continue
            }

            let mut next_score = node_score + edge_cost(edge);

            match scores.entry(next) {
                Occupied(ent) => {
                    let old_score = *ent.get();
                    if next_score < old_score {
                        *ent.into_mut() = next_score;
                        path_tracker.set_predecessor(next, node);
                    } else {
                        next_score = old_score;
                    }
                },
                Vacant(ent) => {
                    ent.insert(next_score);
                    path_tracker.set_predecessor(next, node);
                }
            }

            let next_estimate_score = next_score + estimate_cost(next);
            visit_next.push(MinScored(next_estimate_score, next));
        }
    }

    None
}

pub type Path<G: GraphBase> = Vec<G::NodeId>;

struct PathTracker<G>
    where G: GraphBase,
          G::NodeId: Eq + Hash,
{
    came_from: HashMap<G::NodeId, G::NodeId>,
}

impl<G> PathTracker<G>
    where G: GraphBase,
          G::NodeId: Eq + Hash,
{
    fn new() -> PathTracker<G> {
        PathTracker {
            came_from: HashMap::new(),
        }
    }

    fn set_predecessor(&mut self, node: G::NodeId, previous: G::NodeId) {
        self.came_from.insert(node, previous);
    }

    fn reconstruct_path_to(&self, last: G::NodeId) -> Path<G> {
        let mut path = vec![last];

        let mut current = last;
        loop {
            if let Some(&previous) = self.came_from.get(&current) {
                path.push(previous);
                current = previous;
            } else {
                break
            }
        }

        path.reverse();

        path
    }
}