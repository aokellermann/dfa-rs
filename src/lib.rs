use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_json;

pub type State = String;
pub type States = HashSet<State>;

pub type Symbol = char;

pub const EPSILON: Symbol = 'ε';

pub type Alphabet = HashSet<Symbol>;
pub type Language = String;

pub type Transitions = HashMap<Symbol, State>;
pub type StateTransitions = HashMap<State, Transitions>;

#[derive(Debug, PartialEq)]
pub enum Acceptance {
    Accepted,
    Rejected,
    InvalidAlphabet,
    NoTransition,
}

#[derive(Serialize, Deserialize)]
pub struct Dfa {
    states: States,
    alphabet: Alphabet,
    state_transitions: StateTransitions,
    start_state: State,
    final_states: States,
}

impl Dfa {
    pub fn new(json_string: &'static str) -> Dfa {
        serde_json::from_str(json_string).unwrap()
    }

    pub fn get_states(&self) -> &States {
        &self.states
    }
    pub fn get_alphabet(&self) -> &Alphabet {
        &self.alphabet
    }
    pub fn get_state_transitions(&self) -> &StateTransitions {
        &self.state_transitions
    }
    pub fn get_start_state(&self) -> &State {
        &self.start_state
    }
    pub fn get_final_states(&self) -> &States {
        &self.final_states
    }

    pub fn accepts(&self, language: Language) -> Acceptance {
        let mut walk_state = &self.start_state;
        for walk_symbol in language.chars() {
            match self.alphabet.get(&walk_symbol) {
                None => return Acceptance::InvalidAlphabet,
                _ => (),
            }
            walk_state = match self.state_transitions.get(walk_state) {
                Some(walk_state_transitions) => match walk_state_transitions.get(&walk_symbol) {
                    Some(walk_state_transition_for_symbol) => walk_state_transition_for_symbol,
                    _ => return Acceptance::NoTransition,
                },
                _ => return Acceptance::NoTransition
            }
        };
        match self.final_states.get(walk_state) {
            Some(_) => Acceptance::Accepted,
            _ => Acceptance::Rejected,
        }
    }
}
