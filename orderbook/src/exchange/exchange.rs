use crate::trader::base_trader::TraderStrcuture;
use crate::orderbook::order::MarketSide;
use crate::orderbook::orderbook::SimpleOrderLine;
use crate::Order;
use crate::orderbook::orderbook::BasicOrderBook;
use crate::orderbook::orderbook::BsEOrderBook;

#[derive(Clone)]
pub struct Exchange {
    pub buy_orderbook: BsEOrderBook,
    pub sell_orderbook: BsEOrderBook,
    pub traders: Vec<TraderStrcuture>,
}

pub trait BasicExchange {
    fn new() -> Self;

    fn add_order(&mut self, order: Order);

    fn process(&mut self, order: Order);

    fn get_order_count(&mut self) -> u32;

    fn get_buy_orderbook(&mut self) -> Vec<SimpleOrderLine>;

    fn get_sell_orderbook(&mut self) -> Vec<SimpleOrderLine>;
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
    fn add_order(&mut self, order: Order) {
        self.add_order(order);
    }

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
}
