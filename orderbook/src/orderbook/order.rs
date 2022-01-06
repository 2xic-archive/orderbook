

#[derive(Copy, Clone)]
pub struct Order {
    pub market_side: MarketSide,
    pub price:  u8,
    pub time: u8,
    pub id: u8,
}


#[derive(Copy, Clone, PartialEq)]
pub enum MarketSide {
    BUY,
    SELL,
}
