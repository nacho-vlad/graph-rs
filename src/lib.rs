#![crate_name = "graph"]
//! Fast and efficient graph data structure library.
//!
//! [`Graph`] is implemented with 3 [`HashMap`]s, 1 for
//! the outbound edges, 1 for the inbound edges and 1 for the weight of the edges.
//! The data in the vertices is kept in an arena.
//!
//! I also implemented [`GraphMap`], which identifies the nodes by
//! the data they hold, instead of [`VertexId`].
pub mod traversal;

use generational_arena::{ Arena, Index };
use std::vec::IntoIter;
use std::hash::Hash;
use std::collections::{
    HashMap, 
    hash_set::HashSet,
    hash_map,
};

pub type VertexId = Index;
pub type EdgeId = (VertexId, VertexId);

/// Graph data structure. [`V`] is the Vertex data,
/// and [`E`] is the Edge data.
#[derive(Clone, Debug)]
pub struct Graph<V, E> {
    arena: Arena<V>,
    inbound: HashMap<VertexId, HashSet<VertexId>>,
    outbound: HashMap<VertexId, HashSet<VertexId>>,
    edges: HashMap<EdgeId, E>,
}

impl<V: std::fmt::Debug,E> Graph<V, E> {

    pub fn new() -> Self {
        let arena = Arena::new();
        let inbound = HashMap::new();
        let outbound = HashMap::new();
        let edges = HashMap::new();
        Graph {
            arena,
            inbound,
            outbound,
            edges,
        }
    }
    
    /// Adds a vertes to the graph, and returns an Id.
    /// Only way to get Id.
    pub fn add_vertex(&mut self, vertex: V) -> VertexId {
        let id = self.arena.insert(vertex);
        self.inbound.entry(id).or_default();
        self.outbound.entry(id).or_default();
        id
    }
    
    /// Returns the data in the vertex.
    pub fn get_vertex(&self, vertex: VertexId) -> Option<&V> {
        self.arena.get(vertex)
    }
    
    /// Adds an edge, or modifies the existing one.
    pub fn add_edge(&mut self, edge: EdgeId, weight: E) {
        self.edges.insert(edge, weight);
        let (from, to) = edge;
        self.outbound.entry(from).or_default().insert(to);
        self.inbound.entry(to).or_default().insert(from);
    }
    
    /// Get the edge.
    pub fn get_edge(&self, edge: EdgeId) -> Option<&E> {
        self.edges.get(&edge) 
    }
    
    /// Removes the vertes.
    /// Time complexity: O(outdegree(v))
    pub fn remove_vertex(&mut self, vertex: VertexId) {
        self.arena.remove(vertex);
        let from = vertex;

        for &to in self.outbound[&from].iter() {
            self.edges.remove(&(from,to));
            self.inbound.get_mut(&to).unwrap().remove(&from);
        }

        let to = from;
        for &from in self.inbound[&to].iter() {
            self.edges.remove(&(from,to));
        }
        
        self.inbound.remove(&from);
        self.outbound.remove(&from);
    }
    
    /// Remove an edge
    /// Time complexity: O(1)
    pub fn remove_edge(&mut self, edge: EdgeId) {
        self.edges.remove(&edge);              
        let (from, to) = edge;
        self.outbound.get_mut(&from).unwrap().remove(&to);
        self.inbound.get_mut(&to).unwrap().remove(&from);
    }
    
    /// Returns an iterator over outbound edges
    pub fn adj_out(&self, vertex: VertexId) -> Option<IntoIter<(VertexId, &E)>> {
        let outbound = self.outbound.get(&vertex)?;
        let vec: Vec<(VertexId, &E)> = outbound.iter().map(|&target| {
            (target, self.edges.get(&(vertex,target)).unwrap())
        }).collect();
        Some(vec.into_iter())
    }

    /// Returns an iterator over inbound edges
    pub fn adj_in(&self, vertex: VertexId) -> Option<IntoIter<(VertexId, &E)>> {
        let inbound = self.inbound.get(&vertex)?;
        let vec: Vec<(VertexId, &E)> = inbound.iter().map(|&target| {
            (target, self.edges.get(&(target,vertex)).unwrap())
        }).collect();
        Some(vec.into_iter())
    }
    
    /// Indegree of the vertex
    pub fn indegree(&self, vertex: VertexId) -> usize {
        match self.inbound.get(&vertex) {
            Some(set) => set.len(),
            None => 0,
        }
    }
    
    /// Outdegree of the vertex
    pub fn outdegree(&self, vertex: VertexId) -> usize {
        match self.outbound.get(&vertex) {
            Some(set) => set.len(),
            None => 0,
        }
    }

    /// Number of vertices
    pub fn vertex_count(&self) -> usize {
        self.arena.len()
    }
    
