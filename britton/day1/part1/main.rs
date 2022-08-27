use std::fs;

fn main() {
    let input_data: String = read_file_string("input.txt").unwrap();
    let moves: Vec<String> = input_data.split(", ").map(str::to_string).collect();
    let mut current_coordinates: [i32; 2] = [0, 0];
    let mut axis: i32 = 1;
    let mut direction: i32 = 1;

    moves.into_iter().for_each(|next_move: String| {
        let next_input: &String = &String::from_iter(next_move.chars());
        let next_turn: String = next_input.chars().next().unwrap().to_string();
        let num_blocks: i32 = crop_letters(next_input, 1).parse::<i32>().unwrap();
        let pos_x: bool = axis == 0 && direction == 1;
        let neg_x: bool = axis == 0 && direction == -1;
        let pos_y: bool = axis == 1 && direction == 1;
        let neg_y: bool = axis == 1 && direction == -1;
        let next_turn_right: bool = next_turn == "R";

        if pos_x {
            axis = 1;
            if next_turn_right {
                direction = -1;
            } else {
                direction = 1;
            }
        } else if neg_x {
            axis = 1;
            if next_turn_right {
                direction = 1;
            } else {
                direction = -1;
            }
        } else if pos_y {
            axis = 0;
            if next_turn_right {
                direction = 1;
            } else {
                direction = -1;
            }
        } else if neg_y {
            axis = 0;
            if next_turn_right {
                direction = -1;
            } else {
                direction = 1;
            }
        }
        current_coordinates = update_coordinate((axis, num_blocks, direction, current_coordinates));
    });
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
    let data: String = fs::read_to_string(filepath)?;
    Ok(data)
}
