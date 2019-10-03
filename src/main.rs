use std::collections::BTreeMap;

type Price = u32;
type Amount = u32;

enum OrderType {
    Bid,
    Ask,
}

#[derive(Debug)]
struct OrderBook {
    bids: BTreeMap<Price, Amount>,
    asks: BTreeMap<Price, Amount>,
}

impl OrderBook {
    fn new() -> Self {
        let bids: BTreeMap<Price, Amount> = BTreeMap::new();
        let asks: BTreeMap<Price, Amount> = BTreeMap::new();
        OrderBook { bids, asks }
    }

    fn is_empty(&self) -> bool {
        self.bids.is_empty() & self.asks.is_empty()
    }
    fn add(&mut self, price: Price, amount: Amount, order_type: OrderType) {
        match order_type {
            OrderType::Bid => {
                *self.bids.entry(price).or_insert(0) += amount;
            }
            OrderType::Ask => {
                *self.asks.entry(price).or_insert(0) += amount;
            }
        }
    }

    fn add_bid(&mut self, price: Price, amount: Amount) {
        self.add(price, amount, OrderType::Bid);
    }
    fn add_ask(&mut self, price: Price, amount: Amount) {
        self.add(price, amount, OrderType::Ask);
    }
}

fn main() {
    let mut orderbook = OrderBook::new();

    assert!(&orderbook.is_empty());

    println!("{:?}", orderbook);

    orderbook.add_ask(39, 100);
    orderbook.add_ask(50, 100);
    orderbook.add_ask(40, 100);

    orderbook.add_bid(35, 100);
    orderbook.add_bid(10, 100);
    orderbook.add_bid(37, 100);

    println!("{:?}", orderbook);

    println!("asks");
    for ask in orderbook.asks.iter().rev() {
        println!("{:?}", ask);
    }
    println!("bids");
    for bid in orderbook.bids.iter().rev() {
        println!("{:?}", bid);
    }
}
