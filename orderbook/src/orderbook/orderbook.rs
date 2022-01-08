use crate::orderbook::order::MarketSide;
use crate::orderbook::order::Order;
use std::collections::HashMap;

pub struct OrderLine {
    pub orders: Vec<Order>,
    pub quantity: u8,
}

pub struct BsEOrderBook {
    order_layout: HashMap<u8, OrderLine>,

    depth: u8,

    pub best: u8,

    pub orders: u8,

    pub market_side: MarketSide,
}

pub trait BasicOrderBook {
    fn new(market_side: MarketSide) -> Self;

    fn add_orders(&mut self, order: Order);

    fn remove_best(&mut self);
}

impl BasicOrderBook for BsEOrderBook {
    fn new(market_side: MarketSide) -> BsEOrderBook {
        return BsEOrderBook {
            market_side,
            depth: 0,
            best: 0,
            orders: 0,
            order_layout: HashMap::new(),
        };
    }

    fn add_orders(&mut self, order: Order) {
        match self.order_layout.get_mut(&order.price) {
            Some(data) => {
                data.orders.push(order);
            }
            None => {
                let orderline = OrderLine {
                    orders: vec![order],
                    quantity: 1,
                };
                self.order_layout.insert(order.price, orderline);
                self.depth += 1;
            }
        }
        self.best = *self.order_layout.keys().max_by(|x, y| x.cmp(y)).unwrap();
        self.orders += 1;
    }

    fn remove_best(&mut self) {
        self.orders -= 1;
        self.best = *self.order_layout.keys().max_by(|x, y| x.cmp(y)).unwrap();

        match self.order_layout.get_mut(&self.best) {
            Some(data) => {
                data.orders.pop().unwrap();
                data.quantity -= 1;
                if data.quantity == 0 {
                    self.order_layout.remove(&self.best);
                }
            }
            None => {}
        }
    }
}
