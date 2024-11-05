// Trading engine, first thing is the matching engine, so try to match the order in the order book if we place it on an exchage
enum BidorAsk{
    Bid,
    Ask
}

// Create a struct
Struct Order {
    size: f64,
    bid_or_ask: BidorAsk

}

// Constructor to add method to it
impl Order{
    fn new(bid_or_ask:BidorAsk,size: f64) -> Order{
        Order{bid_or_ask,size}
  }
}
