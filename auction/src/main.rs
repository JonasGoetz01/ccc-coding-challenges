use std::env;

fn main() {
    let second_arg = env::args().nth(1).expect("No second argument provided");
    let input: Vec<&str> = second_arg.split(",").collect();

    let start_amount: i32 = input[0].parse().expect("Invalid start amount");

    let mut current_price = start_amount;
    let mut highest_bidder = String::new();
    let mut highest_bid = start_amount;

    let mut i = 1;
    while i < input.len() {
        let auctor = String::from(input[i]);
        let amount = input[i + 1].parse().expect("Invalid bet amount");

        if amount > highest_bid {
            highest_bid = amount;
            highest_bidder = auctor.clone();
        }

        if amount > current_price {
            current_price = current_price + 1;
            highest_bidder = auctor.clone();
            highest_bid = amount;
        }

        i += 2;
    }

    println!("{},{}", highest_bidder, highest_bid);
}
