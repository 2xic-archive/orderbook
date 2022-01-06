
use crate::exchange::exchange::BasicExchange;

#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

fn hello(_py: Python, val: &str) -> PyResult<u64> {

    print!("hello");
    print!("{}", val);

    Ok(1)
}

py_module_initializer!(liborderbooklib, initliborderbooklib, PyInit_liborderbooklib, |py, m | {
    m.add(py, "__doc__", "This module is implemented in Rust")?;
    m.add(py, "hello", py_fn!(py, hello(val: &str)))?;
    Ok(())
});

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
