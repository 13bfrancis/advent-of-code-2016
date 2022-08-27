use std::{collections::HashMap, fs};

fn main() {
    let input_data: String = read_file_string("input.txt").unwrap();
    let moves: Vec<String> = input_data.split(", ").map(str::to_string).collect();
    let mut location: [i32; 2] = [0, 0];
    let mut axis: i32 = 1;
    let mut directionality: i32 = 1;
    let axis_map = HashMap::from([(0, 1), (1, 0)]);

    moves.into_iter().for_each(|next_move: String| {
        let next_input: &String = &String::from_iter(next_move.chars());
        let next_turn: String = next_input
            .chars()
            .next()
            .unwrap()
            .to_string()
            .to_lowercase();
        let num_blocks: i32 = crop_letters(next_input, 1).parse::<i32>().unwrap();
        let east: bool = axis == 0 && directionality == 1;
        let south: bool = axis == 1 && directionality == -1;
        axis = axis_map.get(&axis).unwrap().clone();

        match east || south {
            true => {
                directionality = 1;
                if next_turn == "r" {
                    directionality = -1;
                }
            }
            _ => {
                directionality = -1;
                if next_turn == "r" {
                    directionality = 1;
                }
            }
        };
        location = update_location((axis, num_blocks, directionality, location));
    });
    println!("{:?}", location);
    println!(
        "{:?}",
        location[0].abs() + location[1].abs()
    );
}

fn update_location(
    (axis, input_mult, directionality, mut location): (i32, i32, i32, [i32; 2]),
) -> [i32; 2] {
    location[axis as usize] += input_mult * directionality;
    return location;
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
