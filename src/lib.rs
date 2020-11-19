use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::iter::FromIterator;

use serde::{Deserialize, Serialize};

pub type Symbol = char;

pub const EPSILON: Symbol = 'ε';

pub type Alphabet = HashSet<Symbol>;
pub type Language = str;

type CState = str;
pub type State = String;
pub type States = HashSet<State>;
pub type Transitions = HashMap<Symbol, State>;
pub type StateTransitions = HashMap<State, Transitions>;

pub type NfaTransitions = HashMap<Symbol, States>;
pub type NfaStateTransitions = HashMap<State, NfaTransitions>;

type MultiState = BTreeSet<State>;
type MultiStates = BTreeSet<MultiState>;
type MultiNfaTransitions = HashMap<Symbol, MultiState>;
type MultiNfaStateTransitions = BTreeMap<MultiState, MultiNfaTransitions>;

trait ToState {
    fn to_state(&self) -> State;
}

impl ToState for MultiState {
    fn to_state(&self) -> State {
        BTreeSet::from_iter(self.clone())
            .into_iter()
            .collect::<Vec<String>>()
            .join(",")
    }
}

trait ToMultiState {
    fn to_multi_state(&self) -> MultiState;
}

impl ToMultiState for State {
    fn to_multi_state(&self) -> MultiState {
        let mut mstate = MultiState::new();
        mstate.insert(self.clone());
        mstate
    }
}

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

#[derive(Serialize, Deserialize)]
pub struct Nfa {
    states: States,
    alphabet: Alphabet,
    state_transitions: NfaStateTransitions,
    start_state: State,
    final_states: States,
}

struct MultiNfa {
    states: MultiStates,
    alphabet: Alphabet,
    state_transitions: MultiNfaStateTransitions,
    start_state: MultiState,
    final_states: MultiStates,
}

impl Dfa {
    pub fn from_json(json_string: &'static str) -> Dfa {
        let nfa: Nfa = serde_json::from_str(json_string).unwrap();
        Dfa::from_nfa(nfa)
    }

    pub fn from_nfa(nfa: Nfa) -> Dfa {
        match nfa.is_dfa() {
            true => {
                let mut transitions = StateTransitions::new();
                for (state, nfa_transitions) in nfa.state_transitions.iter() {
                    let state_transitions = transitions
                        .entry(state.clone())
                        .or_insert_with(Transitions::new);
                    for (symbol, states) in nfa_transitions {
                        for state in states {
                            state_transitions.insert(*symbol, state.clone());
                        }
                    }
                }

                Dfa {
                    states: nfa.states,
                    alphabet: nfa.alphabet,
                    state_transitions: transitions,
                    start_state: nfa.start_state,
                    final_states: nfa.final_states,
                }
            }
            _ => {
                let multi_nfa = MultiNfa::from_nfa(&nfa);
                let states: States = multi_nfa
                    .states
                    .into_iter()
                    .map(|state| state.to_state())
                    .collect();
                let alphabet: Alphabet = multi_nfa.alphabet;
                let state_transitions: StateTransitions = multi_nfa
                    .state_transitions
                    .into_iter()
                    .map(|(state, transitions)| {
                        (
                            state.to_state(),
                            transitions
                                .into_iter()
                                .map(|(symbol, to_state)| (symbol, to_state.to_state()))
                                .collect(),
                        )
                    })
                    .collect();
                let start_state: State = multi_nfa.start_state.to_state();
                let final_states: States = multi_nfa
                    .final_states
                    .into_iter()
                    .map(|state| state.to_state())
                    .collect();
                Dfa {
                    states,
                    alphabet,
                    state_transitions,
                    start_state,
                    final_states,
                }
            }
        }
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

    pub fn accepts(&self, language: &Language) -> Acceptance {
        let mut walk_state = &self.start_state;
        for walk_symbol in language.chars() {
            if self.alphabet.get(&walk_symbol).is_none() {
                return Acceptance::InvalidAlphabet;
            }

            walk_state = match self.state_transitions.get(walk_state) {
                Some(walk_state_transitions) => match walk_state_transitions.get(&walk_symbol) {
                    Some(walk_state_transition_for_symbol) => walk_state_transition_for_symbol,
                    _ => return Acceptance::NoTransition,
                },
                _ => return Acceptance::NoTransition,
            }
        }
        match self.final_states.get(walk_state) {
            Some(_) => Acceptance::Accepted,
            _ => Acceptance::Rejected,
        }
    }
}

impl Nfa {
    pub fn is_dfa(&self) -> bool {
        !self.state_transitions.values().any(|state_transitions| {
            state_transitions
                .iter()
                .any(|(symbol, transitions)| symbol.eq(&EPSILON) || transitions.len() > 1)
        })
    }
}

impl MultiNfa {
    pub fn from_nfa(nfa: &Nfa) -> MultiNfa {
        let mut state_transitions = MultiNfaStateTransitions::new();

        // First, find all states reachable by epsilon closure from the start state.
        let mut start_state = nfa.start_state.to_multi_state();
        MultiNfa::epsilon_closure(&mut start_state, &nfa.start_state, &nfa.state_transitions);

        // Recursively find all states reachable by reading input from the start state.
        MultiNfa::aggregate_transitions(
            &mut state_transitions,
            &nfa.start_state.to_multi_state(),
            &nfa.state_transitions,
        );

        let states: MultiStates = state_transitions.keys().cloned().collect();

        let mut final_states = MultiStates::new();
        for state in states.iter() {
            for final_state in nfa.final_states.iter() {
                if state.contains(final_state) {
                    final_states.insert(state.clone());
                }
            }
        }

        let alphabet = nfa.alphabet.clone();

        MultiNfa {
            states,
            alphabet,
            state_transitions,
            start_state,
            final_states,
        }
    }

