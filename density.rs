use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn calculate_density(nodes: &HashSet<i32>, edges: &Vec<(i32, i32)>) -> f64 {
    let num_nodes = nodes.len() as f64;
    let num_edges = edges.len() as f64;
    let max_edges = num_nodes * (num_nodes - 1.0) / 2.0;
    num_edges / max_edges
}

fn compute_densest_subgraph(nodes: &HashSet<i32>, edges: &Vec<(i32, i32)>) -> HashSet<i32> {
    let mut densest_subgraph = HashSet::new();
    let mut densest_subgraph_density = 0.0;

    for &(src, dst) in edges {
        let mut subgraph = HashSet::new();
        subgraph.insert(src);
        subgraph.insert(dst);

        for &(other_src, other_dst) in edges {
            if (src == other_src && !subgraph.contains(&other_dst))
                || (src == other_dst && !subgraph.contains(&other_src))
                || (dst == other_src && !subgraph.contains(&other_dst))
                || (dst == other_dst && !subgraph.contains(&other_src))
            {
                subgraph.insert(other_src);
                subgraph.insert(other_dst);
            }
        }

        let subgraph_density = subgraph.len() as f64 / (subgraph.len() * (subgraph.len() - 1) / 2) as f64;

        if subgraph_density > densest_subgraph_density {
            densest_subgraph = subgraph.clone();
            densest_subgraph_density = subgraph_density;
        }
    }

    densest_subgraph
}

fn main() {
    // Read in graph from file
    let file = File::open("hollywood-2011-utf8.graph").unwrap();
    let reader = BufReader::new(file);

    let mut nodes = HashSet::new();
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
            nodes.insert(src);
            nodes.insert(dst);
        } else {
            continue;
        }
    }

    // Calculate graph density
    let num_nodes = nodes.len() as f64;
    let num_edges = edges.len() as f64;
    let max_edges = num_nodes * (num_nodes - 1.0) / 2.0;
    let density = num_edges / max_edges;
    println!("Graph density: {}", density);

    // Compute densest subgraph using 2-approximation algorithm
    let mut densest_subgraph = HashSet::new();
    let mut densest_subgraph_density = 0.0;

    for &(src, dst) in &edges {
        let mut subgraph = HashSet::new();
        subgraph.insert(src);
        subgraph.insert(dst);

        for &(other_src, other_dst) in &edges {
            if (src == other_src && !subgraph.contains(&other_dst))
                || (src == other_dst && !subgraph.contains(&other_src))
                || (dst == other_src && !subgraph.contains(&other_dst))
                || (dst == other_dst && !subgraph.contains(&other_src))
            {
                subgraph.insert(other_src);
                subgraph.insert(other_dst);
            }
        }

        let subgraph_density = subgraph.len() as f64 / (subgraph.len() * (subgraph.len() - 1) / 2) as f64;

        if subgraph_density > densest_subgraph_density {
            densest_subgraph = subgraph.clone();
            densest_subgraph_density = subgraph_density;
        }
    }

    println!("Densest subgraph: {:?}", densest_subgraph);
    println!("Densest subgraph density: {}", densest_subgraph_density);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density() {
        // Test graph with 3 nodes and 2 edges
        let nodes = [1, 2, 3].iter().cloned().collect();
        let edges = [(1, 2), (2, 3)].iter().cloned().collect();
        let density = 2.0 / 3.0;
        assert_eq!(calculate_density(&nodes, &edges), density);
    }

    #[test]
    fn test_densest_subgraph() {
        // Test graph with 5 nodes and 5 edges
        let nodes = [1, 2, 3, 4, 5].iter().cloned().collect();
        let edges = [(1, 2), (1, 3), (1, 4), (2, 3), (3, 4)].iter().cloned().collect();
        let densest_subgraph = [1, 2, 3].iter().cloned().collect();
        assert_eq!(compute_densest_subgraph(&nodes, &edges), densest_subgraph);
    }
}

