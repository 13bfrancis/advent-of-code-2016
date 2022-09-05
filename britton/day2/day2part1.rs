use std::fs;

fn main() {
    let input: String = read_file_string("input.txt").unwrap();
    let directions: std::str::Split<&str> = input.split("\n");
    let num_pad: Vec<[i32; 3]> = vec![[1, 4, 7], [2, 5, 8], [3, 6, 9]];
    let mut x: usize = 1;
    let mut y: usize = 1;
    let mut code: Vec<i32> = Vec::new();
    let mut position: i32 = num_pad[x][y];

    directions.for_each(|line: &str| {
        line.chars().for_each(|c: char| {
            if c == 'L' && x > 0 {
                x = x - 1;
                position = num_pad[x][y];
            } else if c == 'R' && x < 2 {
                x = x + 1;
                position = num_pad[x][y];
            } else if c == 'U' && y > 0 {
                y = y - 1;
                position = num_pad[x][y];
            } else if c == 'D' && y < 2 {
                y = y + 1;
                position = num_pad[x][y];
            }
        });
        code.push(position);
    });
    println!("{:?}", code);
}

fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data: String = fs::read_to_string(filepath).expect("Unable to read file");
    Ok(data)
}