    /// Number of edges
    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }
    
    /// Iterator over the vertices
    pub fn vertices(&self) -> generational_arena::Iter<V> {
        self.arena.iter()
    }
    
    /// Iterator over the edges
    pub fn edges(&self) -> hash_map::Iter<EdgeId, E> {
        self.edges.iter()
    }

}


/// Wrapper around the [`Graph`] that allows you
/// to identify the vertices by their data.
/// [`V`] needs to be [`Hash`].
#[derive(Clone, Debug)]
pub struct GraphMap<V: Eq + Hash + Clone, E> {
    graph: Graph<V, E>,
    map: HashMap<V, VertexId>,
}

impl<V: Eq + Hash + Clone + std::fmt::Debug, E> GraphMap<V,E> {
    pub fn new() -> Self {
        let graph = Graph::new();
        let map = HashMap::new();
        GraphMap {
            graph,
            map
        }
    }

    fn add_or_get_vertex(&mut self, vertex: V) -> VertexId {

        match self.map.get(&vertex) {
            Some(&id) => id,
            None => {
                let id = self.graph.add_vertex(vertex.clone());
                self.map.insert(vertex, id);
                id
            }
        }
        
    }

    /// Adds a vertes to the graph. If already inside
    /// do nothing
    pub fn add_vertex(&mut self, vertex: V) {
        match self.map.get(&vertex) {
            Some(_) => return (),
            None => {
                let id = self.graph.add_vertex(vertex.clone());
                self.map.insert(vertex, id);
            }
        }

    }

    /// Adds an edge, or modifies the existing one.
    pub fn add_edge(&mut self, edge: (V, V), weight: E) {
        let (from, to) = edge;

        let from = self.add_or_get_vertex(from);
        let to = self.add_or_get_vertex(to);
        self.graph.add_edge((from,to), weight);
    }
    
    ///Get an edge
    pub fn get_edge(&self, edge: (V, V)) -> Option<&E> {
        let (from, to) = edge;
        let from = *self.map.get(&from)?;
        let to = *self.map.get(&to)?;
        self.graph.get_edge((from,to)) 
    }

    pub fn contains_edge(&self, edge: (V,V)) -> bool {
        !self.get_edge(edge).is_none()
    }

    /// Removes the vertes.
    /// Time complexity: O(outdegree(v))
    pub fn remove_vertex(&mut self, vertex: V) -> bool {
        let id = match self.map.remove(&vertex) {
            Some(id) => id,
            None => return false,
        };
        self.graph.remove_vertex(id);
        true
    }
    
    /// Removes an edge.
    pub fn remove_edge(&mut self, edge: (V,V) ) -> bool {
        let (from, to) = edge;
        let from = *match self.map.get(&from) {
            Some(id) => id,
            None => return false,
        };
        let to = *match self.map.get(&to) {
            Some(id) => id,
            None => return false,
        };
        self.graph.remove_edge((from,to));
        true
    }
    
    /// Iterate over the outbound nodes.
    /// Returns pairs of (vertex, weight).
    pub fn adj_out(&self, vertex: V) -> Option<IntoIter<(&V, &E)>> {
        let id = *self.map.get(&vertex)?;
        let vec: Vec<(&V,&E)> = self.graph.adj_out(id)?.map(|(id, e)| {
            (self.graph.get_vertex(id).unwrap(), e)
        }).collect();
        Some(vec.into_iter())
    }

    /// Iterate over the inbound nodes.
    /// Returns pairs of (vertex, weight).
    pub fn adj_in(&self, vertex: V) -> Option<IntoIter<(&V, &E)>> {
        let id = *self.map.get(&vertex)?;
        let vec: Vec<(&V,&E)> = self.graph.adj_in(id)?.map(|(id, e)| {
            (self.graph.get_vertex(id).unwrap(), e)
        }).collect();
        Some(vec.into_iter())
    }
    
    /// Indegree of the node
    pub fn indegree(&self, vertex: V) -> usize {
        self.graph.indegree(self.map[&vertex])
    }

    /// Outdegree of the node
    pub fn outdegree(&self, vertex: V) -> usize {
        self.graph.outdegree(self.map[&vertex])
    }

    pub fn vertex_count(&self) -> usize {
        self.graph.vertex_count()
    }

    pub fn edge_count(&self) -> usize {
        self.graph.edge_count()
    }

    pub fn vertices(&self) -> hash_map::Keys<V, VertexId> {
        self.map.keys()
    }

    pub fn edges(&self) -> IntoIter<((&V,&V), &E)> {
        let edges = self.graph.edges();
        let vec: Vec<((&V,&V), &E)> = edges.map(|((from,to), e)| {
            // println!("{:?} {:?}", self.graph.get_vertex(*from), self.graph.get_vertex(*to));
            ((self.graph.get_vertex(*from).unwrap(), self.graph.get_vertex(*to).unwrap()), e)
        }).collect();
        vec.into_iter()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
