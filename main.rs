#[derive(Debug)]
enum BidOrAsk {
    Bid,
    Ask,
}
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

fn main() {
    println!("first commit");
}