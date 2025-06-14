use std::collections::{HashMap, HashSet};

use regex::Regex;

#[derive(Clone)]
struct Edge {
    from: String,
    to: String,
    distance: i32,
}

#[derive(Clone)]
struct CityDistance {
    city: String,
    distance: i32,
}

fn extract_values(str: &str) -> Edge {
    let regex = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    let caps = match regex.captures(str) {
        Some(caps) => caps,
        None => panic!("Invalid input: {}", str),
    };
    let from = caps.get(1).unwrap().as_str().to_string();
    let to = caps.get(2).unwrap().as_str().to_string();
    let distance = caps.get(3).unwrap().as_str().parse().unwrap();

    Edge { from, to, distance }
}

fn minimum_path_from_city(
    current_city: String,
    graph: HashMap<String, Vec<CityDistance>>,
    mut visited: HashSet<String>,
    distance: i32,
) -> i32 {
    visited.insert(current_city.clone());
    if visited.len() == graph.len() {
        distance
    } else {
        let neighbors = graph.get(current_city.as_str()).unwrap();
        let mut min_distance = std::i32::MAX;
        for neighbor in neighbors {
            if !visited.contains(neighbor.city.as_str()) {
                min_distance = std::cmp::min(
                    min_distance,
                    minimum_path_from_city(
                        neighbor.city.clone(),
                        graph.clone(),
                        visited.clone(),
                        distance + neighbor.distance,
                    ),
                );
            }
        }
        min_distance
    }
}

/**
 * Part 1
 */
fn calculate_min_distance(graph: HashMap<String, Vec<CityDistance>>) -> i32 {
    let mut min_distance = std::i32::MAX;
    for (city, _) in graph.iter() {
        min_distance = std::cmp::min(
            min_distance,
            minimum_path_from_city(city.clone(), graph.clone(), HashSet::new(), 0),
        );
    }
    min_distance
}

/**
 * Part 2
 */
fn maximum_path_from_city(
    current_city: String,
    graph: HashMap<String, Vec<CityDistance>>,
    mut visited: HashSet<String>,
    distance: i32,
) -> i32 {
    visited.insert(current_city.clone());
    if visited.len() == graph.len() {
        distance
    } else {
        let neighbors = graph.get(current_city.as_str()).unwrap();
        let mut max_distance = std::i32::MIN;
        for neighbor in neighbors {
            if !visited.contains(neighbor.city.as_str()) {
                max_distance = std::cmp::max(
                    max_distance,
                    maximum_path_from_city(
                        neighbor.city.clone(),
                        graph.clone(),
                        visited.clone(),
                        distance + neighbor.distance,
                    ),
                );
            }
        }
        max_distance
    }
}

fn calculate_max_distance(graph: HashMap<String, Vec<CityDistance>>) -> i32 {
    let mut max_distance = std::i32::MIN;
    for (city, _) in graph.iter() {
        max_distance = std::cmp::max(
            max_distance,
            maximum_path_from_city(city.clone(), graph.clone(), HashSet::new(), 0),
        );
    }
    max_distance
}

pub fn run_ex9_2015() {
    let input = std::fs::read_to_string("src/ex9_2015/input.txt").unwrap();
    let mut graph: HashMap<String, Vec<CityDistance>> = HashMap::new();
    for line in input.lines() {
        let edge = extract_values(line);
        graph
            .entry(edge.from.clone())
            .or_insert(Vec::new())
            .push(CityDistance {
                city: edge.to.clone(),
                distance: edge.distance,
            });
        graph
            .entry(edge.to.clone())
            .or_insert(Vec::new())
            .push(CityDistance {
                city: edge.from.clone(),
                distance: edge.distance,
            });
        // println!("{} {} {}", edge.from, edge.to, edge.distance);
    }

    let min_distance = calculate_min_distance(graph.clone());
    println!("Min distance: {}", min_distance);

    let max_distance = calculate_max_distance(graph);
    println!("Max distance: {}", max_distance);
}
