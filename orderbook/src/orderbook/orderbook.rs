use crate::orderbook::order::Order;
use std::collections::HashMap;

pub struct BsEOrderBook {
    pub orders: Vec<Order>,
}

pub trait BasicOrderBook {
    fn new(order: Order) -> Self;

    fn add_orders(&mut self, order: Order);

    fn match_orders(&mut self);

    fn clear(&mut self);
}

impl BasicOrderBook for BsEOrderBook {
    fn new(order: Order) -> BsEOrderBook {
        let orders = vec![order];
        return BsEOrderBook { orders };
    }

    fn add_orders(&mut self, order: Order) {
        self.orders.push(order);
    }

    fn match_orders(&mut self) {
        let mut orders: HashMap<u8, Vec<Order>> = HashMap::new();
        for order in &self.orders {
            match orders.get_mut(&order.price) {
                Some(data) => {
                    data.push(*order);
                    if data.len() == 2 {
                        data.clear();
                    }
                }
                None => {
                    orders.insert(order.price, vec![*order]);
                }
            }
        }
        self.orders.clear();
        for price_orders in orders.values() {
            for order in price_orders {
                self.orders.push(*order);
            }
        }
    }

    fn clear(&mut self) { 
        self.orders.clear();
    }
}
