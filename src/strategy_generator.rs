use action_calculator::ActionCalculator;
use hash_database::HashDatabase;
use hash_database::InMemoryHashDatabase;
use hand::BJHand;
use cards::card::Card;
use shoe::shoe::DirectShoe;
use bjaction::BJAction;
use rules::BJRules;

/**
 * Calculate a strategy chart for blackjack.  To calculate if you
 * should hit or stand at a value, we first calculate every way to get
 * to that value and weigh that against the expected value of each action
 * assuming that all future play uses the same strategy.
 *
 * This means we calculate higher values first, since we need to know if we
 * will hit or stand on that 16 vs 10 before we can know what the EV of hitting
 * or standing 15 vs 10 is.
 */
pub struct StrategyGenerator<'a, 'b> {
    action_calc: Box<ActionCalculator<'a, 'b>>,
    database: Box<HashDatabase + 'a>,
}

// Store: Odds of the situation are X:
//          Then, for each situation, store the EV assuming you
//          only do basic strategy for each next situation.
//
//          For splits, this means we don't allow just splitting
//          two times so we enforce that you split each time if
//          you split any times.
//
// Then simplify the situation and add up the odds times EV
// to get the simplified result.
// 
// Splits get complicated because they create cycles.  For example,
// should you hit 16 vs 10?  If you know you'll split 8/8 vs 10, then
// that changes the combinations of 16s you get to be less bias towards 8.
// But to know if you should split 8/8 vs 10, you need to know what will happen
// if one if your 8's becomes 16 and you're faced with 16 vs 10.  Now we have
// a basic strategy cycle.
//
// To resolve this, I calculate all basic strategy hit/stand/double/sur assuming no
// splits, then splits assuming those actions.
// 
// I must know exactly what I will do on 16 vs 10 before I can calculate 15 vs 10

impl <'a, 'b> StrategyGenerator<'a, 'b> {
/*    fn new() -> StrategyGenerator<'a> {
        StrategyGenerator{
            action_calc: box ActionCalculator::new(),
            database: box InMemoryHashDatabase::new(),
        }
    }
    */
}
#[cfg(test)]
mod tests {
    extern crate test;
}
