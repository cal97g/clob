use rust_decimal::prelude::*;

const ORDER_BOOK_LEVELS: usize = 32;
const MIDDLE_LEVEL: u32 = 15;
const MINIMUM_PAD: usize = 3;

#[derive(Default,Debug)]
pub struct Level {
    // price in sats or cents
    price: Decimal,
    // quantity in sats
    quantity: Decimal,
}

impl Level {
    fn new(price: &str, quantity: &str) -> Level {
        Level {
            price: Decimal::from_str(price).unwrap(),
            quantity: Decimal::from_str(quantity).unwrap()
        }
        
    }
}


#[derive(Default)]
pub struct Orderbook {
    // the price at levels[0]
    start_price: Decimal,

    // the idx of the current best bid (refers to levels array)
    bid_pointer: u32,

    // the idx of the current best ask (refers to levels array)
    ask_pointer: u32,
    
    // the minimum depth which must be kept either side of bid/ask
    min_depth: u8,

    // min_step
    min_step: Decimal,

    levels: [Level; ORDER_BOOK_LEVELS],
}

impl Orderbook {
    pub fn new(best_bid_price: Decimal, best_ask_price: Decimal, min_step: Decimal) -> Orderbook {
        let mut newobject: Orderbook = Default::default();
        newobject.min_step = min_step;
        newobject.new_levels(best_bid_price, best_ask_price, min_step);
        newobject
    }

    fn rebuild(&mut self) -> bool {
        true
    }

    fn new_levels(&mut self, best_bid_price: Decimal, best_ask_price: Decimal, min_step: Decimal) {
        // find a midpoint to the nearest step: (best_ask_price - best_bid_price) / 2
        // the ask lives above the midpoint: (ask_price - mid_price) / step levels
        // the bid lives below the midpoint: (bid_price - mid_rpice / step) levels (should be neg sign)
        // all over prices points filled in

        // this can never be a float
        let spread_steps = ((best_ask_price - best_bid_price) / min_step).to_u32().unwrap();

        let spread_div_rem = spread_steps % 2;

        self.ask_pointer = MIDDLE_LEVEL + (spread_steps / 2);
        self.bid_pointer = MIDDLE_LEVEL - (spread_steps / 2) - spread_div_rem;

        // 0 should represent the lowest price level we support, while 31 should represent the highest.
        let start_price: Decimal = (best_bid_price / min_step) - Decimal::from_u32(self.bid_pointer).unwrap() * min_step;
        self.start_price = start_price.clone();

        for (i, level) in self.levels.iter_mut().enumerate() {
            level.price = start_price + (min_step * Decimal::from_usize(i).unwrap());
            level.quantity = Decimal::from_u32(0).unwrap();
        }
    }

    pub fn spread_steps(&self) -> u32 {
        self.ask_pointer - self.bid_pointer
    }

    pub fn spread_cost(&self) -> Decimal {
        Decimal::from_usize((self.ask_pointer - self.bid_pointer).to_usize().unwrap()).unwrap() * self.min_step
    }

    // where can I find the information for this price?
    pub fn price_index(&self, price_point: Decimal) -> Option<usize> {
        let index = ((self.start_price - price_point) * self.min_step).to_usize().unwrap();


        match index {
            0..=ORDER_BOOK_LEVELS => Some(index),
            _ => None
        }
    }

    // what is the volume at this price?
    pub fn price_quantity(&self, price_point: Decimal) -> Decimal {
        self.levels[self.price_index(price_point).unwrap()].quantity
    }
}


#[cfg(test)]
mod test {
    use super::*;
    


    #[test]
    fn test_Orderbook_create() {
        let bid = Decimal::from_f32(123.00).unwrap();
        let ask = Decimal::from_f32(123.06).unwrap();
        let min_step = Decimal::from_str("0.01").unwrap();
        
        let some_market = Orderbook::new(bid, ask, min_step);

        // println!("{:?}", some_market.levels);

        assert_eq!(some_market.bid_pointer, 12);
        assert_eq!(some_market.ask_pointer, 18);

        assert_eq!(some_market.price_index(Decimal::from_str("12299.89").unwrap()).unwrap(), 1);
        assert_eq!(some_market.price_index(Decimal::from_str("12300.17").unwrap()).unwrap(), 29);
    }

    #[test]
    fn test_Orderbook_spread_steps() {

    }

    #[test]
    fn test_Orderbook_spread_cost() {

    }


    #[test]
    fn test_new_level() {
        let level = Level::new("10000.32", "0.00032800");
        assert_eq!(Decimal::from_f32(10000.32).unwrap(), level.price);
        assert_eq!(Decimal::from_f32(0.00032800).unwrap(), level.quantity);
    }
 

}