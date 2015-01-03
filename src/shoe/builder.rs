use shoe::shoe::DirectShoe;
use shoe::randomshoe::new_random_shoe;
use cards::card::Card;
use shoe::directshoe::DirectActualShoe;

pub struct ShoeBuilder {
    num_decks: u32,
    cards: Option<Vec<Card>>
}

impl ShoeBuilder {
    pub fn new() -> ShoeBuilder {
        ShoeBuilder {
            num_decks: 1,
            cards: None,
        }
    }

//    pub fn build<'b>(&'b mut self, c: &'b mut Vec<Card>) -> &'b DirectShoe {
//        return &'b new_random_shoe(1);
//        let r: &'b DirectShoe = &DirectActualShoe::new(c);
//        return r;
//    }
}
