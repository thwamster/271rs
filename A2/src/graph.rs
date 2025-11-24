use std::collections::*;

pub struct Graph<V, E> {
	directional : u8,
	list_vertices : LinkedList<GraphVertex<V, E>>,
	list_edges : LinkedList<GraphEdge<V, E>>
}

pub struct GraphVertex<V, E> {
	value : V,
	directional : u8,
	inbound : HashMap<GraphVertex<V, E>, GraphEdge<V, E>>,
	outbound : HashMap<GraphVertex<V, E>, GraphEdge<V, E>>
}

pub struct GraphEdge<V, E> {
	value : E,
	connections : (GraphVertex<V, E>, GraphVertex<V, E>)
}
