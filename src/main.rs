mod engine;

use engine::orderbook::{Order, BidOrAsk, Orderbook};;

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