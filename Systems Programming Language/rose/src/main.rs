// Starter file provided to CSC 330, Summer 2025, Assignment 3
// Copyright Mike Zastre, UVic 2025.
//
// This echoes the functionality provided by the starter file in
// Haskell for the similar problem in Assignment 1.
//
// Therefore your task is to complete the functionality needed
// by `best_route()` -- and which will (perhaps) including writing
// other Rust functions in turn.
//

use std::fs::read_to_string;
use std::env;

#[allow(dead_code)]
#[derive(Debug)]
struct RouteStats {
    length : i32,
    max_items : i32,
}


// Helper function to build adjacency list representation of the graph
fn build_graph(num_locations: i32, edges: &Vec<(i32, i32, i32)>) -> Vec<Vec<(i32, i32)>> {
    let mut graph = vec![Vec::new(); num_locations as usize];
    
    for &(from, to, weight) in edges {
        // Convert to 0-indexed and add bidirectional edges
        let from_idx = (from - 1) as usize;
        let to_idx = (to - 1) as usize;
        
        graph[from_idx].push((to_idx as i32, weight));
        graph[to_idx].push((from_idx as i32, weight));
    }
    
    graph
}



// Find the shortest path from start to end, and among all shortest paths,
// choose the one that collects the most items
fn find_optimal_path(start: usize, end: usize, graph: &Vec<Vec<(i32, i32)>>, 
                    num_items: &Vec<i32>) -> Option<RouteStats> {
    let mut all_paths = Vec::new();
    let mut visited = vec![false; graph.len()];
    
    fn dfs(current: usize, end: usize, visited: &mut Vec<bool>, 
           current_distance: i32, current_items: i32,
           graph: &Vec<Vec<(i32, i32)>>, num_items: &Vec<i32>,
           all_paths: &mut Vec<RouteStats>) {
        
        if current == end {
            // Found a path to the end
            all_paths.push(RouteStats {
                length: current_distance,
                max_items: current_items,
            });
            return;
        }
        
        // Try each neighbor
        for &(neighbor, edge_weight) in &graph[current] {
            let neighbor_idx = neighbor as usize;
            if !visited[neighbor_idx] {
                visited[neighbor_idx] = true;
                let new_distance = current_distance + edge_weight;
                let new_items = current_items + num_items[neighbor_idx];
                
                dfs(neighbor_idx, end, visited, new_distance, new_items,
                    graph, num_items, all_paths);
                
                visited[neighbor_idx] = false; // backtrack
            }
        }
    }
    
    visited[start] = true;
    dfs(start, end, &mut visited, 0, num_items[start], 
        graph, num_items, &mut all_paths);
    
    if all_paths.is_empty() {
        return None;
    }
    
    // Find the minimum distance (shortest path length)
    let min_distance = all_paths.iter().map(|stats| stats.length).min().unwrap();
    
    // Among all shortest paths, find the one with maximum items
    let best_path = all_paths.iter()
        .filter(|stats| stats.length == min_distance)
        .max_by_key(|stats| stats.max_items)
        .unwrap();
    
    Some(RouteStats {
        length: best_path.length,
        max_items: best_path.max_items,
    })
}

fn best_route(num_locations: i32, num_items: Vec<i32>,
    edges: Vec<(i32, i32, i32)>) -> Option<RouteStats>
{
    if num_locations == 0 {
        return None;
    }
    
    // Special case: if there are no edges and more than one location
    if edges.is_empty() && num_locations > 1 {
        return None; // Disconnected graph - endpoints inaccessible
    }
    
    // Special case: single location
    if num_locations == 1 {
        return Some(RouteStats { length: 0, max_items: num_items[0] });
    }
    
    // Build graph representation
    let graph = build_graph(num_locations, &edges);
    
    // Find shortest path from first location (0) to last location (num_locations-1)
    let start = 0;
    let end = (num_locations - 1) as usize;
    
    find_optimal_path(start, end, &graph, &num_items)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input file>", args[0]);
        return;
    }

    let contents: String = read_to_string(&args[1])
        .expect("Should have been able to read the file.");

    // What follows makes the assumption that the content
    // of an input file is properly formed. If we wanted to
    // write error code to handle input-file issues, then
    // much more would be needed...
    //

    let mut lines = contents.lines();
    let mut input_line = lines.next().unwrap();
    let location_count: i32 = input_line.parse().unwrap();

    input_line = lines.next().unwrap();
    let location_items: Vec<i32> = input_line
        .split_whitespace()
        .map(|s| s.parse::<i32>().expect("Invalid integer in file"))
        .collect();

    input_line = lines.next().unwrap();
    let edge_count: i32 = input_line.parse().unwrap();

    let mut edge_data: Vec<(i32, i32, i32)> = Vec::new();
    for _ in 0..edge_count {
        input_line = lines.next().unwrap();
        let e: Vec<i32> = input_line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Oh MAN! :-("))
            .collect();
        let edge = (e[0], e[1], e[2]);
        edge_data.push(edge);
    }

    let result = best_route(location_count, location_items,
        edge_data);
    
    match result {
        Some(stats) => println!("{} {}", stats.length, stats.max_items),
        None => println!("endpoint inaccessible"),
    } 

}

