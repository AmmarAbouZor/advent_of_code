#![allow(warnings, unused)]
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use itertools::{Itertools, MinMaxResult};

use crate::utls::read_lines_from_file;

#[derive(Debug)]
struct Root {
    pub location_1: String,
    pub location_2: String,
}

impl Root {
    fn new(location_1: &str, location_2: &str) -> Self {
        Self {
            location_1: location_1.into(),
            location_2: location_2.into(),
        }
    }
}

impl PartialEq for Root {
    fn eq(&self, other: &Self) -> bool {
        (self.location_1 == other.location_1 && self.location_2 == other.location_2)
            || (self.location_1 == other.location_2 && self.location_2 == other.location_1)
    }
}

impl Eq for Root {}

impl Hash for Root {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut locations = [&self.location_1, &self.location_2];
        locations.sort();
        for location in locations {
            location.hash(state);
        }
    }
}

#[derive(Debug)]
struct DistanceInfo {
    root: Root,
    distance: usize,
}

impl From<&str> for DistanceInfo {
    fn from(text: &str) -> Self {
        let parts: Vec<&str> = text.split(' ').collect();
        DistanceInfo {
            root: Root::new(parts[0], parts[2]),
            distance: parts[4].parse().unwrap(),
        }
    }
}

fn calc_route_distance(route: Vec<&String>, distance_map: &HashMap<Root, usize>) -> usize {
    let mut distance = 0;
    let mut index = 0usize;
    while index < route.len() - 1 {
        let current_loc = route[index];
        let next_loc = route[index + 1];
        distance += distance_map.get(&Root::new(current_loc, next_loc)).unwrap();
        index += 1;
    }

    distance
}

pub fn run() {
    let distances: Vec<DistanceInfo> = read_lines_from_file(r"src/year_15/day_9.txt")
        .iter()
        .map(|line| line.as_str().into())
        .collect();

    let locations: Vec<String> = distances
        .iter()
        .map(|info| [info.root.location_1.clone(), info.root.location_2.clone()])
        .flatten()
        .unique()
        .collect();

    let distance_map: HashMap<Root, usize> = distances
        .into_iter()
        .map(|info| (info.root, info.distance))
        .collect();

    let shortest = locations
        .iter()
        .permutations(locations.len())
        .unique()
        .map(|route| calc_route_distance(route, &distance_map))
        .minmax();

    if let MinMaxResult::MinMax(min, max) = shortest {
        println!("shortest distance is {min}");
        println!("longest distance is {max}");
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_distance_map() {
        let mut map = HashMap::new();
        map.insert(Root::new("A", "B"), 2);
        assert!(map.get(&Root::new("A", "B")).is_some());
        assert!(map.get(&Root::new("B", "A")).is_some());
    }
}
