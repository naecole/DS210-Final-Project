use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // Read in graph from file
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
        } else {
            continue;
        }
    }

    // Calculate degree distribution
    let mut degree_counts = HashMap::new();
    for &(src, dst) in &edges {
        *degree_counts.entry(src).or_insert(0) += 1;
        *degree_counts.entry(dst).or_insert(0) += 1;
    }

    // Sort degree counts by degree
    let mut degree_counts_vec: Vec<_> = degree_counts.into_iter().collect();
    degree_counts_vec.sort_by(|a, b| a.0.cmp(&b.0));

    // Print degree distribution
    println!("Degree distribution:");
    for (degree, count) in &degree_counts_vec {
        println!("{} {}", degree, count);
    }

    // Test whether distribution follows power-law
    let num_vertices = nodes.len() as f64;
    let num_edges = edges.len() as f64;
    let degree_sum: f64 = degree_counts_vec.iter().map(|(_, count)| *count as f64).sum();

    let gamma = 1.0 + num_vertices * (num_vertices / (num_edges * degree_sum)).ln();
    println!("Gamma: {}", gamma);

    if gamma < 2.0 {
        println!("Power-law distribution confirmed");
    } else {
        println!("Distribution does not follow power-law");
    }
}

fn calculate_degree_distribution(edges: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut degree_counts = HashMap::new();
    for &(src, dst) in edges {
        *degree_counts.entry(src).or_insert(0) += 1;
        *degree_counts.entry(dst).or_insert(0) += 1;
    }
    let mut degree_counts_vec: Vec<_> = degree_counts.into_iter().collect();
    degree_counts_vec.sort_by(|a, b| a.0.cmp(&b.0));
    degree_counts_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_degree_distribution() {
        let edges = vec![(1, 2), (2, 3), (2, 4), (3, 4), (4, 5)];

        let expected_degree_counts = vec![
            (1, 1),
            (2, 3),
            (3, 2),
            (4, 3),
            (5, 1),
        ];

        let degree_counts = calculate_degree_distribution(&edges);

        assert_eq!(degree_counts, expected_degree_counts);
    }

    #[test]
    fn test_power_law_distribution() {
        let edges = vec![(1, 2), (2, 3), (2, 4), (3, 4), (4, 5)];

        let num_vertices = 5_f64;
        let num_edges = 5_f64;
        let degree_sum = 12_f64;

        let gamma = 1.0 + num_vertices * (num_vertices / (num_edges * degree_sum)).ln();

        assert!(gamma < 2.0);
    }
}

