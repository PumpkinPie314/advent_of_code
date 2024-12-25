use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

fn main() {
    // let input = include_str!("tests/day23");
    let input = include_str!("inputs/day23");
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let l = line.split('-').collect::<Vec<_>>();
        let (a, b) = (l[0], l[1]);
        graph.entry(a).or_default().insert(b);
        graph.entry(b).or_default().insert(a);
    }
    // Initial state
    
    let mut cliques = Vec::new();
    let mut stack = VecDeque::new();
    stack.push_back((
        HashSet::new(),        // Current clique (R)
        graph.keys().cloned().collect::<HashSet<_>>(), // Potential nodes to add (P)
        HashSet::new(),        // Excluded nodes (X)
    ));

    while let Some((r, mut p, mut x)) = stack.pop_back() {
        if p.is_empty() && x.is_empty() {
            // Found a maximal clique
            cliques.push(r);
        } else {
            // Choose a pivot u from P ∪ X (we use a node from P if available)
            let u = p.union(&x).next().cloned().unwrap();

            // Consider only nodes in P that are not neighbors of the pivot
            let non_neighbors: HashSet<_> = p.difference(graph.get(u).unwrap()).cloned().collect();

            for v in non_neighbors {
                let mut r_new = r.clone();
                r_new.insert(v);

                let neighbors: HashSet<_> = graph[v].iter().cloned().collect();
                let p_new = &p & &neighbors;
                let x_new = &x & &neighbors;

                stack.push_back((r_new, p_new, x_new));

                // Update P and X
                p.remove(&v);
                x.insert(v);
            }
        }
    }
    let p2 = cliques.into_iter()
        .max_by(|a, b|a.len().cmp(&b.len()))
        .unwrap()
        .into_iter()
        .sorted()
        .join(",");
    println!("{}", p2);
}