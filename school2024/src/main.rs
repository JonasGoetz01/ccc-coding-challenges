// use std::env;
// use std::fs::File;
// use std::io::{BufRead, BufReader, Write};

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let input_file_path = args[1].to_owned();
//     let output_file_path = format!("{}.out", input_file_path);

//     let input_file = File::open(input_file_path).expect("Error opening input file");
//     let reader = BufReader::new(input_file);

//     // skip the first two line
//     let mut reader = reader.lines();
//     reader.next();
//     reader.next();

//     let mut coins = Vec::new();

//     let mut output_file =
//         File::create(output_file_path.clone()).expect("Error creating output file");

//     // loop over other lines
//     for line in reader {
//         let line = line.expect("Error reading line");
//         coins.clear();
//         for coin in line.split_whitespace() {
//             // add coin parsed to int to coins vector
//             coins.push(coin.parse::<i32>().expect("Error parsing coin"));
//         }
//         for value in 1..1000 {
//             if !coins.contains(&value) {
//                 writeln!(output_file, "{}", value).expect("Error writing to file");
//                 break;
//             }
//         }
//         // println!("{:?}", coins);
//     }
// }

// use std::env;
// use std::fs::File;
// use std::io::{BufRead, BufReader, Write};

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let input_file_path = args[1].to_owned();
//     let output_file_path = format!("{}.out", input_file_path);

//     let input_file = File::open(input_file_path).expect("Error opening input file");
//     let reader = BufReader::new(input_file);

//     // skip the first two line
//     let mut reader = reader.lines();
//     reader.next();
//     reader.next();
//     reader.next();

//     let mut coins = Vec::new();
//     let mut amounts = Vec::new();

//     let mut output_file =
//         File::create(output_file_path.clone()).expect("Error creating output file");

//     // loop over other lines
//     for line in reader {
//         let line = line.expect("Error reading line");
//         coins.clear();
//         for coin in line.split_whitespace() {
//             // add coin parsed to int to coins vector
//             coins.push(coin.parse::<i32>().expect("Error parsing coin"));
//         }
//         // get next line of file in the input file to the line variable
//         let line = reader
//             .next()
//             .expect("Error reading line")
//             .expect("Error reading line");

//         for amount in line.split_whitespace() {
//             // add coin parsed to int to coins vector
//             amounts.push(amount.parse::<i32>().expect("Error parsing coin"));
//         }
//         for amount in amounts.clone() {
//             for coin in coins {
//                 if amount - coin > 0 && coins.contains(&(amount - coin)) {
//                     println!("{} {}", coin, amount - coin);
//                     break;
//                 }
//             }
//         }
//         // println!("{:?}", coins);
//     }
// }

// use std::env;
// use std::fs::File;
// use std::io::{BufRead, BufReader, Write};

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let input_file_path = args[1].to_owned();
//     let output_file_path = format!("{}.out", input_file_path);

//     let input_file = File::open(&input_file_path).expect("Error opening input file");
//     let reader = BufReader::new(input_file);

//     // skip the first two lines
//     let mut reader = reader.lines().skip(2);

//     let mut coins = Vec::new();
//     let mut amounts = Vec::new();

//     let mut output_file =
//         File::create(output_file_path.clone()).expect("Error creating output file");

//     // loop over other lines
//     let iter = &mut reader;
//     let amount_num = iter
//         .next()
//         .expect("Error reading line")
//         .expect("Error reading line")
//         .parse::<i32>()
//         .expect("Error parsing amount");

//     while let Some(line) = iter.next() {
//         let line = line.expect("Error reading line");
//         coins.clear();
//         for coin in line.split_whitespace() {
//             coins.push(coin.parse::<i32>().expect("Error parsing coin"));
//         }
//         // get next line of file in the input file to the line variable
//         let line = iter
//             .next()
//             .expect("Error reading line")
//             .expect("Error reading line");

//         amounts.clear();
//         for amount in line.split_whitespace() {
//             amounts.push(amount.parse::<i32>().expect("Error parsing amount"));
//         }

//         let mut temp_amount = amount_num;
//         for amount in amounts.clone() {
//             println!("{}", amount);
//             for coin in &coins {
//                 if amount - coin > 0 && coins.contains(&(amount - coin)) {
//                     writeln!(output_file, "{} {}", coin, amount - coin);
//                     temp_amount -= 1;
//                     break;
//                 }
//             }
//             if temp_amount == 0 {
//                 break;
//             }
//         }
//         // writeln!(output_file, "\n");
//     }
// }

