use crate::model::format::ass::stylist::{
    style_strategy::StyleStrategy,
    stylist::{
        strategies::{main_strategy::MainStrategy, second_strategy::SecondStrategy},
        style_type::StyleType,
    },
};

pub struct StrategyFactory;

impl StrategyFactory {
    pub fn create(style_type: &StyleType) -> Box<dyn StyleStrategy> {
        match style_type {
            StyleType::Main => Box::new(MainStrategy::new()),
            StyleType::Second => Box::new(SecondStrategy::new()),
        }
    }
}
