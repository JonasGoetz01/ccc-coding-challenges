use diff::lines;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Write};

pub fn check_example(level: &str) {
    let file2_content = fs::read_to_string(format!(
        "input/level{}/level{}_example.me.out",
        level, level
    ))
    .expect("Unable to read my file");
    let file1_content =
        fs::read_to_string(format!("input/level{}/level{}_example.out", level, level))
            .expect("Unable to read expected file");
    let diff = lines(&file2_content, &file1_content);
    if diff.is_empty() {
        println!("OK");
    } else {
        let mut num_diff = 0;
        for d in &diff {
            match d {
                diff::Result::Left(l) => {
                    num_diff += 1;
                    println!("- {}", l)
                }
                diff::Result::Both(_, _) => {} // Ignoring lines that are the same
                diff::Result::Right(r) => {
                    num_diff += 1;
                    println!("+ {}", r)
                }
            }
        }
        println!("{} differences found", num_diff);
    }
}

fn main() {
    let level = "1";
    let input_files = vec![
        format!("input/level{}/level{}_example.in", level, level),
        format!("input/level{}/level{}_1.in", level, level),
        format!("input/level{}/level{}_2.in", level, level),
        format!("input/level{}/level{}_3.in", level, level),
        format!("input/level{}/level{}_4.in", level, level),
        format!("input/level{}/level{}_5.in", level, level),
    ];
    for input_file in input_files {
        println!("Processing file: {}", input_file);
        let reader =
            BufReader::new(File::open(input_file.clone()).expect("Error opening input file"));
        let reader = reader.lines();
        let output_file_name;
        if input_file.ends_with("example.in") {
            println!("Skipping example file");
            output_file_name = input_file.to_string().replace(".in", ".me.out");
        } else {
            output_file_name = input_file.to_string().replace(".in", ".out");
        }
        let mut output_file =
            File::create(output_file_name.clone()).expect("Error creating output file");
        //let mut reader = reader.skip(1);

        // do the main logic here
        for line in reader {
            println!("{}", line.expect("Error reading line"));
        }

        writeln!(output_file, "Hello, world!").expect("Error writing to output file");
        println!("Output written to: {}", output_file_name.clone());
    }
    check_example(level);
}
