use crate::orderbook::order::MarketSide;
use crate::orderbook::order::Order;
use crate::orderbook::orderbook::BasicOrderBook;
use crate::orderbook::orderbook::BsEOrderBook;

pub struct Exchange {
    pub orderbook: BsEOrderBook,
}

pub trait BasicExchange {
    fn new() -> Self;
}

impl BasicExchange for Exchange {
    fn new() -> Exchange {
        let buy_order = Order {
            price: 1,
            time: 2,
            market_side: MarketSide::BUY,
        };
        let orderbook: BsEOrderBook = BasicOrderBook::new(buy_order);
        return Exchange { orderbook };
    }
}
