
use super::{GraphMap};
use std::hash::Hash;
use std::cmp::Reverse;
use std::collections::{
    VecDeque,
    BinaryHeap,
    HashSet,
    HashMap
};

impl<V: Eq + Hash + Clone + std::fmt::Debug + Ord, E: Clone + Ord + std::ops::Add> GraphMap<V,E> {
    
    /// Performs a BFS starting on the given node.
    /// Returns a vector ov all nodes, in the order
    /// they were traversed
    pub fn bfs(&self, start: &V) -> Vec<V> {
        let mut visited = HashSet::new();
        let mut nodes = Vec::new();
        let mut queue = VecDeque::new();
        
        queue.push_back(start.clone());
        visited.insert(start.clone());
        
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();

            nodes.push(current.clone());

            for (v, _) in self.adj_out(current).unwrap() {
                if !visited.contains(v) {
                    visited.insert(v.clone());
                    queue.push_back(v.clone());
                }
            }
        }
        nodes
    }

    /// Finds all connected components.
    /// Returns a vector of Graphs, each representing a different
    /// connected component.
    pub fn connected_components(&self) -> Vec<GraphMap<V,E>> {
        let mut components = Vec::new();
        let mut visited = HashSet::<V>::new();

        for v in self.vertices() {
            if !visited.contains(v) {
                let component = self.bfs(v);
                

                //make the graph
                let mut graph = GraphMap::<V,E>::new();

                for node in component.iter() {
                    //add it to the visited nodes
                    visited.insert(node.clone());
                    
                    graph.add_vertex(node.clone());

                    for (adj, w) in self.adj_out(node.clone()).unwrap() {
                        graph.add_edge((node.clone(),adj.clone()), (*w).clone());
                    }
                }
                components.push(graph);
            }
        }

        components
    }

}


impl GraphMap<u32, u32> {

    pub fn dijkstra(&self, start: u32, end: u32) -> Option<(Vec<u32>, u32)> {
        let mut queue = BinaryHeap::new();
        let mut dist = HashMap::<u32, u32>::new();
        let mut next = HashMap::<u32, u32>::new();

        dist.insert(end, 0);
        queue.push(Reverse((0, end)));

        while !queue.is_empty() {
            let Reverse((_, node)) = queue.pop().unwrap();       


            for (&prev, &cost) in self.adj_in(node).unwrap() {
                if !dist.contains_key(&prev) || dist[&node] + cost < dist[&prev] {
                    dist.insert(prev, dist[&node] + cost);
                    queue.push(Reverse((dist[&prev], prev)));
                    next.insert(prev,node);
                }
            }
        }

        if !dist.contains_key(&start) {
            return None;
        }

        let mut path = Vec::new();
        let mut curr = start;
        
        while curr != end {
            path.push(curr);    
            curr = next[&curr];
        }
        
        path.push(end);

        Some((path, dist[&start]))
    }

}
