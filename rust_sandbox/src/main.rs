use std::io::{self, Write};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[allow(dead_code)] // We don't use Sell in this example, but it's here for completeness
#[derive(Debug, PartialEq, Eq)]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq, Eq)]
enum OrderType {
    Market,
    Limit,
}

#[derive(Debug)]
struct Order {
    id: u64,
    symbol: String,
    side: Side,
    order_type: OrderType,
    price: Decimal,
    quantity: u32,
}   

const MAX_LIMIT_ERROR: &str = "Quantity exceeds max limit.";

fn validate_order(order: &Order) -> Result<(), String> {
    if order.quantity > 10_000 {
        return Err(format!("Order {} rejected: {}", order.id, MAX_LIMIT_ERROR));
    }

    if order.symbol.is_empty() {
        return Err(String::from("Rejected: Symbol cannot be empty."));
    }

    // 1. Enforce Market Order rules (Price must be exactly 0)
    if order.order_type == OrderType::Market && order.price != dec!(0.0) {
        return Err(format!("Order {} rejected: Market orders must have a price of 0.0.", order.id));
    }

    // 2. Enforce Fat-Finger Risk Checks for Limit Orders
    match order.side {
        Side::Buy if order.price > dec!(1000.0) => {
            Err(format!("Order {} rejected: Buy limit price too high.", order.id))
        }
        Side::Sell if order.price < dec!(0.01) => {
            Err(format!("Order {} rejected: Sell limit price too low.", order.id))
        }
        _ => Ok(()),
    }
}

fn main() {
    println!("--- Fintech Risk Engine Shell ---");
    println!("Type an order price to validate it against 'limit_order'.");
    println!("Press 'q' at any time to exit.\n");

    loop {
        // 1. Print the shell prompt symbol and force it to display instantly
        print!("risk-engine > ");
        io::stdout().flush().unwrap();

        // 2. Allocate a clean mutable string on the heap for user input
        let mut input = String::new();

        // 3. Capture keystrokes from standard input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // 4. Strip trailing newlines (\n or \r\n)
        let trimmed = input.trim();

        // 5. Check for exit instruction
        if trimmed == "q" || trimmed == "Q" {
            println!("Exiting risk engine. Goodbye!");
            break; 
        }

        // 6. Safely attempt to convert the string token into a Decimal
        match trimmed.parse::<Decimal>() {
            Ok(parsed_price) => {
                // Construct a temporary order evaluating their custom input price
                let interactive_order = Order {
                    id: 1001,
                    symbol: String::from("AAPL"),
                    side: Side::Buy,
                    price: parsed_price, 
                    order_type: OrderType::Limit,
                    quantity: 500, // Small value to intentionally bypass the 10k quantity check
                };

                // Run the validation rule suite
                match validate_order(&interactive_order) {
                    Ok(()) => println!("Success: Order {} passed risk validation.", interactive_order.id),
                    Err(error_msg) => println!("Risk Alert -> {}", error_msg),
                }
            }
            Err(_) => {
                println!("Error: Please enter a valid numerical price or 'q' to quit.");
            }
        }

        println!(); // Spacing layout layout for readability
    }
}