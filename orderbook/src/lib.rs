pub mod exchange;
pub mod orderbook;
pub mod trader;
pub mod python;

use crate::exchange::exchange::Exchange;
use crate::trader::base_trader::TraderStrcuture;
use crate::python::communication::communication::step;
use crate::python::communication::communication::create_sell_order;
use crate::python::communication::communication::create_buy_order;
use crate::exchange::exchange::BasicExchange;
use crate::orderbook::order::Order;
use crate::python::communication::communication::get_buy_orderbook;
use crate::python::communication::communication::get_order_count;
use crate::python::communication::communication::get_sell_orderbook;

#[macro_use]
extern crate cpython;

use std::sync::Mutex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref EXCHANGE_REF: Mutex<Exchange> = Mutex::new(BasicExchange::new());
    static ref TRADES_REF: Mutex<Vec<TraderStrcuture>> = Mutex::new(vec![
        TraderStrcuture {
            orders: vec![],
            usd_balance: 100,
            balance: 0,
        }
    ]);
}

py_module_initializer!(
    liborderbooklib,
    initliborderbooklib,
    PyInit_liborderbooklib,
    |py, m| {
        m.add(py, "__doc__", "This module is implemented in Rust")?;
        m.add(py, "create_buy_order", py_fn!(py, create_buy_order(price: u32)))?;
        m.add(py, "create_sell_order", py_fn!(py, create_sell_order(price: u32)))?;
        m.add(py, "get_order_count", py_fn!(py, get_order_count()))?;
        m.add(py, "step", py_fn!(py, step()))?;
        m.add(py, "get_buy_orderbook", py_fn!(py, get_buy_orderbook()))?;
        m.add(py, "get_sell_orderbook", py_fn!(py, get_sell_orderbook()))?;

        Ok(())
    }
);

