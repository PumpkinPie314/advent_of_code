use itertools::Itertools;

fn main() {
    let input = "2333133121414131402";
    // let input = include_str!("inputs/day9");
    
    // un pack disk from disk map
    let mut disk: Vec<Option<usize>> = disk_from_disk_map(input);
    debug_disk(&disk);
    // push everything left
    let free_space_pos = disk.clone().into_iter().positions(|x|x.is_none());
    let filled_space_pos = disk.clone().into_iter().positions(|x|x.is_some());
    
    let left_positions = free_space_pos;
    let right_positions = filled_space_pos.rev();
    
    let pairs_to_swap = left_positions.zip(right_positions).peekable();
    for pair in pairs_to_swap {
        let (free_pos, filled_pos) = pair;
        if free_pos > filled_pos {break;}
        disk[free_pos] = disk[filled_pos];
        disk[filled_pos] = None;
    }
    debug_disk(&disk);
    // calculate check-sum
    let p1: usize = disk.into_iter().enumerate().filter_map(|(pos, id)|{
        if let Some(id) = id {
            Some(pos*id)
        } else {None}
    }).sum();
    println!("{p1}")



}

fn disk_from_disk_map(disk_map: &str) -> Vec<Option<usize>>{
    let mut stream = disk_map.chars().into_iter();
    
    let mut disk: Vec<Option<usize>> = vec![];
    let mut is_empty_space = false;
    let mut id = 0;
    while let Some(block_size) = stream.next() {
        let block_size: usize = block_size.to_digit(10).expect("not a digit") as usize; 
        match is_empty_space {
            true => {
                let block= vec![None; block_size];
                disk.extend(block);
            },
            false => {
                let block= vec![Some(id); block_size];
                disk.extend(block);
                id += 1;
            }
        }
        is_empty_space = ! is_empty_space;
    };
    disk
}

// fn debug_positions(position_list: &(impl Iterator<Item = usize> + Clone)) {
//     let out_size = position_list.clone().max().expect("no positions").to_owned();
//     let mut out = vec!['.'; out_size+1];
//     let mut i = 0;
//     for pos in position_list.clone() {
//         out[pos] = i.to_string().chars().last().unwrap();
//         i += 1;
//     }
//     println!("{}",out.into_iter().collect::<String>())
// }

fn debug_disk(disk: &Vec<Option<usize>>){
    let out = disk.into_iter().map(|x|{
        match x {
            None => {'.'},
            Some(x) => {x.to_string().chars().next().unwrap()}
        }
    }).collect::<String>();
    println!("{}", out)
}