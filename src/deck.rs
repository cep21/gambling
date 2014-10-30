use cards;
use suit;
use value;

pub fn cards_in_deck(numDecks: uint, ret: &mut Vec<cards::CardImpl>) -> &mut Vec<cards::CardImpl> {
    ret.clear();
    ret.reserve_exact(52 * numDecks);
    for i in range(0, numDecks) {
        for &s in suit::SUITS.iter() {
            for &v in value::VALUES.iter() {
                ret.push(cards::CardImpl{v: v, s: s});
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
