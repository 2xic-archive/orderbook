extern crate orderbooklib;

#[cfg(test)]
mod tests {
    use crate::orderbooklib::exchange::exchange::BasicExchange;
    use crate::orderbooklib::exchange::exchange::Exchange;
    use crate::orderbooklib::orderbook::order::Order;
    use crate::orderbooklib::orderbook::order::MarketSide;

    #[test]
    fn test_exchange_should_not_match_same_order_type() {

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
}
