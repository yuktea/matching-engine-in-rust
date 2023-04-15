use std::collections::HashMap;

#[derive(Debug)]
struct OrderBook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl OrderBook {
    fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    fn add_order(&mut self, price: f64, order: Order) {
        let price = Price::new(price);

        match order.bid_ask {
            BidOrAsk::Bid =>

                match self.bids.get_mut(&price) {
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


#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask,
}
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Price {
    integral: u64,
    fractional : u64,
    scalar : u64,
}

impl Price {
    fn new(price: f64) -> Price {
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            integral,
            scalar,
            fractional,
        }
    }
}

#[derive(Debug)]
struct Limit {
    price: Price,
    order: Vec<Order>,
}

impl Limit {
    fn new(price: Price) -> Limit {
        Limit {
            price,
            order: Vec::new(),
        }
    }

    fn add_order(&mut self, order: Order) {
        self.order.push(order);
    }
}

#[derive(Debug)]
struct Order {
    size: f64,
    bid_ask: BidOrAsk
}

impl Order {
    fn new(bid_ask: BidOrAsk, size: f64) -> Order {
        Order {
                    bid_ask,
                    size,
                }
    }
}

fn main() {
    let bid = Order::new(BidOrAsk::Bid, 5.0);
    let ask = Order::new(BidOrAsk::Ask, 2.0);




    let mut orderbook = OrderBook::new();
    orderbook.add_order(2.0, bid);
    orderbook.add_order(4.0, ask);

    let selling = Order::new(BidOrAsk::Ask, 6.5);
    orderbook.add_order(20.0, selling);
    println!("{:?}", orderbook); 


}