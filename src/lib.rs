#![feature(phase)]
#![feature(associated_types)]
#![feature(default_type_params)]

pub mod cards {
    pub mod suit;
    pub mod color;
    pub mod value;
    pub mod card;
}
pub mod shoe {
    pub mod deck;
    pub mod shoe;
    pub mod directshoe;
    pub mod randomshoe;
}
pub mod hand;
pub mod cardshoetracker;
pub mod bjaction;
pub mod action_calculator;
pub mod rules;
pub mod hand_hasher;
pub mod hash_database;
pub mod time;
