use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug, Clone)]
#[storage(NullStorage)]
pub struct Grounded;

#[derive(Component, Debug, Clone)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

#[derive(Component, Debug, Clone)]
#[storage(NullStorage)]
pub struct Player;