    fn epsilon_closure(
        reachable_mstate: &mut MultiState,
        initial_state: &CState,
        state_transitions: &NfaStateTransitions,
    ) {
        if let Some(initial_state_transitions) = state_transitions.get(initial_state) {
            if let Some(initial_state_epsilon_transitions) = initial_state_transitions.get(&EPSILON)
            {
                for epsilon_transition_state in initial_state_epsilon_transitions.iter() {
                    if reachable_mstate.insert(epsilon_transition_state.clone()) {
                        // Recurse if new reachable state was inserted
                        MultiNfa::epsilon_closure(
                            reachable_mstate,
                            epsilon_transition_state,
                            state_transitions,
                        );
                    }
                }
            }
        }
    }

    fn aggregate_transitions(
        all_transitions: &mut MultiNfaStateTransitions,
        current_state: &MultiState,
        state_transitions: &NfaStateTransitions,
    ) {
        // Check if there is at least one transition for the current state.
        if current_state
            .iter()
            .any(|state| state_transitions.get(state).is_none())
        {
            return;
        };

        // Get epsilon closure before finding reachable states.
        let mut reachable_current_mstate = current_state.clone();
        for state in current_state {
            MultiNfa::epsilon_closure(&mut reachable_current_mstate, state, state_transitions);
        }

        // Return if transitions are already calculated
        if all_transitions.get(&reachable_current_mstate).is_some() {
            return;
        };

        // Get transitions for all reachable states.
        let mut aggregated_current_state_transitions = MultiNfaTransitions::new();

        for reachable_state in reachable_current_mstate.iter() {
            if let Some(reachable_state_transitions) = state_transitions.get(reachable_state) {
                for (symbol, reachable_state_transitions) in reachable_state_transitions {
                    if symbol.ne(&EPSILON) {
                        let mut reachable_states_with_epsilon_closure = reachable_state_transitions
                            .clone()
                            .into_iter()
                            .collect::<MultiState>();
                        for state in reachable_state_transitions {
                            MultiNfa::epsilon_closure(
                                &mut reachable_states_with_epsilon_closure,
                                state,
                                state_transitions,
                            );
                        }

                        aggregated_current_state_transitions
                            .entry(*symbol)
                            .or_insert_with(MultiState::new)
                            .extend(reachable_states_with_epsilon_closure);
                    }
                }
            }
        }

        // Add transitions for current state to all_transitions so they aren't duplicated when we recurse.
        all_transitions.insert(
            reachable_current_mstate,
            aggregated_current_state_transitions.clone(),
        );

        let reachable_states = aggregated_current_state_transitions
            .values()
            .cloned()
            .collect::<MultiStates>();

        // Recurse on reachable states
        for reachable_state in reachable_states.iter() {
            MultiNfa::aggregate_transitions(all_transitions, reachable_state, state_transitions);
        }
    }
}
