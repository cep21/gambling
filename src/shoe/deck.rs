use cards::suit;
use cards::card::Card;
use cards::value;

pub fn cards_in_deck<'a>(num_decks: u32) -> Vec<Card> {
    let mut ret = Vec::new();
    ret.clear();
    ret.reserve_exact(52 * num_decks as usize);
    for _ in range(0, num_decks) {
        for s in suit::SUITS.iter() {
            for v in value::VALUES.iter() {
                ret.push(Card::new(*v, *s));
            }
        }
    }
    return ret;
}

#[test]
fn test_decks() {
    let d = cards_in_deck(2);
    assert_eq!(104, d.len());
}
