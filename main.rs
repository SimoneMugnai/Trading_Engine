// Trading engine, first thing is the matching engine, so try to match the order in the order book if we place it on an exchage
#[derive(Debug)]
enum BidorAsk{
    Bid,
    Ask
}

// Create a struct
#[derive(Debug)]
struct Order {
    size: f64,
    bid_or_ask: BidorAsk
}

// Constructor to add method to it
impl Order{
    fn new(bid_or_ask:BidorAsk,size: f64) -> Order{
        Order{bid_or_ask,size}
  }
}

//I want to put it in an hashmap, so f64 is not great for inconsistent values.
//So our own price implementation
#[derive(Debug)]
struct Price{
    integral: u64,
    fractional: u64,
    scalar: u64

}

impl Price{
    fn new(price: f64) -> Price{
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            scalar,
            integral,
            fractional

        }

    }
}

#[derive(Debug)]
struct Limit{
    price: Price,
    orders:Vec<Order>
}

impl  Limit {
    fn new(price: f64) -> Limit{
        Limit{
            price:Price::new(price),
            orders: Vec::new()
        }

    }
}

fn main(){
    let limit = Limit::new(78.3);
    println!("{:?}",limit);
}