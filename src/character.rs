use std::collections::HashMap;

use common::Facing;
use entity::Entity;
use map::Map;
use state::{State, StateMachine};

// StateData for character
pub struct CharacterData<'a> {
    map: &'a mut Map,
    character: &'a mut Character,
    entity: &'a mut Entity,
}

pub struct Character {
    entity_id: String,
    anims: HashMap<String, Vec<usize>>,
    facing: Facing,
}

pub struct CharacterController<'a> {
    entity_id: String,
    state_machine: StateMachine<'a, CharacterData<'a>>,
}

impl<'a> CharacterController<'a> {
    pub fn new<S: State<CharacterData<'a>> + 'a>(character: Character, inital_state: S) -> Self {
        CharacterController {
            entity_id: character.entity_id.clone(),
            state_machine: StateMachine::new(inital_state),
        }
    }
}