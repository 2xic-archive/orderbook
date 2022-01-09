use crate::BasicExchange;
use crate::Exchange;
use crate::Mutex;

use cpython::{py_class, PyDict, PyObject, PyResult, Python};

pub mod communication {

    use crate::orderbook::orderbook::SimpleOrderLine;
use crate::EXCHANGE_REF;
    use cpython::PyList;

    use crate::cpython::PythonObject;
    use crate::cpython::ToPyObject;
    use crate::exchange::exchange::BasicExchange;
    use crate::orderbook::order::MarketSide;
    use crate::Order;
    use cpython::PyDict;
    use cpython::PyObject;
    use cpython::PyResult;
    use cpython::Python;

    py_class!(class MyType |py| {
        data number: i32;
        def __new__(_cls, arg: i32) -> PyResult<MyType> {
            MyType::create_instance(py, arg)
        }
        def half(&self) -> PyResult<i32> {
            println!("half() was called with self={:?}", self.number(py));
            Ok(self.number(py) / 2)
        }
    });

    pub fn create_buy_order(_py: Python, price: u8) -> PyResult<u8> {
        EXCHANGE_REF.lock().unwrap().add_order(Order {
            price,
            market_side: MarketSide::BUY,
            time: 0,
            id: 0,
        });

        Ok(1)
    }

    pub fn create_sell_order(_py: Python, price: u8) -> PyResult<u8> {
        EXCHANGE_REF.lock().unwrap().add_order(Order {
            price,
            market_side: MarketSide::SELL,
            time: 0,
            id: 0,
        });

        Ok(1)
    }

    pub fn get_order_count(_py: Python) -> PyResult<u8> {
        let size: u8 = EXCHANGE_REF.lock().unwrap().get_order_count();

        Ok(size)
    }

    pub fn get_buy_orderbook(_py: Python) -> PyResult<PyList> {
        let mut vectorrs: Vec<PyObject> = vec![];
        EXCHANGE_REF
            .lock()
            .unwrap()
            .get_buy_orderbook()
            .iter()
            .for_each(|a| {
                vectorrs.push(create_py_object_from_order(_py, a));
            });

        let list: PyList = PyList::new(_py, &vectorrs);
        Ok(list)
    }

    pub fn get_sell_orderbook(_py: Python) -> PyResult<PyList> {
        let mut vectorrs: Vec<PyObject> = vec![];
        EXCHANGE_REF
            .lock()
            .unwrap()
            .get_sell_orderbook()
            .iter()
            .for_each(|a| {
                vectorrs.push(create_py_object_from_order(_py, a));
            });

        let list: PyList = PyList::new(_py, &vectorrs);
        Ok(list)
    }

    fn create_py_object_from_order(_py: Python, order: &SimpleOrderLine) -> PyObject {
        let refd = PyDict::new(_py);
        refd.set_item(_py, "price", order.price).unwrap();
        refd.set_item(_py, "quantity", order.quantity).unwrap();

        let obj: cpython::PyObject = refd.into_object();
        return obj;
    }
}
