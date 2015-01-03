#![feature(phase)]

#[phase(plugin)]
extern crate lazy_static;
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
    pub mod builder;
}
pub mod hand;
pub mod cardshoetracker;
pub mod bjaction;
pub mod action_calculator;
pub mod strategy_generator;
pub mod rules;
pub mod hand_hasher;
pub mod hash_database;
pub mod time;
