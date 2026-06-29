// src/domain.rs
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[allow(dead_code)] 
#[derive(Debug, PartialEq, Eq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OrderType {
    Market,
    Limit,
}

#[derive(Debug)]
pub struct Order {
    pub id: u64,
    pub symbol: String,
    pub side: Side,
    pub order_type: OrderType,
    pub price: Decimal,
    pub quantity: u32,
}   

const MAX_LIMIT_ERROR: &str = "Quantity exceeds max limit.";

pub fn validate_order(order: &Order) -> Result<(), String> {
    if order.quantity > 10_000 {
        return Err(format!("Order {} rejected: {}", order.id, MAX_LIMIT_ERROR));
    }

    if order.symbol.is_empty() {
        return Err(String::from("Rejected: Symbol cannot be empty."));
    }

    // Enforce Market Order rules
    if order.order_type == OrderType::Market && order.price != dec!(0.0) {
        return Err(format!("Order {} rejected: Market orders must have a price of 0.0.", order.id));
    }

    // Enforce Fat-Finger Risk Checks for Limit Orders
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