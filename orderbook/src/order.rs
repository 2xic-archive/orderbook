

pub struct Order {
    pub market_side: MarketSide,
    pub price:  u8,
    pub time: u8,
}

pub enum MarketSide {
    BUY,
    SELL,
}
