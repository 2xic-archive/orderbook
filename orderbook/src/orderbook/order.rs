

#[derive(Copy, Clone)]
pub struct Order {
    pub market_side: MarketSide,
    pub price:  u32,
    pub time: u32,
    pub id: u32,
}


#[derive(Copy, Clone, PartialEq)]
pub enum MarketSide {
    BUY,
    SELL,
}
