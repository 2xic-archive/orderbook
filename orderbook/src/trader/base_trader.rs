use std::sync::MutexGuard;
use crate::BasicExchange;
use crate::Exchange;
use crate::trader::trader::Trader;
use crate::Order;


#[derive(Clone)]
pub struct TraderStrcuture {
    pub orders: Vec<Order>,

    pub usd_balance: u32,

    // sum of profits / loss of orders.
    pub balance: u32,
}

pub trait BaseTrader {
    fn try_to_create_order(&mut self, exchange: MutexGuard<Exchange>) -> u8;

    fn can_create_order(&mut self) -> bool;
}

impl<T> BaseTrader for T
where
    T: Trader,
{
    fn try_to_create_order(&mut self, mut exchange: MutexGuard<Exchange>) -> u8 {
        if self.can_create_order() {
            exchange.add_order(self.create_order())
        }

        return 0;
    }

    fn can_create_order(&mut self) -> bool {
        if self.get_balance() > 0 {
            return true;
        }
        return false;
    }
}
