use std::{collections::HashMap, fs};

fn main() {
    let axis: HashMap<i32, i32> = HashMap::from([(0, 1), (1, 0)]);
    let cardinal_directions: HashMap<[i32; 2], &str> =
        HashMap::from([([1, 1], "n"), ([0, 1], "e"), ([0, -1], "w"), ([1, -1], "s")]);
    let input_data: String = read_file_string("input.txt").unwrap();
    let moves: Vec<String> = input_data.split(", ").map(str::to_string).collect();
    let mut location: [i32; 2] = [0, 0];
    let mut current_axis: i32 = 1;
    let mut direction_of_change: i32 = 1;

    moves.into_iter().for_each(|next_move: String| {
        let next_input: &String = &String::from_iter(next_move.chars());
        let next_turn: String = next_input
            .chars()
            .next()
            .unwrap()
            .to_string()
            .to_lowercase();
        let num_blocks: i32 = crop_letters(next_input, 1).parse::<i32>().unwrap();
        let direction: String = cardinal_directions
            .get(&[current_axis, direction_of_change])
            .unwrap()
            .to_string();
        let res = next_turn == "r" && (direction == "e" || direction == "s");
        let lnw = next_turn == "l" && (direction == "n" || direction == "w");

        current_axis = axis.get(&current_axis).unwrap().clone();
        direction_of_change = 1;

        if res || lnw {
            direction_of_change = -1;
        }
        location = update_location((location, current_axis, num_blocks, direction_of_change));
    });

    let abs_value: i32 = location[0].abs() + location[1].abs();
    let output: String = format!(
        "# of blocks: {}\nLocation: [{},{}]",
        abs_value, location[0], location[1]
    );

    write_file_string("output.txt", output);
}

fn update_location(
    (mut location, current_axis, input_mult, direction_of_change): ([i32; 2], i32, i32, i32),
) -> [i32; 2] {
    location[current_axis as usize] += input_mult * direction_of_change;
    return location;
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

fn write_file_string(filepath: &str, data: String) {
    fs::write(filepath, data).expect("Unable to write file");
}
