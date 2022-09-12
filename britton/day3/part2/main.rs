use std::fs;
use std::str::FromStr;

fn main() {
    let input_string: String = read_file_string("input.txt").unwrap();
    let cln_input: Vec<[u32; 3]> = get_cln_input(input_string);
    let num_valid: u32 = get_num_valid(cln_input);
    println!("{:?}", num_valid);
}

fn get_cln_input(input_string: String) -> Vec<[u32; 3]> {
    let inputs: std::str::Split<&str> = input_string.split("\n");
    let mut cln_input: Vec<[u32; 3]> = Vec::new();

    inputs.for_each(|mut input: &str| {
        input = input.trim_start();
        let lengths: Vec<&str> = input.split_whitespace().collect();
        let val1: u32 = FromStr::from_str(lengths[0]).unwrap();
        let val2: u32 = FromStr::from_str(lengths[1]).unwrap();
        let val3: u32 = FromStr::from_str(lengths[2]).unwrap();

        cln_input.push([val1, val2, val3]);
    });

    return cln_input;
}

fn is_valid_triangle(side_x: u32, side_y: u32, side_z: u32) -> bool {
    let mut is_valid_triangle: bool = false;
    if side_x + side_y > side_z && side_x + side_z > side_y && side_y + side_z > side_x {
        is_valid_triangle = true;
    }

    return is_valid_triangle;
}

fn get_num_valid(cln_input: Vec<[u32; 3]>) -> u32 {
    let mut start: usize = 0;
    let mut end: usize = 2;
    let mut num_valid: u32 = 0;

    while end < cln_input.len() {
        let mut side_x: u32 = cln_input[start..=end][0][0];
        let mut side_y: u32 = cln_input[start..=end][1][0];
        let mut side_z: u32 = cln_input[start..=end][2][0];

        if is_valid_triangle(side_x, side_y, side_z) {
            num_valid = num_valid + 1;
        }

        side_x = cln_input[start..=end][0][1];
        side_y = cln_input[start..=end][1][1];
        side_z = cln_input[start..=end][2][1];

        if is_valid_triangle(side_x, side_y, side_z) {
            num_valid = num_valid + 1;
        }

        side_x = cln_input[start..=end][0][2];
        side_y = cln_input[start..=end][1][2];
        side_z = cln_input[start..=end][2][2];

        if is_valid_triangle(side_x, side_y, side_z) {
            num_valid = num_valid + 1;
        }

        start = start + 3;
        end = end + 3;
    }

    return num_valid;
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data: String = fs::read_to_string(filepath).expect("Unable to read file");
    Ok(data)
}
