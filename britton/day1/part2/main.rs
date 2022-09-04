use std::collections::HashMap;
use std::fs;

struct Move {
    direction: String,
    blocks: i32,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Location {
    x: i32,
    y: i32,
}

fn main() {
    let cardinal_directions: HashMap<[i32; 2], &str> =
        HashMap::from([([1, 1], "n"), ([0, 1], "e"), ([0, -1], "w"), ([1, -1], "s")]);
    let axis: HashMap<i32, i32> = HashMap::from([(0, 1), (1, 0)]);
    let mut current_location: Location = Location { x: (0), y: (0) };
    let mut current_axis: i32 = 1;
    let mut direction_of_change: i32 = 1;
    let mut visited_locations: Vec<Location> = Vec::new();
    let mut first_hit: bool = true;

    let mapped_moves: Vec<Move> = get_moves();

    mapped_moves.into_iter().for_each(|next_move: Move| {
        let direction: String = cardinal_directions
            .get(&[current_axis, direction_of_change])
            .unwrap()
            .to_string();
        let res: bool = next_move.direction == "r" && (direction == "e" || direction == "s");
        let lnw: bool = next_move.direction == "l" && (direction == "n" || direction == "w");

        current_axis = axis.get(&current_axis).unwrap().clone();
        direction_of_change = 1;

        if res || lnw {
            direction_of_change = -1;
        }

        let num_blocks: i32 = next_move.blocks * direction_of_change;
        let vals: (Vec<Location>, Location) =
            parse_move(current_location, current_axis, num_blocks);
        let next_locations: Vec<Location> = vals.0;

        let cur_loc: Location = vals.1;
        current_location = cur_loc;
        next_locations.into_iter().for_each(|location: Location| {
            if visited_locations.contains(&location) && first_hit == true {
                println!("{:?}", location);
                first_hit = false;
            }
            visited_locations.push(location.clone());
        });
    });
    println!("{:?}", visited_locations.last().unwrap());
}

fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data: String = fs::read_to_string(filepath).expect("Unable to read file");
    Ok(data)
}

fn parse_move(
    current_location: Location,
    current_axis: i32,
    num_blocks: i32,
) -> (Vec<Location>, Location) {
    let mut locations: Vec<Location> = Vec::new();
    let counter: i32 = num_blocks.abs() + 1;
    let x: i32 = current_location.x.clone();
    let y: i32 = current_location.y.clone();

    let mut next_location: Location = Location {
        x: (current_location.x),
        y: (current_location.y),
    };

    for _i in 1..counter {
        if current_axis == 0 {
            let new_x: i32 = next_location.x + (num_blocks.abs() / num_blocks);
            next_location = Location { x: (new_x), y: (y) };
            locations.push(next_location);
        } else if current_axis == 1 {
            let new_y: i32 = next_location.y + (num_blocks.abs() / num_blocks);
            next_location = Location { x: (x), y: (new_y) };
            locations.push(next_location);
        }
    }
    return (locations, next_location);
}

fn get_moves() -> Vec<Move> {
    let input_data: String = read_file_string("input.txt").unwrap();
    let raw_moves: Vec<String> = input_data.split(", ").map(str::to_string).collect();
    let mut mapped_moves: Vec<Move> = Vec::new();

    raw_moves.into_iter().for_each(|next_move: String| {
        let next_input: &String = &String::from_iter(next_move.chars());
        let next_turn: String = next_input
            .chars()
            .next()
            .unwrap()
            .to_string()
            .to_lowercase();
        let num_blocks: i32 = crop_letters(next_input, 1).parse::<i32>().unwrap();

        let current_move: Move = Move {
            direction: next_turn,
            blocks: num_blocks,
        };
        mapped_moves.push(current_move);
    });
    return mapped_moves;
}
