// use std::env;
// use std::fs::File;
// use std::io::{self, BufRead, Write};

// fn main() -> io::Result<()> {
//     let args: Vec<String> = env::args().collect();
//     let input_file = &args[1];
//     let output_file = format!("{}.out", input_file);

//     let file = File::open(input_file)?;
//     let reader = io::BufReader::new(file);

//     let mut num_cells = 0;

//     for line in reader.lines() {
//         for c in line?.chars() {
//             if c == 'O' {
//                 num_cells += 1;
//             }
//         }
//     }

//     let mut output_file = File::create(output_file)?;
//     write!(output_file, "{}", num_cells)?;

//     Ok(())
// }

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn count_surrounding_os(matrix: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let mut count = 0;
    let directions: [(i32, i32); 8] = [
        (-1, -2),
        (-1, 0),
        (-1, 2),
        (0, -2),
        (0, 2),
        (1, -2),
        (1, 0),
        (1, 2),
    ];

    for &(dx, dy) in directions.iter() {
        let new_row = row as i32 + dx;
        let new_col = col as i32 + dy;

        if new_row >= 0
            && new_row < matrix.len() as i32
            && new_col >= 0
            && new_col < matrix[0].len() as i32
        {
            match matrix[new_row as usize][new_col as usize] {
                'O' => count += 1,
                'W' => count += 1,
                _ => (),
            }
        }
    }

    count
}

fn process_block(matrix: &Vec<Vec<char>>, output_file: &mut File) {
    for row in 0..matrix.len() {
        if matrix[row].is_empty() {
            continue; // Skip empty rows
        }
        for col in 0..matrix[row].len() {
            if matrix[row][col] == 'W' {
                let surrounding_os = count_surrounding_os(matrix, row, col);
                writeln!(output_file, "{}", surrounding_os).expect("Error writing to file");
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = args[1].to_owned();
    let output_file_path = format!("{}.out", input_file_path);

    let input_file = File::open(input_file_path).expect("Error opening input file");
    let reader = BufReader::new(input_file);

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error reading line");
        let row: Vec<char> = line.trim().chars().collect();
        matrix.push(row);
    }

    let mut output_file =
        File::create(output_file_path.clone()).expect("Error creating output file");

    for i in (0..matrix.len()).step_by(9) {
        let mut block: Vec<Vec<char>> = Vec::new();
        for j in 0..8 {
            if i + j < matrix.len() {
                block.push(matrix[i + j].clone());
            }
        }

        process_block(&block, &mut output_file);
    }

    println!("Output written to {}", output_file_path);
}
