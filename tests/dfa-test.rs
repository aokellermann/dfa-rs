use dfa_rs::*;

type TestLanguage = &'static str;
type TestLanguages = Vec<TestLanguage>;

type TestAlphabet = &'static str;

type TestSymbol = Symbol;
type TestState = &'static str;
type TestStates = Vec<TestState>;

struct TestTransition {
    pub s1: TestState,
    pub symbol: TestSymbol,
    pub s2: TestState,
}

type TestTransitions = Vec<TestTransition>;

const TEST_DFA_JSON1: &'static str = r#"{"states":["q1","q2","q3"],"alphabet":["0","1"],"start_state":"q1","final_states":["q2"],"state_transitions":{"q1":{"0":["q1"],"1":["q2"]},"q2":{"0":["q3"],"1":["q2"]},"q3":{"0":["q2"],"1":["q2"]}}}"#;
const TEST_NFA_JSON1: &'static str = r#"{"states":["q0","q1","q2","q3"],"alphabet":["a","b"],"start_state":"q0","final_states":["q0"],"state_transitions":{"q0":{"ε":["q1"]},"q1":{"a":["q1","q2"],"b":["q2"]},"q2":{"a":["q0","q2"],"b":["q3"]},"q3":{"b":["q1"]}}}"#;

fn assert_dfa_state(dfa: Dfa,
                    states: TestStates,
                    alphabet: TestAlphabet,
                    start_state: TestState,
                    final_states: TestStates,
                    transitions: TestTransitions) {
    for state in states {
        assert!(dfa.get_states().contains(state),
                "state <{}> not found in states <{:#?}>", state, dfa.get_states());
    }

    for symbol in alphabet.chars() {
        assert!(dfa.get_alphabet().contains(&symbol),
                "symbol <{}> not found in alphabet <{:#?}>", symbol, dfa.get_alphabet());
    }

    assert_eq!(dfa.get_start_state(), start_state,
               "state <{}> not correct start state <{:#?}>", start_state, dfa.get_start_state());

    for state in final_states {
        assert!(dfa.get_final_states().contains(state),
                "state <{}> not found in final states <{:#?}>", state, dfa.get_final_states());
    }

    for expected_transition in transitions.iter() {
        let transition = dfa.get_state_transitions()
            .get(expected_transition.s1)
            .expect(format!("Transitions not found for state <{:#?}> in transitions <{:#?}>",
                            expected_transition.s1,
                            dfa.get_state_transitions()).as_str())
            .get(&expected_transition.symbol)
            .expect(format!(
                "Transition not found for state <{:#?}> for symbol <{:#?}> in transitions <{:#?}>",
                expected_transition.s1,
                expected_transition.symbol,
                dfa.get_state_transitions()).as_str());
        assert_eq!(&expected_transition.s2, transition,
                   "Incorrect transition for state <{:#?}> for symbol <{:#?}>
                   in transitions <{:#?}>",
                   expected_transition.s1,
                   expected_transition.symbol,
                   dfa.get_state_transitions()
        );
    }
}

fn assert_dfa_acceptance(dfa: Dfa, languages: TestLanguages, acceptance: Acceptance) {
    for language in languages {
        assert_eq!(acceptance, dfa.accepts(&language.to_string()), "language <{}>", language);
    }
}

#[test]
fn deserialize() {
    let dfa = Dfa::from_json(TEST_DFA_JSON1);

    assert_dfa_state(dfa,
                     vec!["q1", "q2", "q3"],
                     "01",
                     "q1",
                     vec!["q2"],
                     vec![
                         TestTransition { s1: "q1", symbol: '0', s2: "q1" },
                         TestTransition { s1: "q1", symbol: '1', s2: "q2" },
                         TestTransition { s1: "q2", symbol: '0', s2: "q3" },
                         TestTransition { s1: "q2", symbol: '1', s2: "q2" },
                         TestTransition { s1: "q3", symbol: '0', s2: "q2" },
                         TestTransition { s1: "q3", symbol: '1', s2: "q2" },
                     ],
    );
}

#[test]
fn accepts() {
    let dfa = Dfa::from_json(TEST_DFA_JSON1);

    assert_dfa_acceptance(dfa,
                          vec!["11111", "00100", "11100", "110011", "001001", "0010001"],
                          Acceptance::Accepted);
}

#[test]
fn rejects() {
    let dfa = Dfa::from_json(TEST_DFA_JSON1);

    assert_dfa_acceptance(dfa,
                          vec!["00000", "01010", "001000"],
                          Acceptance::Rejected);
}

#[test]
fn invalid_alphabet() {
    let dfa = Dfa::from_json(TEST_DFA_JSON1);

    assert_dfa_acceptance(dfa,
                          vec!["a11111", "00100b", "111c00", "111020", "1-11c00"],
                          Acceptance::InvalidAlphabet);
}

#[test]
fn deserialize_nfa() {
    let dfa = Dfa::from_json(TEST_NFA_JSON1);

    assert_dfa_state(dfa,
                     vec!["q1", "q2", "q3", "q0,q1", "q1,q2", "q1,q3", "q2,q3", "q0,q1,q2"],
                     "ab",
                     "q0,q1",
                     vec!["q0,q1", "q0,q1,q2"],
                     vec![
                         TestTransition { s1: "q1", symbol: 'a', s2: "q1,q2", },
                         TestTransition { s1: "q1", symbol: 'b', s2: "q2", },
                         TestTransition { s1: "q2", symbol: 'a', s2: "q0,q1,q2", },
                         TestTransition { s1: "q2", symbol: 'b', s2: "q3", },
                         TestTransition { s1: "q3", symbol: 'b', s2: "q1", },
                         TestTransition { s1: "q0,q1", symbol: 'a', s2: "q1,q2", },
                         TestTransition { s1: "q0,q1", symbol: 'b', s2: "q2", },
                         TestTransition { s1: "q1,q2", symbol: 'a', s2: "q0,q1,q2", },
                         TestTransition { s1: "q1,q2", symbol: 'b', s2: "q2,q3", },
                         TestTransition { s1: "q1,q3", symbol: 'a', s2: "q1,q2", },
                         TestTransition { s1: "q1,q3", symbol: 'b', s2: "q1,q2", },
                         TestTransition { s1: "q2,q3", symbol: 'a', s2: "q0,q1,q2", },
                         TestTransition { s1: "q2,q3", symbol: 'b', s2: "q1,q3", },
                         TestTransition { s1: "q0,q1,q2", symbol: 'a', s2: "q0,q1,q2", },
                         TestTransition { s1: "q0,q1,q2", symbol: 'b', s2: "q2,q3", },
                     ],
    );
}
