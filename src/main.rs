use std::collections::BTreeMap;
use std::fmt;

type Price = u32;
type Size = u32;

enum OrderType {
    Bid,
    Ask,
}

#[derive(Debug)]
struct OrderBook {
    bids: BTreeMap<Price, Size>,
    asks: BTreeMap<Price, Size>,
}

impl OrderBook {
    fn new() -> Self {
        let bids: BTreeMap<Price, Size> = BTreeMap::new();
        let asks: BTreeMap<Price, Size> = BTreeMap::new();
        OrderBook { bids, asks }
    }

    fn is_empty(&self) -> bool {
        self.bids.is_empty() & self.asks.is_empty()
    }
    fn add(&mut self, price: Price, amount: Size, order_type: OrderType) {
        match order_type {
            OrderType::Bid => {
                *self.bids.entry(price).or_insert(0) += amount;
            }
            OrderType::Ask => {
                *self.asks.entry(price).or_insert(0) += amount;
            }
        }
    }

    fn add_bid(&mut self, price: Price, amount: Size) {
        self.add(price, amount, OrderType::Bid);
    }
    fn add_ask(&mut self, price: Price, amount: Size) {
        self.add(price, amount, OrderType::Ask);
    }
}

impl fmt::Display for OrderBook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "Orderbook empty")
        } else {
            write!(f, "\tASK\n\n").unwrap();
            write!(f, "PRICE\tAMOUNT\n").unwrap();

            for (price, amount) in self.asks.iter().rev() {
                write!(f, "{}\t{}\n", price, amount).unwrap();
            }
            write!(f, "-----------------\n").unwrap();

            for (price, amount) in self.bids.iter().rev() {
                write!(f, "{}\t{}\n", price, amount).unwrap();
            }
            write!(f, "\n\tBID\n")
        }
    }
}

fn main() {
    let mut orderbook = OrderBook::new();

    assert!(&orderbook.is_empty());

    //println!("{}", orderbook);

    orderbook.add_ask(39, 100);
    orderbook.add_ask(50, 100);
    orderbook.add_ask(40, 100);

    orderbook.add_bid(35, 100);
    orderbook.add_bid(10, 100);
    orderbook.add_bid(37, 100);

    println!("{}", orderbook);
}
