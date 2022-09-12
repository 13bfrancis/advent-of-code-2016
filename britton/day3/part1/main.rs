use std::fs;
use std::str::FromStr;

fn main() {
    let input: String = read_file_string("input.txt").unwrap();
    let triangles: std::str::Split<&str> = input.split("\n");

    let mut valid_triangles: i32 = 0;

    triangles.for_each(|mut triangle: &str| {
        triangle = triangle.trim_start();
        let lengths: Vec<&str> = triangle.split_whitespace().collect();
        let side_x: u32 = FromStr::from_str(lengths[0]).unwrap();
        let side_y: u32 = FromStr::from_str(lengths[1]).unwrap();
        let side_z: u32 = FromStr::from_str(lengths[2]).unwrap();

        if side_x + side_y > side_z && side_x + side_z > side_y && side_y + side_z > side_x {
            valid_triangles = valid_triangles + 1;
        }
    });
    println!("{:?}", valid_triangles);
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data: String = fs::read_to_string(filepath).expect("Unable to read file");
    Ok(data)
}
