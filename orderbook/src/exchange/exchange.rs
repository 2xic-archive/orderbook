use crate::orderbook::order::MarketSide;
use crate::orderbook::order::Order;
use crate::orderbook::orderbook::BasicOrderBook;
use crate::orderbook::orderbook::BsEOrderBook;

pub struct Exchange {
    pub buy_orderbook: BsEOrderBook,
    pub sell_orderbook: BsEOrderBook,
}

pub trait BasicExchange {
    fn new() -> Self;

    fn add_order(&mut self, order: Order);

    fn process(&mut self, order: Order);

    fn get_order_count(&mut self) -> u8;
}

impl BasicExchange for Exchange {
    fn new() -> Exchange {
        let buy_orderbook: BsEOrderBook = BasicOrderBook::new(MarketSide::BUY);
        let sell_orderbook: BsEOrderBook = BasicOrderBook::new(MarketSide::SELL);
        return Exchange { 
            buy_orderbook,
            sell_orderbook,
         };
    }

    fn get_order_count(&mut self) -> u8 {
        return self.buy_orderbook.orders;
    }

    fn add_order(&mut self, order: Order) {
        if order.market_side == MarketSide::BUY {
            self.buy_orderbook.add_orders(order);
        } else {
            self.sell_orderbook.add_orders(order);
        }
    }

     fn process(&mut self, order: Order) {
        self.add_order(order);

        if self.sell_orderbook.orders > 0 && self.buy_orderbook.orders > 0{
            if order.market_side == MarketSide::BUY {
                if self.sell_orderbook.best <= self.buy_orderbook.best {
                    self.sell_orderbook.remove_best();
                    self.buy_orderbook.remove_best();
                }
            } else {
                if self.sell_orderbook.best <= self.buy_orderbook.best {
                    self.sell_orderbook.remove_best();
                    self.buy_orderbook.remove_best();
                }
            }
        }
    }
}
