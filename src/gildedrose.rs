use std::fmt::{self, Display};

pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for i in 0..self.items.len() {
            self.items[i].quality = self.calculate_quality(&self.items[i]);
            self.items[i].sell_in = self.calculate_sell_in(&self.items[i]);
        }
    }

    fn calculate_sell_in(&self, item: &Item) -> i32 {
        if item.name.contains("Sulfuras") {
            item.sell_in
        } else {
            item.sell_in - 1
        }

    }

    fn calculate_quality(&self, item: &Item) -> i32 {
        if item.name == "Aged Brie"
        {
            (item.quality - GildedRose::calculate_item_quality_increment(item.sell_in)).min(50)
        } else if item.name.contains("Backstage passes") {
            (item.quality + GildedRose::calculate_backstage_pass_quality_increment(item.sell_in, item.quality)).min(50)
        } else if item.name.contains("Sulfuras") {
            80// NOOP }
        } else {
            (item.quality + GildedRose::calculate_item_quality_increment(item.sell_in)).max(0)
        }
    }

    fn calculate_backstage_pass_quality_increment(sell_in: i32, quality: i32) -> i32 {
        if sell_in < 11 && sell_in > 5 {
            2
        } else if sell_in <= 5 && sell_in > 0 {
            3
        } else if sell_in <= 0 {
            -quality
        } else {
            1
        }
    }

    fn calculate_item_quality_increment(sell_in: i32) -> i32 {
        if sell_in < 1 {
            -2
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod tests {
    mod ragnaros {
        use crate::gildedrose::{GildedRose, Item};

        #[test]
        fn when_updated_then_does_not_alter_quality() {
            // given
            let item = Item::new("Sulfuras", 20, 80);
            let mut rose = GildedRose::new(vec![item]);

            // when
            rose.update_quality();

            // then
            assert_eq!(rose.items[0].quality, 80);
        }

        #[test]
        fn when_updated_then_does_not_alter_sell_in() {
            // given
            let item = Item::new("Sulfuras", 20, 80);
            let mut rose = GildedRose::new(vec![item]);

            // when
            rose.update_quality();

            // then
            assert_eq!(rose.items[0].sell_in, 20);
        }
    }

    mod normal_item {
        use crate::gildedrose::{GildedRose, Item};

        #[test]
        fn when_updated_then_decreases_in_quality() {
            // given
            let item = Item::new("Item", 10, 50);
            let mut rose = GildedRose::new(vec![item]);

            // when
            rose.update_quality();

            // then
            assert_eq!(rose.items[0].quality, 49);
        }

        #[test]
        fn when_updated_then_sell_in_decreases() {
            // given
            let item = Item::new("Item", 10, 50);
            let mut rose = GildedRose::new(vec![item]);

            // when
            rose.update_quality();

            // then
            assert_eq!(rose.items[0].sell_in, 9);
        }

        #[test]
        fn given_zero_quality_when_updated_then_quality_is_zero() {
            // given
            let item = Item::new("Item", 10, 0);
            let mut rose = GildedRose::new(vec![item]);

            // when
            rose.update_quality();

            // then
            assert_eq!(rose.items[0].quality, 0);
        }

        #[test]
        fn given_surpassed_sell_date_when_updated_quality_decreases_by_two() {
            // given
            let item = Item::new("Item", 0, 4);
            let mut rose = GildedRose::new(vec![item]);

            // when
            rose.update_quality();

            // then
            assert_eq!(rose.items[0].quality, 2);
        }
    }

    mod aged_brie {
        use crate::gildedrose::{GildedRose, Item};

        #[test]
        fn when_updated_then_sell_in_decreases() {
            // given
            let item = Item::new("Aged Brie", 2, 10);
            let mut rose = GildedRose::new(vec![item]);

            // when
            rose.update_quality();

            // then
            assert_eq!(rose.items[0].sell_in, 1);
        }

        #[test]
        fn when_updated_then_increases_in_quality() {
            // given
            let item = Item::new("Aged Brie", 2, 10);
            let mut rose = GildedRose::new(vec![item]);

            // when
            rose.update_quality();

            // then
            assert_eq!(rose.items[0].quality, 11);
        }

        #[test]
        fn given_sell_in_date_when_updated_then_increases_in_quality_by_two() {
            // given
            let item = Item::new("Aged Brie", 0, 10);
            let mut rose = GildedRose::new(vec![item]);

            // when
            rose.update_quality();

            // then
            assert_eq!(rose.items[0].quality, 12);
        }

        #[test]
        fn given_quality_of_50_when_updated_then_does_not_alter_quality() {
            // given
            let item = Item::new("Aged Brie", 20, 50);
            let mut rose = GildedRose::new(vec![item]);

            // when
            rose.update_quality();

            // then
            assert_eq!(rose.items[0].quality, 50);
        }
    }

    mod backstage_passes {
        use crate::gildedrose::{GildedRose, Item};

        #[test]
        fn when_updated_then_increases_in_quality() {
            // given
            let item = Item::new("Backstage passes", 11, 10);
            let mut rose = GildedRose::new(vec![item]);

            // when
            rose.update_quality();

            // then
            assert_eq!(rose.items[0].quality, 11);
        }

        #[test]
        fn given_10_days_when_updated_then_increases_in_quality_by_two() {
            // given
            let item = Item::new("Backstage passes", 10, 10);
            let mut rose = GildedRose::new(vec![item]);

            // when
            rose.update_quality();

            // then
            assert_eq!(rose.items[0].quality, 12);
        }

        #[test]
        fn given_5_days_when_updated_then_increases_in_quality_by_three() {
            // given
            let item = Item::new("Backstage passes", 5, 10);
            let mut rose = GildedRose::new(vec![item]);

            // when
            rose.update_quality();

            // then
            assert_eq!(rose.items[0].quality, 13);
        }

        #[test]
        fn given_0_days_when_updated_quality_drops_to_zero() {
            // given
            let item = Item::new("Backstage passes", 0, 10);
            let mut rose = GildedRose::new(vec![item]);

            // when
            rose.update_quality();

            // then
            assert_eq!(rose.items[0].quality, 0);
        }

        #[test]
        fn when_updated_then_sell_in_decreases() {
            // given
            let item = Item::new("Backstage passes", 2, 0);
            let mut rose = GildedRose::new(vec![item]);

            // when
            rose.update_quality();

            // then
            assert_eq!(rose.items[0].sell_in, 1);
        }
    }
}
