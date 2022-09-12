use std::fs;
use std::str::FromStr;

fn main() {
    let input_string: String = read_file_string("input.txt").unwrap();
    let inputs: std::str::Split<&str> = input_string.split("\n");
    //  let mut valid_triangles: i32 = 0;
    let mut clean_input: Vec<[u32; 3]> = Vec::new();

    inputs.for_each(|mut input: &str| {
        input = input.trim_start();
        let lengths: Vec<&str> = input.split_whitespace().collect();
        let val1: u32 = FromStr::from_str(lengths[0]).unwrap();
        let val2: u32 = FromStr::from_str(lengths[1]).unwrap();
        let val3: u32 = FromStr::from_str(lengths[2]).unwrap();

        clean_input.push([val1, val2, val3]);
    });

    let mut start: usize = 0;
    let mut end: usize = 2;
    let mut valid_triangles = 0;

    while end < clean_input.len() {
        let mut side_x = clean_input[start..=end][0][0];
        let mut side_y = clean_input[start..=end][1][0];
        let mut side_z = clean_input[start..=end][2][0];

        if side_x + side_y > side_z && side_x + side_z > side_y && side_y + side_z > side_x {
            valid_triangles = valid_triangles + 1;
        }

        side_x = clean_input[start..=end][0][1];
        side_y = clean_input[start..=end][1][1];
        side_z = clean_input[start..=end][2][1];

        if side_x + side_y > side_z && side_x + side_z > side_y && side_y + side_z > side_x {
            valid_triangles = valid_triangles + 1;
        }

        side_x = clean_input[start..=end][0][2];
        side_y = clean_input[start..=end][1][2];
        side_z = clean_input[start..=end][2][2];

        if side_x + side_y > side_z && side_x + side_z > side_y && side_y + side_z > side_x {
            valid_triangles = valid_triangles + 1;
        }

        start = start + 3;
        end = end + 3;
    }
    println!("{:?}", valid_triangles);
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data: String = fs::read_to_string(filepath).expect("Unable to read file");
    Ok(data)
}
