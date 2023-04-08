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

    fn add_order(&mut self, order: Order) {
        match order.bid_ask {
            BidOrAsk::Ask => {
                let price = Price::from_price(order
            }
            BidOrAsk::Bid => {
                self.bids.insert(order.price, order);
            }
        }
    }
}


#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask,
}
#[derive(Debug)]
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
    fn new(price: f64) -> Limit {
        Limit {
            price: Price::new(price),
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
    let mut limit = Limit::new(65.3);
    let price = Price::new(90.5);
    let buy = Order::new(BidOrAsk::Bid, 5.1);
    let sell = Order::new(BidOrAsk::Ask, 2.4);

    limit.add_order(buy);
    limit.add_order(sell);

    println!("{:?}", limit); 
}