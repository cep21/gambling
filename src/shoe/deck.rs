use cards::card::CardImpl;
use cards::suit;
use cards::value;

pub fn cards_in_deck(numDecks: uint, ret: &mut Vec<CardImpl>) -> &mut Vec<CardImpl> {
    ret.clear();
    ret.reserve_exact(52 * numDecks);
    for i in range(0, numDecks) {
        for &s in suit::SUITS.iter() {
            for &v in value::VALUES.iter() {
                ret.push(CardImpl{v: v, s: s});
            }
        }
    }
    return ret;
}

#[test]
fn test_decks() {
    let mut v = Vec::new();
    let d = cards_in_deck(2, &mut v);
    assert_eq!(104, d.len());
}
