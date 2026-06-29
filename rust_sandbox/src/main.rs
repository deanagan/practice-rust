// src/main.rs
use std::io::{self, Write};
use rust_decimal::Decimal;

// 1. Declare the module so the compiler links the file
mod domain;

// 2. Bring the specific public items into our local namespace
use domain::{Order, OrderType, Side, validate_order};

fn main() {
    println!("--- Fintech Risk Engine Shell (Modular) ---");
    println!("Type an order price to validate it against 'limit_order'.");
    println!("Press 'q' at any time to exit.\n");

    loop {
        print!("risk-engine > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed = input.trim();

        if trimmed == "q" || trimmed == "Q" {
            println!("Exiting risk engine. Goodbye!");
            break; 
        }

        match trimmed.parse::<Decimal>() {
            Ok(parsed_price) => {
                // Construct the order using our imported domain types
                let interactive_order = Order {
                    id: 1001,
                    symbol: String::from("AAPL"),
                    side: Side::Buy,
                    price: parsed_price, 
                    order_type: OrderType::Limit,
                    quantity: 500, 
                };

                // Execute the imported validation module function
                match validate_order(&interactive_order) {
                    Ok(()) => println!("Success: Order {} passed risk validation.", interactive_order.id),
                    Err(error_msg) => println!("Risk Alert -> {}", error_msg),
                }
            }
            Err(_) => {
                println!("Error: Please enter a valid numerical price or 'q' to quit.");
            }
        }

        println!(); 
    }
}