fn main() {
    let dirs: String = String::from("R4, R3, R5, L3, L5, R2, L2, R5, L2, R5, R5, R5, R1, R3, L2, L2, L1, R5, L3, R1, L2, R1, L3, L5, L1, R3, L4, R2, R4, L3, L1, R4, L4, R3, L5, L3, R188, R4, L1, R48, L5, R4, R71, R3, L2, R188, L3, R2, L3, R3, L5, L1, R1, L2, L4, L2, R5, L3, R3, R3, R4, L3, L4, R5, L4, L4, R3, R4, L4, R1, L3, L1, L1, R4, R1, L4, R1, L1, L3, R2, L2, R2, L1, R5, R3, R4, L5, R2, R5, L5, R1, R2, L1, L3, R3, R1, R3, L4, R4, L4, L1, R1, L2, L2, L4, R1, L3, R4, L2, R3, L1, L5, R4, R5, R2, R5, R1, R5, R1, R3, L3, L2, L2, L5, R2, L2, R5, R5, L2, R3, L5, R5, L2, R4, R2, L1, R3, L5, R3, R2, R5, L1, R3, L2, R2, R1");
    let dir_vect: Vec<String> = dirs.split(", ").map(str::to_string).collect();
    let mut current_loc: [i32; 2] = [0, 0];
    let mut coord_to_mod: i32 = 1;
    let mut current_mult: i32 = 1;

    for dir in dir_vect {
        let is_negative = current_mult == -1;
        let is_positive = current_mult == 1;
        let is_x = coord_to_mod == 0;
        let is_y = coord_to_mod == 1;
        let direction = dir.chars().nth(0).unwrap().to_string();
        let input_text = dir
            .chars();
        let input_string = &String::from_iter(input_text);
        let multiplier = crop_letters(input_string, 1).parse::<i32>().unwrap();

        if is_x && is_positive {
            coord_to_mod = 1;
            if direction == "R" {
                current_mult = -1;
            } else {
                current_mult = 1;
            }
        } else if is_x && is_negative {
            coord_to_mod = 1;
            if direction == "R" {
                current_mult = 1;
            } else {
                current_mult = -1;
            }
        } else if is_y && is_positive {
            coord_to_mod = 0;
            if direction == "R" {
                current_mult = 1;
            } else {
                current_mult = -1;
            }
        } else if is_y && is_negative {
            coord_to_mod = 0;
            if direction == "R" {
                current_mult = -1;
            } else {
                current_mult = 1;
            }
        }
        current_loc = update_coordinate((coord_to_mod, multiplier, current_mult, current_loc));
    }

    println!("{:?}", current_loc);
    println!("{:?}", current_loc[0].abs() + current_loc[1].abs())
}

fn update_coordinate(
    (coord_to_mod, input_mult, current_mult, mut current_loc): (i32, i32, i32, [i32; 2]),
) -> [i32; 2] {
    current_loc[coord_to_mod as usize] += input_mult * current_mult;
    return current_loc;
}

fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}