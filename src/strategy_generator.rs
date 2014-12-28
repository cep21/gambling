use action_calculator::ActionCalculator;
use hash_database::HashDatabase;
use hash_database::InMemoryHashDatabase;

pub struct StrategyGenerator<'a> {
    action_calc: &'a mut ActionCalculator<'a>,
    database: &'a mut (HashDatabase + 'a),
}

impl <'a> StrategyGenerator<'a> {
    fn new() -> StrategyGenerator<'a> {
        StrategyGenerator{
            action_calc: &ActionCalculator::new(),
            database: &mut InMemoryHashDatabase::new(),
        }
    }
}
#[cfg(test)]
mod tests {
    extern crate test;
}
