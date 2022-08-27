use std::fs;

fn main() {
    let input = read_file_string("input.txt").unwrap();
    let moves: Vec<String> = input.split(", ").map(str::to_string).collect();
    let mut current_coordinates: [i32; 2] = [0, 0];
    let mut axis: i32 = 1;
    let mut direction: i32 = 1;

    for next_move in moves {
        let input_string = &String::from_iter(next_move.chars());
        let next_turn = input_string.chars().next().unwrap().to_string();
        let num_blocks = crop_letters(input_string, 1).parse::<i32>().unwrap();
        let is_pos_x = axis == 0 && direction == 1;
        let is_neg_x = axis == 0 && direction == -1;
        let is_pos_y = axis == 1 && direction == 1;
        let is_neg_y = axis == 1 && direction == -1;

        if is_pos_x {
            axis = 1;
            if next_turn == "R" {
                direction = -1;
            } else {
                direction = 1;
            }
        } else if is_neg_x {
            axis = 1;
            if next_turn == "R" {
                direction = 1;
            } else {
                direction = -1;
            }
        } else if is_pos_y {
            axis = 0;
            if next_turn == "R" {
                direction = 1;
            } else {
                direction = -1;
            }
        } else if is_neg_y {
            axis = 0;
            if next_turn == "R" {
                direction = -1;
            } else {
                direction = 1;
            }
        }
        current_coordinates = update_coordinate((axis, num_blocks, direction, current_coordinates));
    }
    println!("{:?}", current_coordinates);
    println!("{:?}", current_coordinates[0].abs() + current_coordinates[1].abs());
}

fn update_coordinate(
    (axis, input_mult, direction, mut current_coordinates): (i32, i32, i32, [i32; 2]),
) -> [i32; 2] {
    current_coordinates[axis as usize] += input_mult * direction;
    return current_coordinates;
}

fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read_to_string(filepath)?;
    Ok(data)
}
