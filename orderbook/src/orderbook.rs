use crate::order::Order;

pub struct BsEOrderBook {
    pub orders: Vec<Order>,
}

pub trait BasicOrderBook {
    fn new(order: Order) -> Self;

    fn add_orders(&mut self, order: Order);
}

impl BasicOrderBook for BsEOrderBook {
    fn new(order: Order) -> BsEOrderBook {
        let orders = vec![order];
        return BsEOrderBook { orders };
    }

    fn add_orders(&mut self, order: Order) {
        self.orders.push(order);
    }
}
