use crate::orderbook::order::MarketSide;
use crate::orderbook::order::Order;
use crate::trader::base_trader::TraderStrcuture;

pub trait Trader {
    fn create_order(&mut self) -> Order;

    fn get_balance(&mut self) -> u32;

    fn get_market_side(&mut self) -> MarketSide;
}

impl Trader for TraderStrcuture {
    fn create_order(&mut self) -> Order {
        let order = Order {
            time: 1,
            price: 1,
            market_side: self.get_market_side(),
            id: 1,
        };

        return order;
    }

    fn get_market_side(&mut self) -> MarketSide {
        return MarketSide::BUY;
    }

    fn get_balance(&mut self) -> u32 {
        return self.usd_balance;
    }
}
