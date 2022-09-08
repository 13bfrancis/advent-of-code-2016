use std::fs;

fn main() {
    let input: String = read_file_string("input.txt").unwrap();
    let directions: std::str::Split<&str> = input.split("\n");
    let num_pad: Vec<[&str; 5]> = vec![
        ["-", "-", "1", "-", "-"],
        ["-", "2", "3", "4", "-"],
        ["5", "6", "7", "8", "9"],
        ["-", "A", "B", "C", "-"],
        ["-", "-", "D", "-", "-"],
    ];

    let mut x: usize = 0;
    let mut y: usize = 2;
    let mut code: Vec<&str> = Vec::new();
    let mut position: &str = num_pad[y][x];

    directions.for_each(|line: &str| {
        line.chars().for_each(|c: char| {
            let mut is_button: bool = false;

            if c == 'L' {
                let mut new_x: usize = x;
                if x > 0 {
                    new_x = x - 1;
                    is_button = num_pad[y][new_x] != "-";
                }
                if is_button {
                    position = num_pad[y][new_x];
                    x = new_x;
                }
            } else if c == 'R' {
                let new_x: usize = x + 1;

                if new_x <= 4 {
                    is_button = num_pad[y][new_x] != "-";
                }
                if is_button {
                    position = num_pad[y][new_x];
                    x = new_x;
                }
            } else if c == 'U' {
                let mut new_y: usize = y;
                if y > 0 {
                    new_y = y - 1;
                    is_button = num_pad[new_y][x] != "-";
                }
                if is_button {
                    position = num_pad[new_y][x];
                    y = new_y;
                }
            } else if c == 'D' {
                let new_y: usize = y + 1;

                if new_y <= 4 {
                    is_button = num_pad[new_y][x] != "-";
                }
                if is_button {
                    position = num_pad[new_y][x];
                    y = new_y;
                }
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
