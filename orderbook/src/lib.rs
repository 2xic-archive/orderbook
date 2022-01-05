use crate::exchange::exchange::BasicExchange;

mod exchange;
mod orderbook;

#[cfg(test)]
mod tests {
    #[test]

    fn test_orderbook() {
        use crate::orderbook::order::MarketSide;
        use crate::orderbook::order::Order;
        use crate::orderbook::orderbook::BasicOrderBook;
        use crate::orderbook::orderbook::BsEOrderBook;

        let buy_order = Order {
            price: 1,
            time: 2,
            market_side: MarketSide::BUY,
        };

        let mut order_boook: BsEOrderBook = BasicOrderBook::new(buy_order);
        assert_eq!(order_boook.orders.len(), 1);
        let order = &order_boook.orders[0];
        assert_eq!(order.price, 1);

        let sell_order = Order {
            price: 1,
            time: 2,
            market_side: MarketSide::BUY,
        };
        order_boook.add_orders(sell_order);
        assert_eq!(order_boook.orders.len(), 2);
    }

    #[test]
    fn test_exchange() {
        use crate::exchange::exchange::Exchange;
        use crate::BasicExchange;

        let exchange: Exchange = BasicExchange::new();
        assert_eq!(exchange.orderbook.orders.len(), 1);
    }

    #[test]
    fn test_exchange_should_match_orders() {
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
        };
        let sell_order = Order {
            price: 1,
            time: 2,
            market_side: MarketSide::SELL,
        };

        exchange.orderbook.clear();
        exchange.orderbook.add_orders(buy_order);
        exchange.orderbook.add_orders(sell_order);
        exchange.orderbook.match_orders();

        assert_eq!(exchange.orderbook.orders.len(), 0);
    }
}
