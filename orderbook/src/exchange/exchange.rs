use crate::exchange::order_tape::BasicOrderTape;
use crate::exchange::order_tape::OrderTape;
use crate::orderbook::order::MarketSide;
use crate::orderbook::orderbook::BasicOrderBook;
use crate::orderbook::orderbook::BsEOrderBook;
use crate::orderbook::orderbook::SimpleOrderLine;
use crate::trader::base_trader::TraderStrcuture;
use crate::Order;

extern crate rand;
use rand::{thread_rng, Rng};

#[derive(Clone)]
pub struct Exchange {
    pub buy_orderbook: BsEOrderBook,
    pub sell_orderbook: BsEOrderBook,
    pub traders: Vec<TraderStrcuture>,
    pub order_tape: OrderTape,
}

pub trait BasicExchange {
    fn new() -> Self;

    fn add_order(&mut self, order: Order);

    fn process(&mut self, order: Order);

    fn get_order_count(&mut self) -> u32;

    fn get_buy_orderbook(&mut self) -> Vec<SimpleOrderLine>;

    fn get_sell_orderbook(&mut self) -> Vec<SimpleOrderLine>;

    fn empty_tape(&mut self) -> u32;

    fn can_prcocess_order(&mut self) -> bool;
}

impl Exchange {
    fn add_order(&mut self, order: Order) {
        if order.market_side == MarketSide::BUY {
            self.buy_orderbook.add_orders(order);
        } else {
            self.sell_orderbook.add_orders(order);
        }
    }
}

impl BasicExchange for Exchange {
    fn new() -> Exchange {
        let buy_orderbook: BsEOrderBook = BasicOrderBook::new(MarketSide::BUY);
        let sell_orderbook: BsEOrderBook = BasicOrderBook::new(MarketSide::SELL);
        let trader: TraderStrcuture = TraderStrcuture {
            orders: vec![],
            usd_balance: 100,
            balance: 0,
        };

        return Exchange {
            buy_orderbook,
            sell_orderbook,
            traders: vec![trader],
            order_tape: BasicOrderTape::new(),
        };
    }

    fn get_order_count(&mut self) -> u32 {
        return self.buy_orderbook.orders;
    }

    fn get_buy_orderbook(&mut self) -> Vec<SimpleOrderLine> {
        return self.buy_orderbook.get_orders();
    }

    fn get_sell_orderbook(&mut self) -> Vec<SimpleOrderLine> {
        return self.sell_orderbook.get_orders();
    }

    fn add_order(&mut self, order: Order) {
        self.order_tape.add_order(order);
    }

    fn process(&mut self, order: Order) {
        self.add_order(order);

        if self.sell_orderbook.orders > 0 && self.buy_orderbook.orders > 0 {
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

    fn empty_tape(&mut self) -> u32 {
        while 0 < self.order_tape.orders_on_tape.len() {
            let order = self.order_tape.get_orders();
            if self.can_prcocess_order() {
                self.process(order);
            }
        }

        return 1;
    }
    
    fn can_prcocess_order(&mut self) -> bool { 
        let mut rng = thread_rng();
        let x: f64 = rng.gen();
        let is_network_error = x < 0.01;

        if is_network_error {
            return false;
        }

        return true;
    }
}
