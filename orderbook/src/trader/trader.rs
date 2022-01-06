use crate::orderbook::order::MarketSide;
use crate::orderbook::order::Order;
use crate::orderbook::orderbook::OrderLine;
use std::collections::HashMap;

pub trait Trader {
    fn new() -> Self;

    fn create_order(&mut self) -> Order;
}

pub struct TraderStrcuture {}

impl Trader for TraderStrcuture {
    fn create_order(&mut self) -> Order {
        return Order {
            time: 1,
            price: 1,
            market_side: MarketSide::BUY,
            id: 1,
        };
    }

    fn new() -> Self {
        return TraderStrcuture {};
    }
}
