use cards::suit;
use cards::card::Card;
use cards::value;

pub fn cards_in_deck<'a>(num_decks: uint) -> Vec<Card> {
    let mut ret = Vec::new();
    ret.clear();
    ret.reserve_exact(52 * num_decks);
    for _ in range(0, num_decks) {
        for s in suit::SUITS.iter() {
            for v in value::VALUES.iter() {
                let v1 = (*v).clone();
                let s1 = (*s).clone();
                ret.push(Card::new(v1, s1));
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
