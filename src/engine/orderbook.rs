use std::collections::HashMap;

#[derive(Debug)]
pub enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
pub struct Orderbook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        match market_order.bid_or_ask {
            BidOrAsk::Bid => {
                for limit_order in self.ask_limits() {
                    limit_order.fill_order(market_order);

                    if market_order.is_filled() {
                        break;
                    }
                }
            }
            BidOrAsk::Ask => {}
        }
    }

    pub fn ask_limits(&mut self) -> Vec<&mut Limit> {
        self.asks.values_mut().collect::<Vec<&mut Limit>>()
    }

    pub fn bid_limits(&mut self) -> Vec<&mut Limit> {
        self.bids.values_mut().collect::<Vec<&mut Limit>>()
    }

    pub fn add_order(&mut self, price: f64, order: Order) {
        let price = Price::new(price);

        match order.bid_or_ask {
            BidOrAsk::Bid => match self.bids.get_mut(&price) {
                Some(limit) => limit.add_order(order),
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.bids.insert(price, limit);
                }
            },
            BidOrAsk::Ask => match self.asks.get_mut(&price) {
                Some(limit) => limit.add_order(order),
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.asks.insert(price, limit);
                }
            },
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    pub fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            scalar,
            integral,
            fractional,
        }
    }
}

#[derive(Debug)]
pub struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    pub fn new(price: Price) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    fn total_volume(&self) -> f64 {
        self.orders
            .iter()
            .map(|order| order.size)
            .reduce(|a, b| a + b)
            .unwrap()
    }

    fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0
                }
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0
                }
            }

            if market_order.is_filled() {
                break;
            }
        }
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

#[derive(Debug)]
pub struct Order {
    size: f64,
    bid_or_ask: BidOrAsk,
}

impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { bid_or_ask, size }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}

    }
}