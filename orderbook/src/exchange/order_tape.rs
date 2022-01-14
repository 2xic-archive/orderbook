use crate::orderbook::order::Order;

#[derive(Clone)]
pub struct OrderTape {
    pub orders_on_tape: Vec<Order>,
}

pub trait BasicOrderTape {
    fn new() -> Self;

    fn add_order(&mut self, order: Order);

    fn get_orders(&mut self) -> Order;
}

impl BasicOrderTape for OrderTape {
    fn new() -> OrderTape {
        return OrderTape {
            orders_on_tape: vec![],
        };
    }

    fn add_order(&mut self, order: Order) {
        self.orders_on_tape.push(order)
    }

    fn get_orders(&mut self) -> Order {
        return self.orders_on_tape.pop().unwrap();
    }
}
