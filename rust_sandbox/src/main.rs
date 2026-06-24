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

fn validate_order(order: &Order) -> Result<(), String> {
    if order.quantity > 10_000 {
        return Err(format!("Order {} rejected: Quantity exceeds max limit.", order.id));
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
    let limit_order = Order {
        id: 1001,
        symbol: String::from("AAPL"),
        side: Side::Buy,
        price: dec!(1050.0), // This should trigger our fat-finger check
        order_type: OrderType::Limit,
        quantity: 500,
    };

    println!("Validating order: {:?}", limit_order);

    // Handle the Result explicitly
    match validate_order(&limit_order) {
        Ok(()) => println!("Order {} passed risk validation.", limit_order.id),
        Err(error_msg) => println!("Risk Alert -> {}", error_msg),
    }
}