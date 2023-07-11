mod matching_engine;
use matching_engine::orderbook::{BidOrAsk, Order, Orderbook};
fn main() {
    let buy_order = Order::new(BidOrAsk::Bid, 5.5);
    let buy2_order = Order::new(BidOrAsk::Bid, 55.5);
    let mut orderbook = Orderbook::new();
    orderbook.add_order(4.4, buy_order);
    orderbook.add_order(6.4, buy2_order);

    let sell_order = Order::new(BidOrAsk::Ask, 5.2);

    orderbook.add_order(20.0, sell_order);

    println!("{:?}", orderbook);
}
