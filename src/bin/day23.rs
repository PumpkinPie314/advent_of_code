use std::collections::{HashMap, HashSet};

fn main() {
    // let input = include_str!("tests/day23");
    let input = include_str!("inputs/day23");
    // given a node, give the nabors
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let l = line.split('-').collect::<Vec<_>>();
        let (a, b) = (l[0], l[1]);
        graph.entry(a).or_default().push(b);
        graph.entry(b).or_default().push(a);
    }
    let mut cliques = HashSet::new();
    for (node, adjacent) in &graph {
        if !node.starts_with('t') {continue;}
        if adjacent.len() < 2 {continue;}
        for adj in adjacent {
            for &cousin in graph.get(adj).unwrap() {
                if graph.get(cousin).unwrap().contains(&node) {
                    cliques.insert((*node, *adj, cousin));
                }
            }
        }
    }
    //dedupe
    let mut p1 = 0;
    for clique in cliques.clone() {
        let (a, b, c) = clique;
        if cliques.contains(&clique) {
            p1 += 1;
            // println!("{:?}", clique)
        }
        cliques.remove(&(a,c,b));
        cliques.remove(&(b,a,c));
        cliques.remove(&(b,c,a));
        cliques.remove(&(c,a,b));
        cliques.remove(&(c,b,a));
    };
    println!("{}", p1)
    //2306 too high
}