// use std::env;
// use std::fs::File;
// use std::io::{BufRead, BufReader, Write};

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let input_file_path = args[1].to_owned();
//     let output_file_path = format!("{}.out", input_file_path);

//     let input_file = File::open(&input_file_path).expect("Error opening input file");
//     let reader = BufReader::new(input_file);

//     // skip the first two lines
//     let mut reader = reader.lines().skip(2);

//     let mut coins = Vec::new();
//     let mut amounts = Vec::new();

//     let mut output_file =
//         File::create(output_file_path.clone()).expect("Error creating output file");

//     // loop over other lines
//     let iter = &mut reader;
//     let amount_num = iter
//         .next()
//         .expect("Error reading line")
//         .expect("Error reading line")
//         .parse::<i32>()
//         .expect("Error parsing amount");

//     while let Some(line) = iter.next() {
//         let line = line.expect("Error reading line");
//         coins.clear();
//         for coin in line.split_whitespace() {
//             coins.push(coin.parse::<i32>().expect("Error parsing coin"));
//         }
//         // get next line of file in the input file to the line variable
//         let line = iter
//             .next()
//             .expect("Error reading line")
//             .expect("Error reading line");

//         amounts.clear();
//         for amount in line.split_whitespace() {
//             amounts.push(amount.parse::<i32>().expect("Error parsing amount"));
//         }

//         let mut temp_amount = amount_num;
//         for amount in amounts.clone() {
//             println!("{}", amount);
//             for coin in &coins {
//                 if amount - coin > 0 && coins.contains(&(amount - coin)) {
//                     writeln!(output_file, "{} {}", coin, amount - coin);
//                     temp_amount -= 1;
//                     break;
//                 }
//             }
//             if temp_amount == 0 {
//                 break;
//             }
//         }
//         // writeln!(output_file, "\n");
//     }
// }

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_file_path = &args[1];
    let output_file_path = format!("{}.out", input_file_path);

    let input_file = File::open(input_file_path).expect("Error opening input file");
    let reader = BufReader::new(input_file);

    // skip the first two lines
    let mut iter = reader.lines().skip(2);

    let mut output_file =
        File::create(output_file_path.clone()).expect("Error creating output file");

    while let Some(line) = iter.next() {
        let line = line.expect("Error reading line");
        let coins: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Error parsing coin"))
            .collect();

        // Vector to store the fewest number of coins needed for each amount from 1 to 100
        let mut fewest_coins = vec![1000; 101]; // Initializing with a large number

        // Base case: no coins needed for amount 0
        fewest_coins[0] = 0;

        // Function to calculate the fewest number of coins needed for each amount using backtracking
        fn backtrack(coins: &[i32], amount: i32, fewest_coins: &mut [i32]) -> i32 {
            if amount < 0 {
                return 1000; // Infinity
            }
            if fewest_coins[amount as usize] != 1000 {
                return fewest_coins[amount as usize];
            }
            let mut min_coins = 1000; // Infinity
            for &coin in coins {
                min_coins = min_coins.min(backtrack(coins, amount - coin, fewest_coins) + 1);
            }
            fewest_coins[amount as usize] = min_coins;
            min_coins
        }

        // Calculate fewest coins for each amount
        for amount in 1..=100 {
            backtrack(&coins, amount, &mut fewest_coins);
        }

        // Writing the fewest number of coins needed for each amount to the output file
        for amount in 1..=100 {
            let mut remaining_amount = amount;
            while remaining_amount > 0 {
                for &coin in coins.iter().rev() {
                    if remaining_amount - coin >= 0
                        && fewest_coins[remaining_amount as usize]
                            == fewest_coins[(remaining_amount - coin) as usize] + 1
                    {
                        write!(output_file, "{}x{}", remaining_amount / coin, coin)
                            .expect("Error writing to file");
                        remaining_amount -= coin * (remaining_amount / coin);
                        if remaining_amount > 0 {
                            write!(output_file, " ").expect("Error writing to file");
                        }
                        break;
                    }
                }
            }
            writeln!(output_file, "").expect("Error writing to file");
        }
    }
}
