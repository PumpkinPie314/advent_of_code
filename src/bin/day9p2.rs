use itertools::Itertools;

fn main() {
    // let mut input = "2333133121414131402".to_string();
    let mut input = include_str!("inputs/day9").to_string();

    if input.len() % 2 == 1 {input.push('0')}; // forces input to be even
    let (not_free, free):(Vec<usize>, Vec<usize>)
     = input.chars()
        .map(|x|x.to_digit(10).unwrap() as usize)
        .tuples()
        .unzip();
    let not_free = not_free.into_iter().enumerate().map(|(id, size)| (Some(id), size));
    let free = free.into_iter().map(|size| (None, size));

    // Vec<content, size> where content is Some(id) or None(empty)
    let disk_map= not_free.zip(free).flat_map(|(a,b)|[a,b]).collect::<Vec<_>>();
    let mut p2 = disk_map.clone();
    for right_block in disk_map.iter().rev() {
        let &(rcont, rsize) = right_block;
        if rcont.is_none() {continue;};
        let ridx = p2.iter().find_position(|&x|x == right_block).unwrap().0;
        for left_block in p2.iter().enumerate() {
            let (lidx, &(lcont, lsize)) = left_block;
            if lcont.is_some() {continue;};
            // done defineing stuff
            if lsize < rsize {continue;}
            if lidx > ridx {break;}
            p2[ridx] = (None, rsize);
            p2.insert(lidx, (rcont, rsize));
            p2[lidx + 1] = (None, lsize - rsize);
            break;

        }
    }
    let check_sum: usize = p2.clone().into_iter().flat_map(|x|{
        let (content, size) = x;
        match content {
            None => {
                vec![None; size]
                // vec!['.'; size]
            },
            Some(x) => {
                vec![Some(x); size]
                // vec![x.to_string().chars().next().unwrap(); size]
            }
        }
    }).enumerate().filter_map(|(pos, id)|{
        if let Some(id) = id {
            Some(pos*id)
        } else {None}
    }).sum();
    println!("{}", check_sum);
    

}

