use std::collections::{HashMap, HashSet};

fn main() {
    // let input = include_str!("test");
    let input = include_str!("inputs/day5");
    //parse
    let mut get_left_of_key: HashMap<i32, HashSet<i32>> = HashMap::new();//given a page, gives all the values that ought to be before it in sequence. 
    let mut updates: Vec<Vec<i32>> = vec![];
    for line in input.lines() {
        if line.contains('|') {
            let mut split = line.split('|');
            let first: i32 = split.next().unwrap().parse().unwrap();
            let second: i32 = split.next().unwrap().parse().unwrap();
            get_left_of_key.entry(second).or_default().insert(first);

        } else if !line.is_empty() {
            let pages: Vec<i32> = line.split(',').map(|x|x.parse().unwrap()).collect();
            updates.push(pages);
        }
    };
    let ordered_updates: Vec<Vec<i32>> = updates.iter().filter(|update|is_ordered(&get_left_of_key, &update)).cloned().collect();
    let middle_pages: Vec<i32> = ordered_updates.into_iter().map(|update|{
        let middle_index = update.len()/2;
        update[middle_index]
    }).collect();
    let out: i32 = middle_pages.into_iter().sum();
    println!("p1: {out:?}");

    
    let unordered_updates: Vec<Vec<i32>> = updates.iter().filter(|update|!is_ordered(&get_left_of_key, &update)).cloned().collect();
    let newly_sorted: Vec<Vec<i32>> = unordered_updates.into_iter().map(|mut update|{
        update.sort_by(|a,b |get_left_of_key[b].contains(a).cmp(&true));
        update
    }).collect::<Vec<_>>();
    let out2 = newly_sorted.into_iter().fold(0, |acc, x|acc+x[x.len()/2]);
    println!("p2: {out2:?}");

}

fn is_ordered(get_left_of_key: &HashMap<i32, HashSet<i32>>, pages: &Vec<i32>) -> bool {
    pages.is_sorted_by(|left_page, right_page| {
        get_left_of_key[right_page].contains(left_page)
    })
}