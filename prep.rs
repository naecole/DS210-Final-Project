use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let file = File::open("hollywood-2011-utf8.graph").unwrap();
    let reader = BufReader::new(file);

    let mut nodes = Vec::new();
    let mut edges = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        // Ignore comment lines
        if line.starts_with('#') {
            continue;
        }

        let numbers: Vec<_> = line
            .split_whitespace()
            .map(|s| i32::from_str_radix(s, 10))
            .filter_map(Result::ok)
            .collect();

        if numbers.len() == 2 {
            let (src, dst) = (numbers[0], numbers[1]);
            edges.push((src, dst));
        } else if numbers.len() == 1 {
            let node_id = numbers[0];
            nodes.push(node_id);
        }
    }

    println!("Nodes: {:?}", nodes);
    println!("Edges: {:?}", edges);

 // Write nodes and edges to file
    let mut file = File::create("nodes_and_edges.txt").unwrap();

    writeln!(file, "Nodes: {:?}", nodes).unwrap();
    writeln!(file, "Edges: {:?}", edges).unwrap();
}