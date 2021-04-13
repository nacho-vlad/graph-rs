
use super::{GraphMap};
use std::hash::Hash;
use std::collections::{
    VecDeque,
    HashSet
};

impl<V: Eq + Hash + Clone + std::fmt::Debug, E: Clone> GraphMap<V,E> {
    
    pub fn bfs(&self, start: &V) -> Vec<V> {
        let mut visited = HashSet::new();
        let mut nodes = Vec::new();
        let mut queue = VecDeque::new();
        
        queue.push_back(start.clone());

        
        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();

            nodes.push(current.clone());
            visited.insert(current.clone());

            for (v, _) in self.adj_out(current).unwrap() {
                if !visited.contains(v) {
                    queue.push_back(v.clone());
                }
            }
        }
        nodes
    }

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
