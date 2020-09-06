use rust_decimal::prelude::*;

const ORDER_BOOK_LEVELS: u8 = 31;

pub struct Level {

    // price in sats or cents
    price: Decimal,

    // quantity in sats
    quantity: Decimal,
}

impl Level {
    fn new(price: &str, quantity: &str) -> Level {
        Level {
            price: Decimal::from_str(price),
            quantity: Decimal::from_str(quantity)
        }
        
    }
}

#[derive(Serialize, Deserialize)]
pub struct Orderbook {
    levels: [Level; 32],

    // the idx of the current best bid (refers to levels array)
    current_bid_index: u8,

    // the idx of the current best ask (refers to levels array)
    current_ask_index: u8,

    // the minimum depth which must be kept either side of bid/ask
    min_depth: u8,

    // min_step
    min_step: u16,

}

impl Orderbook {
    pub fn new(min_step: u32, current_bid_price: Decimal, current_ask_price: Decimal) -> Orderbook {

        let fair_mid_price = (current_ask_price + current_bid_price) / 2;







        let ob = Orderbook{};
    }

    fn rebuild(&'a mut self) -> bool {

    }

    fn new_levels(&self) {
        // find a midpoint to the nearest step: (best_ask_price - best_bid_price) / 2
        // the ask lives above the midpoint: (ask_price - mid_price) / step levels
        // the bid lives below the midpoint: (bid_rpice - mid_rpice / step) levels (should be neg sign)
        // all over prices points filled in
    }   

}


#[cfg(test)]
mod test {
   #test
   fn test_new_level() {
       let mut level = Level{price: "10000.32", quantity: "0.00032800"}
   }

    #test
    fn test_orderbook_create() {
        Orderbook::new(*"btcusd", 100, Level{price: 10000 * 100, quantity: 1.2 * 8})
    }
}