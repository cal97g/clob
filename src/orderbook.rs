use rust_decimal::prelude::*;

const ORDER_BOOK_LEVELS: usize = 32;
const MIDDLE_LEVEL: u32 = 15;
const MINIMUM_PAD: usize = 3;

pub enum OrderbookUpdateType {
    SetLevel,
    LimitOrder,
    FillOrKillOrder,
    MarketOrder
}

pub const SET_LEVEL: OrderbookUpdateType  = OrderbookUpdateType::SetLevel;
pub const LIMIT_ORDER: OrderbookUpdateType  = OrderbookUpdateType::SetLevel;
pub const FILL_OR_KILL_ORDER: OrderbookUpdateType  = OrderbookUpdateType::SetLevel;
pub const MARKET_ORDER: OrderbookUpdateType  = OrderbookUpdateType::SetLevel;


#[derive(Default,Debug,Copy,Clone)]
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

pub struct OrderbookUpdate {
    price: Decimal,

    // should be negative if something is being removed from the book
    quantity: Decimal,

    order_type: OrderbookUpdateType,
}

enum Direction {
    Buy,
    Sell
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
    min_depth: u32,

    // min_step
    min_step: Decimal,

    levels: [Level; ORDER_BOOK_LEVELS],

    // the latest update id the book has applied
    update_id: u64,
}



impl Orderbook {
    pub fn new(best_bid_price: Decimal, best_ask_price: Decimal, min_step: Decimal) -> Orderbook {
        let mut newobject: Orderbook = Default::default();
        newobject.min_step = min_step;
        newobject.min_depth = 6;
        newobject.new_levels(best_bid_price, best_ask_price, min_step);
        newobject
    }

    pub fn rebalance(&mut self) -> bool {
        // I | P   | Q   | POINTER
        // 0 | 1.0 | 20  | 
        // 1 | 2.0 | 10  | BID
        // 2 | 3.0 |     | SPREAD > 1
        // 3 | 4.0 | 5   | ASK
        // 4 | 5.0 | 25  | 
        //
        // ask_depth = 5 - (3 + 1) = 1
        // bid_depth = 1

        let ask_depth = 32 - self.ask_pointer + 1;
        let bid_depth = self.bid_pointer;

        let mut rebal: bool = (ask_depth <= self.min_depth) | (bid_depth <= self.min_depth);
        rebal = true;

        if rebal {
            let old_prices = self.levels.clone();

            self.new_levels(
                self.levels[self.bid_pointer.to_usize().unwrap()].price,
                self.levels[self.ask_pointer.to_usize().unwrap()].price,
                self.min_step
            );

            self.apply_prices(old_prices);
            true
        } else {
            false
        }

    }

    fn apply_prices(&mut self, old_prices: [Level; ORDER_BOOK_LEVELS]) {
        // apply old price/quantity information to new levels
        // will be called after a rebalance, eg the book will have moved

        let previous_start_price = old_prices[0].price;
        let new_start_price = self.levels[0].price;

        // how many indexes we've moved up or down
        let new_relative_pos: i32 = (previous_start_price - new_start_price / self.min_step).to_i32().unwrap();

        if new_relative_pos > 0 {
            // price went up - new_relative_pos:32
            for (i, level) in self.levels.iter_mut().enumerate() {
                if i >= new_relative_pos - 1 {
                    let old_level = old_prices[i - new_relative_pos - 1];
                    println!("new: {} old: {}", level.price, old_level.price);

                } else {
                    continue
                }
            }

        } else {
            // price went down - 0:(32 - new_relative_pos)


        }
        
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
        let index = ((price_point - self.start_price) / self.min_step).to_usize().unwrap();
        match index {
            0..=ORDER_BOOK_LEVELS => Some(index),
            _ => None
        }
    }

    // what is the volume at this price?
    pub fn price_quantity(&self, price_point: Decimal) -> Decimal {
        self.levels[self.price_index(price_point).unwrap()].quantity
    }

    pub fn limit_order(&mut self, price: Decimal, quantity: Decimal) {
        self.levels[self.price_index(price).unwrap()].quantity += quantity;
    }

    pub fn update(&mut self, update_type: OrderbookUpdateType, update: OrderbookUpdate) -> bool {

        let level_index_opt = self.price_index(update.price);
        let level_idx: usize;

        match level_index_opt {
            Some(X) => {
                level_idx = level_index_opt.unwrap();
                true
            },
            None => return false,
        };

        let direction;
        if update.quantity > Decimal::zero() {
            direction = Direction::Buy;
        } else {
            direction = Direction::Sell;
        }
       

        match update.order_type {
            SET_LEVEL => {
                self.levels[level_idx].quantity = update.quantity;
            },
            LIMIT_ORDER => {
                self.levels[level_idx].quantity += update.quantity;
            },
            FILL_OR_KILL_ORDER => {


            },
            MARKET_ORDER => {
                let quantity = update.quantity;

            },
        }

        true
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
        
        let mut some_market = Orderbook::new(bid, ask, min_step);

        // println!("{:?}", some_market.levels);

        assert_eq!(some_market.bid_pointer, 12);
        assert_eq!(some_market.ask_pointer, 18);

        assert_eq!(some_market.spread_steps(), 6);
        assert_eq!(some_market.spread_cost(), Decimal::from_str("0.06").unwrap());

        assert_eq!(some_market.levels[0].price, Decimal::from_str("12299.88").unwrap());
        assert_eq!(some_market.start_price, Decimal::from_str("12299.88").unwrap());

        assert_eq!(some_market.price_index(Decimal::from_str("12299.89").unwrap()).unwrap(), 1);
        assert_eq!(some_market.price_index(Decimal::from_str("12300.17").unwrap()).unwrap(), 29);

        some_market.levels[29].quantity = Decimal::from_str("1234.5566778899").unwrap();
        assert_eq!(some_market.price_quantity(Decimal::from_str("12300.17").unwrap()), Decimal::from_str("1234.5566778899").unwrap());
        
        some_market.rebalance(); 
    
    }


    #[test]
    fn test_new_level() {
        let level = Level::new("10000.32", "0.00032800");
        assert_eq!(Decimal::from_f32(10000.32).unwrap(), level.price);
        assert_eq!(Decimal::from_f32(0.00032800).unwrap(), level.quantity);
    }
 

}