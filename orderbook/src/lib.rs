mod order;
mod orderbook;

#[cfg(test)]
mod tests {
    #[test]

    fn test_orderbook() {
        use crate::order::MarketSide;
        use crate::order::Order;
        use crate::orderbook::BasicOrderBook;
        use crate::orderbook::BsEOrderBook;

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
}
