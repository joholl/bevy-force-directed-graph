use std::path::PathBuf;

use clap::Parser;
use parse::FromDotGraph as _;

mod bevy;
mod parse;

#[derive(Parser, Debug)]
#[command(version, about = "A CLI tool to visualize code as a force-directed graph drawing", long_about = None)]
struct Args {
    /// Path to a graphviz dot file
    #[clap(
        default_value = "./target/debug/incremental/megascope-2cllsz378eeng/s-h5yn6enmam-1q5r4m2-7ay6373jgaavxjyti4wir7hwd/4qq6ek98yp9j1s8jfq85bsva8.ll.callgraph.dot"
    )]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let graph_ast = dot_parser::ast::Graph::from_file(&args.file).unwrap();
    let graph_canonical = dot_parser::canonical::Graph::from(graph_ast);
    let petgraph = petgraph::graph::Graph::<_, _>::from_dot_graph(graph_canonical);

    bevy::run(petgraph);
}
