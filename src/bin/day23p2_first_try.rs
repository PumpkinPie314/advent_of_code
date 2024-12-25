use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}};

fn main()  {
    let input = include_str!("tests/day23");
    // let input = include_str!("inputs/day23");
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let l = line.split('-').collect::<Vec<_>>();
        let (a, b) = (l[0], l[1]);
        graph.entry(a).or_default().push(b);
        graph.entry(b).or_default().push(a);
    }
    // Bron Kerbosch
    let cliques = Arc::new(Mutex::new(vec![]));
    let r = HashSet::new();
    let x = HashSet::new();
    let p = graph.keys().map(|&x|x).collect();
    bron_kerbosch(cliques.clone(), &graph, r, p, x);
    let cliques = cliques.lock().unwrap().clone();
    let p2 = cliques.into_iter()
        .max_by(|a,b|a.len().cmp(&b.len()))
        .unwrap();
    println!("{:?}", p2)
}
fn bron_kerbosch<'a>(cliques: Arc<Mutex<Vec<HashSet<&'a str>>>>, graph: &'a HashMap<&'a str, Vec<&'a str>>, r: HashSet<&'a str>, mut p: HashSet<&'a str>, mut x: HashSet<&'a str>){
    let union_px: HashSet<&str> = p.union(&x).map(|&x|x).collect::<HashSet<_>>();
    if union_px.is_empty() {
        let mut cliques_locked = cliques.lock().unwrap();
        cliques_locked.push(r.clone());
        return;
    }
    // the pivot is chosen to have the most nabors
    let pivot = *union_px.iter().next().unwrap();
    let pivot_nabors = graph.get(pivot).unwrap().iter().map(|&x|x).collect::<HashSet<_>>();
    let p_minus_pivot_nabors = p.difference(&pivot_nabors).map(|&x|x).collect::<HashSet<_>>();
    for v in p_minus_pivot_nabors {
        let nr = r.union(&HashSet::from([v])).map(|&x|x).collect();
        let np = p.intersection(&pivot_nabors).map(|&x|x).collect();
        let nx = x.intersection(&pivot_nabors).map(|&x|x).collect();
        let cliques_clone = Arc::clone(&cliques);
        bron_kerbosch(cliques_clone, graph, nr, np, nx);
        // r.remove(&v);
        p.remove(&v);
        x.insert(v);
    };
}