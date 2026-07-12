// src/main.rs
use std::io::{self, Write};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

// Declare the module so the compiler links the file
mod domain;

// Bring specific public items into our local namespace
use domain::{Order, OrderType, Side, validate_order, calculate_value};

fn main() -> io::Result<()> {
    println!("--- Fintech Risk Engine Shell (Modular) ---");
    println!("Type an order price to validate it against 'limit_order'.");
    println!("Press 'q' at any time to exit.\n");

    // Initialize our persistent state variable on the stack
    let mut running_buy_volume = dec!(0.0);

    loop {
        // Display the current cumulative value at the top of every loop turn
        println!("[Session Buy Volume: ${}]", running_buy_volume);
        print!("risk-engine > ");
        io::stdout().flush()?; // Safely bubbles up any terminal IO error using ?

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
                    Ok(()) => {
                        let order_value = calculate_value(&interactive_order);
                        running_buy_volume += order_value;

                        // 1. STEAL OWNERSHIP: This moves 'interactive_order' into a new variable name.
                        // Because Order contains a String (symbol), Rust deletes the original reference.
                        let order_reference = &interactive_order; 

                        // 2. THE LEGAL READ: We try to read the original variable AFTER it was moved.
                        println!("Success: Order {} passed risk validation.", order_reference.id);
                    }
                    Err(error_msg) => println!("Risk Alert -> {}", error_msg),
                }
            }
            Err(_) => {
                println!("Error: Please enter a valid numerical price or 'q' to quit.");
            }
        }

        println!(); 
    }
    
    Ok(())
}