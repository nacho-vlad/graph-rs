use rustyline::error::ReadlineError;
use rustyline::Editor;
use graph::*;
use rand::{ Rng, seq::IteratorRandom };
use std::path::Path;


fn random_graph(vertices: u32, edges: u32) -> GraphMap<u32,u32> {

    let mut graph = GraphMap::<u32, u32>::new();
    let mut rng = rand::thread_rng();
    
    if vertices*vertices > edges {
        println!("Impossible");
        return graph;
    }

    let mut edge_list = Vec::new();
    for i in 0..vertices {
        for j in 0..vertices {
            edge_list.push((i,j));
        }
    }

    for &(from, to) in edge_list.iter().choose_multiple(&mut rng, edges as usize) {
        let weight: u32= rng.gen_range(0..100);
        graph.add_edge((from,to), weight);
    }
    graph
}

fn read_graph(path: &Path, undirected: bool) -> GraphMap<u32, u32> {
    let mut graph = GraphMap::<u32,u32>::new();
    let contents = std::fs::read_to_string(path).unwrap();
    
    let lines: Vec<&str> = contents.lines().collect();
    let first_line: Vec<&str> = lines[0].split(" ").collect();
    let nodes = str::parse::<u32>(first_line[0]).unwrap();
    // let edges = str::parse::<u32>(first_line[1]).unwrap();

    for v in 0..nodes {
        graph.add_vertex(v);
    }

    for line in lines[1..].iter() {
        let line: Vec<&str> = line.split(" ").collect();
        let origin = str::parse::<u32>(line[0]).unwrap();
        let target = str::parse::<u32>(line[1]).unwrap();
        let cost = str::parse::<u32>(line[2]).unwrap();
        graph.add_edge((origin,target), cost);
        if undirected {
            graph.add_edge((target,origin), cost);
        }
    }
    graph
}

fn write_graph<W: std::io::Write>(writer: &mut W, graph: &GraphMap<u32,u32> ) {
    write!(writer, "{} {}\n", graph.vertex_count(), graph.edge_count()).unwrap();
    
    for ((&origin, &target), &cost) in graph.edges() {
        write!(writer, "{} {} {}\n", origin,target,cost).unwrap();
    }

    for &v in graph.vertices() {
        if graph.outdegree(v) == 0 && graph.indegree(v) == 0 {
            write!(writer, "{}\n", v).unwrap();
        }
    }
}


fn show_help() {
    println!("add_edge <origin> <dest> <cost>");
    println!("remove_edge <origin> <dest>");
    println!("get_edge <origin> <dest>");
    println!("remove_node <vertex>");
    println!("add_vertex <vertex>");
    println!("indegree <vertex>");
    println!("outdegree <vertex>");
    println!("inbound <vertex>");
    println!("outbound <vertex>");
    println!("vertex_count");
    println!("edge_count");
    println!("print_graph");
    println!("contains_edge");
    println!("connected_components");
}

fn main() {
    let in_file: &str = "components.txt";   
    let out_file: &str = "graph1k_modif.txt";
    let mut graph = read_graph(Path::new(in_file), true);

    let mut rl = Editor::<()>::new();

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                
                let line_split: Vec<&str> = line.split(" ").collect();
                match line_split[0] {
                    "help" => show_help(),
                    "add_vertex" => {
                        let first = str::parse::<u32>(line_split[1]).unwrap();
                        graph.add_vertex(first);
                    }
                    "add_edge" => {
                        let first = str::parse::<u32>(line_split[1]).unwrap();
                        let second = str::parse::<u32>(line_split[2]).unwrap();
                        let third = str::parse::<u32>(line_split[3]).unwrap();
                        graph.add_edge((first,second),third);                     
                    },
                    "get_edge" => {
                        let first = str::parse::<u32>(line_split[1]).unwrap();
                        let second = str::parse::<u32>(line_split[2]).unwrap();
                        println!("{:?}", graph.get_edge((first,second)));
                    }
                    "remove_edge" => {
                        let first = str::parse::<u32>(line_split[1]).unwrap();
                        let second = str::parse::<u32>(line_split[2]).unwrap();
                        graph.remove_edge((first,second));
                    },
                    "remove_node" => {
                        let first = str::parse::<u32>(line_split[1]).unwrap();
                        graph.remove_vertex(first);
                    },
                    "indegree" => {
                        let first = str::parse::<u32>(line_split[1]).unwrap();
                        println!("{}", graph.indegree(first));
                    },
                    "outdegree" => {
                        let first = str::parse::<u32>(line_split[1]).unwrap();
                        println!("{}", graph.outdegree(first));
                    },
                    "outbound" => {
                        let first = str::parse::<u32>(line_split[1]).unwrap();
                        for (v, w) in graph.adj_out(first).unwrap() {
                            println!("{} {}", v, w);
                        }
                    },
                    "inbound" => {
                        let first = str::parse::<u32>(line_split[1]).unwrap();
                        for (v, w) in graph.adj_in(first).unwrap() {
                            println!("{} {}", v, w);
                        }
                    },
                    "vertex_count" => {
                        println!("{}", graph.vertex_count());
                    },
                    "edge_count" => {
                        println!("{}", graph.edge_count());
                    },                
                    "print_graph" => {
                        write_graph(&mut std::io::stdout(), &graph);
                    },
                    "contains_edge" => {
                        let first = str::parse::<u32>(line_split[1]).unwrap();
                        let second = str::parse::<u32>(line_split[2]).unwrap();
                        println!("{}", graph.contains_edge((first,second)));
                    }
                    "connected_components" => {
                        let components = graph.connected_components();
                        for g in components.iter() {

                            println!("Component: ");
                            for v in g.vertices() {
                                print!("{} ", v);
                            }
                            println!("");
                        }
                    }
                    _ => {
                        println!("No such command");
                        continue;
                    }
                }
                
                rl.add_history_entry(line.as_str());
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();

    let mut file = std::fs::File::create(Path::new(out_file)).unwrap();
    write_graph(&mut file, &graph);

}
