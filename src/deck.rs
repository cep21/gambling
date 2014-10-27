use cards;
use suit;
use value;

pub fn regular_52_deck(ret: &mut Vec<cards::CardImpl>) -> &mut Vec<cards::CardImpl> {
    ret.clear();
    ret.reserve_exact(52);
    for &s in suit::SUITS.iter() {
        for &v in value::VALUES.iter() {
            ret.push(cards::CardImpl{v: v, s: s});
        }
    }
    return ret;
}

#[test]
fn test_decks() {
    let mut v = Vec::new();
    let d = regular_52_deck(&mut v);
    assert_eq!(52, d.len());
}
