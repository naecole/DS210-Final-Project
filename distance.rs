use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    // Calculate average distance between pairs of vertices
    let mut dist_sum = 0;
    let mut num_pairs = 0;
    let mut distances = HashMap::new();

    for &start_node in &nodes {
        let mut visited = HashSet::new();
        let mut queue = Vec::new();
        let mut depth = 0;
        visited.insert(start_node);
        queue.push(start_node);

        while !queue.is_empty() {
            let mut next_queue = Vec::new();
            for &node in &queue {
                for &(src, dst) in &edges {
                    if src == node && !visited.contains(&dst) {
                        visited.insert(dst);
                        next_queue.push(dst);
                        distances.insert((start_node, dst), depth + 1);
                    } else if dst == node && !visited.contains(&src) {
                        visited.insert(src);
                        next_queue.push(src);
                        distances.insert((start_node, src), depth + 1);
                    }
                }
            }
            queue = next_queue;
            depth += 1;
        }
    }

    for ((src, dst), dist) in &distances {
        if src < dst {
            dist_sum += dist;
            num_pairs += 1;
        }
    }

    let avg_distance = dist_sum as f64 / num_pairs as f64;

    println!("Average distance between pairs of vertices: {}", avg_distance);
}
