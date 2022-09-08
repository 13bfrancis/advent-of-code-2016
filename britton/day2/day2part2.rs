use std::fs;

fn main() {
    let input: String = read_file_string("input.txt").unwrap();
    let directions: std::str::Split<&str> = input.split("\n");
    let num_pad: Vec<[char; 5]> = vec![
        ['-', '-', '1', '-', '-'],
        ['-', '2', '3', '4', '-'],
        ['5', '6', '7', '8', '9'],
        ['-', 'A', 'B', 'C', '-'],
        ['-', '-', 'D', '-', '-'],
    ];

    let mut x: usize = 0;
    let mut y: usize = 2;
    let mut code: Vec<char> = Vec::new();
    let mut position: char = num_pad[y][x];

    directions.for_each(|line: &str| {
        line.chars().for_each(|c: char| {
            let is_button: bool;

            if c == 'L' && x > 0 {
                let new_x: usize = x - 1;
                is_button = num_pad[y][new_x] != '-';

                if is_button {
                    position = num_pad[y][new_x];
                    x = new_x;
                }
            } else if c == 'R' && x <= 3 {
                let new_x: usize = x + 1;
                is_button = num_pad[y][new_x] != '-';

                if is_button {
                    position = num_pad[y][new_x];
                    x = new_x;
                }
            } else if c == 'U' && y > 0 {
                let new_y: usize = y - 1;
                is_button = num_pad[new_y][x] != '-';

                if is_button {
                    position = num_pad[new_y][x];
                    y = new_y;
                }
            } else if c == 'D' && y <= 3 {
                let new_y: usize = y + 1;
                is_button = num_pad[new_y][x] != '-';

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
