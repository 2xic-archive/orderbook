use crate::exchange::exchange::BasicExchange;
use crate::orderbook::order::Order;
use crate::orderbook::order::MarketSide;

#[macro_use]
extern crate cpython;

use crate::exchange::exchange::Exchange;
use cpython::{PyResult, Python};
use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref EXCHANGE_REF: Mutex<Exchange> =  Mutex::new(BasicExchange::new());
}

fn create_order(_py: Python) -> PyResult<u8> {
    EXCHANGE_REF.lock().unwrap().add_order(Order {
        price: 1,
        market_side: MarketSide::BUY,
        time: 0,
        id: 0,
    });

    Ok(1)
}

fn get_order_count(_py: Python) -> PyResult<u8> {
    let size: u8 = EXCHANGE_REF.lock().unwrap().get_order_count();

    Ok(size)
}

py_module_initializer!(
    liborderbooklib,
    initliborderbooklib,
    PyInit_liborderbooklib,
    |py, m| {
        m.add(py, "__doc__", "This module is implemented in Rust")?;
        m.add(py, "create_order", py_fn!(py, create_order()))?;
        m.add(py, "get_order_count", py_fn!(py, get_order_count()))?;

        Ok(())
    }
);

mod exchange;
mod orderbook;
mod trader;

#[cfg(test)]
mod tests {
    #[test]

    fn test_exchange_should_not_match_same_order_type() {
        use crate::exchange::exchange::Exchange;
        use crate::orderbook::order::MarketSide;
        use crate::orderbook::order::Order;
        use crate::orderbook::orderbook::BasicOrderBook;
        use crate::BasicExchange;

        let mut exchange: Exchange = BasicExchange::new();

        let buy_order = Order {
            price: 1,
            time: 2,
            market_side: MarketSide::BUY,
            id: 1,
        };
        let sell_order = Order {
            price: 1,
            time: 2,
            market_side: MarketSide::SELL,
            id: 2,
        };

        exchange.process(buy_order);
        assert_eq!(exchange.buy_orderbook.orders, 1);
        exchange.process(sell_order);

        assert_eq!(exchange.sell_orderbook.orders, 0);
        assert_eq!(exchange.buy_orderbook.orders, 0);
    }

    #[test]
    fn test_trader() {
        use crate::trader::trader::Trader;
        use crate::trader::trader::TraderStrcuture;

        let mut trader: TraderStrcuture = Trader::new();
    }
}
