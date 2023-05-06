use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use petgraph::graphmap::UnGraphMap;

fn main() {
    // Read in graph from file
    let file = File::open("hollywood-2011-utf8.graph").unwrap();
    let reader = BufReader::new(file);

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
        }
    }

    // Calculate degree centrality
    let mut degree_counts = HashMap::new();
    for &(src, dst) in &edges {
        *degree_counts.entry(src).or_insert(0) += 1;
        *degree_counts.entry(dst).or_insert(0) += 1;
    }

    // Sort degree counts by degree
    let mut degree_counts_vec: Vec<_> = degree_counts.into_iter().collect();
    degree_counts_vec.sort_by(|a, b| a.0.cmp(&b.0));

    // Print degree centrality
    println!("Degree centrality:");
    for (node, degree) in degree_counts_vec.iter().rev() {
        println!("Node {}: {}", node, degree);
    }
}
