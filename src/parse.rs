use std::collections::HashMap;

pub trait FromDotGraph<T> {
    fn from_dot_graph(val: T) -> Self;
}

impl<A> FromDotGraph<dot_parser::canonical::Graph<A>>
    for petgraph::graph::Graph<dot_parser::canonical::Node<A>, dot_parser::ast::AList<A>>
{
    fn from_dot_graph(value: dot_parser::canonical::Graph<A>) -> Self {
        let mut graph = petgraph::graph::Graph::new();
        let mut node_indices = HashMap::new();
        for node in value.nodes.set {
            let node_index = graph.add_node(node.1);
            node_indices.insert(node.0, node_index);
        }
        for edge in value.edges.set {
            let from_node_index = node_indices.get(&edge.from).unwrap();
            let to_node_index = node_indices.get(&edge.to).unwrap();
            graph.add_edge(*from_node_index, *to_node_index, edge.attr);
        }
        graph
    }
}
