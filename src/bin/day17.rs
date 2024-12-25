fn main() {
    let input = include_str!("inputs/day17");
    let sections = input.lines()
        .map(|x|{
            x.chars().filter(|c|c.is_digit(10)).collect::<String>()
        }).collect::<Vec<_>>();
    let (a_reg,b_reg,c_reg, program): (usize, usize, usize, Vec<u8>) = (
        sections[0].parse().unwrap(),
        sections[1].parse().unwrap(),
        sections[2].parse().unwrap(), 
        sections[4].chars().map(|x|x.to_digit(10).unwrap() as u8).collect()
    );
    println!("p1: {}", run(a_reg,b_reg,c_reg, &program));
    // let checked_up_to = 568700000;
    // skip 10000000000000;
    let skipped_checked_up_to = 10001883000000;
    println!("target:   2,4,1,1,7,5,1,5,4,5,0,3,5,5,3,0,");
    println!("progress: {}", run(skipped_checked_up_to, 0, 0, &program));
    // 2,4,1,1,7,5,1,5,4,5,0,3,5,5,3,0,
    for a_attempt in skipped_checked_up_to.. {
        if run(a_attempt,0,0, &program) == "2,4,1,1,7,5,1,5,4,5,0,3,5,5,3,0,".to_string(){
            println!("\np2: {}", a_attempt);
            break;
        }
        if a_attempt%1000000 == 0 {print!("{a_attempt}\t")}
    } 
}





fn run(mut a_reg: usize, mut b_reg: usize, mut c_reg: usize, program: &Vec<u8>) -> String {

    let mut instruction_pointer = 0usize; 
    let mut out = String::new();
    loop {
        if instruction_pointer >= program.len()-1 {break;} // halt
        let opperand = program[instruction_pointer + 1];
        let literal_opperand = opperand as usize;
        let combo_opperand = {
            match opperand {
                0..=3 => {literal_opperand}
                4 => {a_reg}
                5 => {b_reg}
                6 => {c_reg}
                7 => {panic!("reserved")}
                8.. => {panic!("not 3 bit")}
            }
        };
        match program[instruction_pointer] {
            // adv
            0 => {
                // println!("devided {} by {}. set A to {}", a_reg, 2usize.pow(combo_opperand as u32), a_reg/2usize.pow(combo_opperand as u32) );
                // a_reg = a_reg.div_floor(2usize.pow(combo_opperand as u32));
                a_reg = a_reg / 2usize.pow(combo_opperand as u32);
            }
            // bxl
            1 => {
                // println!("Takeing {b_reg}%8. and putting it into B");
                b_reg = b_reg ^ literal_opperand;
            }
            // bst
            2 => {
                // println!("Takeing {b_reg}%8. and putting it into B");
                b_reg = combo_opperand%8;
            }
            // jnz
            3 => {
                if a_reg == 0 {
                    // println!("A: {a_reg} is zero. continuing");
                    // nothing
                } else {
                    // println!("A: {a_reg} is not zero. Jumping to {literal_opperand}");
                    instruction_pointer = literal_opperand;
                    continue; // skips advancing the p instruction_pointer
                }
            }
            // bxc
            4 => {
                // println!("xord {b_reg} and {c_reg}. set B to {}", b_reg ^ c_reg );
                b_reg = b_reg ^ c_reg;
            }
            // out
            5 => {
                // println!("outputing {}%8 = {}", combo_opperand, combo_opperand%8);
                out.push_str(format!("{},", combo_opperand%8).as_str());
            }
            // bdv
            6 => {
                // println!("devided {} by {}. set B to {}", a_reg, 2usize.pow(combo_opperand as u32), a_reg/2usize.pow(combo_opperand as u32) );
                b_reg = a_reg/2usize.pow(combo_opperand as u32);
            }
            // cdv
            7 => {
                // println!("devided {} by {}. set C to {}", a_reg, 2usize.pow(combo_opperand as u32), a_reg/2usize.pow(combo_opperand as u32) );
                c_reg = a_reg/2usize.pow(combo_opperand as u32);
            }
            8.. => {panic!("opcode is not 3 bit")}
        }
        instruction_pointer += 2;
    }
    out
}