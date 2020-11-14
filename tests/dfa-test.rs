use dfa_rs::*;

struct DfaTransition {
    pub s1: State,
    pub symbol: Symbol,
    pub s2: State,
}

const TEST_DFA_JSON1: &'static str = r#"{
          "states": [
            "q1",
            "q2",
            "q3"
          ],
          "alphabet": [
            "0",
            "1"
          ],
          "start_state": "q1",
          "final_states": ["q2"],
          "state_transitions": {
            "q1": {
              "0": ["q1"],
              "1": ["q2"]
            },
            "q2": {
              "0": ["q3"],
              "1": ["q2"]
            },
            "q3": {
              "0": ["q2"],
              "1": ["q2"]
            }
          }
        }"#;

const TEST_NFA_JSON1: &'static str = r#"{
	"states": [
		"q0",
		"q1",
		"q2",
		"q3"
	],
	"alphabet": [
		"a",
		"b"
	],
	"start_state": "q0",
	"final_states": ["q0"],
	"state_transitions": {
		"q0": {
			"ε": ["q1"]
		},
		"q1": {
			"a": ["q1", "q2"],
			"b": ["q2"]
		},
		"q2": {
			"a": ["q0", "q2"],
			"b": ["q3"]
		},
		"q3": {
			"b": ["q1"]
		}
	}
}
"#;

#[test]
fn deserialize() {
    let dfa = Dfa::new(TEST_DFA_JSON1);

    let states = dfa.get_states();
    assert!(states.contains("q1"));
    assert!(states.contains("q2"));
    assert!(states.contains("q3"));

    let alphabet = dfa.get_alphabet();
    assert!(alphabet.contains(&'0'));
    assert!(alphabet.contains(&'1'));

    assert_eq!(dfa.get_start_state(), "q1");
    assert!(dfa.get_final_states().contains("q2"));

    let state_transitions = dfa.get_state_transitions();
    let expected_transitions = [
        DfaTransition {
            s1: "q1".parse().unwrap(),
            symbol: "0".parse().unwrap(),
            s2: "q1".parse().unwrap(),
        },
        DfaTransition {
            s1: "q1".parse().unwrap(),
            symbol: "1".parse().unwrap(),
            s2: "q2".parse().unwrap(),
        },
        DfaTransition {
            s1: "q2".parse().unwrap(),
            symbol: "0".parse().unwrap(),
            s2: "q3".parse().unwrap(),
        },
        DfaTransition {
            s1: "q2".parse().unwrap(),
            symbol: "1".parse().unwrap(),
            s2: "q2".parse().unwrap(),
        },
        DfaTransition {
            s1: "q3".parse().unwrap(),
            symbol: "0".parse().unwrap(),
            s2: "q2".parse().unwrap(),
        },
        DfaTransition {
            s1: "q3".parse().unwrap(),
            symbol: "1".parse().unwrap(),
            s2: "q2".parse().unwrap(),
        },
    ];

    for expected_transition in expected_transitions.iter() {
        let transitions = state_transitions
            .get(&expected_transition.s1)
            .expect("Transitions not found");
        let transition = transitions
            .get(&expected_transition.symbol)
            .expect("Transition not found");
        assert_eq!(&expected_transition.s2, transition, "Incorrect transition");
    }
}

#[test]
fn accepts() {
    let dfa = Dfa::new(TEST_DFA_JSON1);

    assert_eq!(dfa.accepts("11111".parse().unwrap()), Acceptance::Accepted);
    assert_eq!(dfa.accepts("00100".parse().unwrap()), Acceptance::Accepted);
    assert_eq!(dfa.accepts("11100".parse().unwrap()), Acceptance::Accepted);
    assert_eq!(dfa.accepts("110011".parse().unwrap()), Acceptance::Accepted);
    assert_eq!(dfa.accepts("001001".parse().unwrap()), Acceptance::Accepted);
    assert_eq!(dfa.accepts("0010001".parse().unwrap()), Acceptance::Accepted);
}

#[test]
fn rejects() {
    let dfa = Dfa::new(TEST_DFA_JSON1);

    assert_eq!(dfa.accepts("00000".parse().unwrap()), Acceptance::Rejected);
    assert_eq!(dfa.accepts("01010".parse().unwrap()), Acceptance::Rejected);
    assert_eq!(dfa.accepts("001000".parse().unwrap()), Acceptance::Rejected);
}

#[test]
fn invalid_alphabet() {
    let dfa = Dfa::new(TEST_DFA_JSON1);

    assert_eq!(dfa.accepts("a11111".parse().unwrap()), Acceptance::InvalidAlphabet);
    assert_eq!(dfa.accepts("00100b".parse().unwrap()), Acceptance::InvalidAlphabet);
    assert_eq!(dfa.accepts("111c00".parse().unwrap()), Acceptance::InvalidAlphabet);
    assert_eq!(dfa.accepts("111020".parse().unwrap()), Acceptance::InvalidAlphabet);
    assert_eq!(dfa.accepts("1-11c00".parse().unwrap()), Acceptance::InvalidAlphabet);
}

#[test]
fn deserialize_nfa() {
    let dfa = Dfa::new(TEST_NFA_JSON1);

    let states = dfa.get_states();
    assert!(states.contains("q1"));
    assert!(states.contains("q2"));
    assert!(states.contains("q3"));
    assert!(states.contains("q0,q1"));
    assert!(states.contains("q1,q2"));
    assert!(states.contains("q1,q3"));
    assert!(states.contains("q2,q3"));
    assert!(states.contains("q0,q1,q2"));

    let alphabet = dfa.get_alphabet();
    assert!(alphabet.contains(&'a'));
    assert!(alphabet.contains(&'b'));

    assert_eq!(dfa.get_start_state(), "q0,q1");
    assert!(dfa.get_final_states().contains("q0,q1"));
    assert!(dfa.get_final_states().contains("q0,q1,q2"));

    let state_transitions = dfa.get_state_transitions();
    let expected_transitions = [
        DfaTransition {
            s1: "q1".parse().unwrap(),
            symbol: "a".parse().unwrap(),
            s2: "q1,q2".parse().unwrap(),
        },
        DfaTransition {
            s1: "q1".parse().unwrap(),
            symbol: "b".parse().unwrap(),
            s2: "q2".parse().unwrap(),
        },
        DfaTransition {
            s1: "q2".parse().unwrap(),
            symbol: "a".parse().unwrap(),
            s2: "q0,q1,q2".parse().unwrap(),
        },
        DfaTransition {
            s1: "q2".parse().unwrap(),
            symbol: "b".parse().unwrap(),
            s2: "q3".parse().unwrap(),
        },
        DfaTransition {
            s1: "q3".parse().unwrap(),
            symbol: "b".parse().unwrap(),
            s2: "q1".parse().unwrap(),
        },
        DfaTransition {
            s1: "q0,q1".parse().unwrap(),
            symbol: "a".parse().unwrap(),
            s2: "q1,q2".parse().unwrap(),
        },
        DfaTransition {
            s1: "q0,q1".parse().unwrap(),
            symbol: "b".parse().unwrap(),
            s2: "q2".parse().unwrap(),
        },
        DfaTransition {
            s1: "q1,q2".parse().unwrap(),
            symbol: "a".parse().unwrap(),
            s2: "q0,q1,q2".parse().unwrap(),
        },
        DfaTransition {
            s1: "q1,q2".parse().unwrap(),
            symbol: "b".parse().unwrap(),
            s2: "q2,q3".parse().unwrap(),
        },
        DfaTransition {
            s1: "q1,q3".parse().unwrap(),
            symbol: "a".parse().unwrap(),
            s2: "q1,q2".parse().unwrap(),
        },
        DfaTransition {
            s1: "q1,q3".parse().unwrap(),
            symbol: "b".parse().unwrap(),
            s2: "q1,q2".parse().unwrap(),
        },
        DfaTransition {
            s1: "q2,q3".parse().unwrap(),
            symbol: "a".parse().unwrap(),
            s2: "q0,q1,q2".parse().unwrap(),
        },
        DfaTransition {
            s1: "q2,q3".parse().unwrap(),
            symbol: "b".parse().unwrap(),
            s2: "q1,q3".parse().unwrap(),
        },
        DfaTransition {
            s1: "q0,q1,q2".parse().unwrap(),
            symbol: "a".parse().unwrap(),
            s2: "q0,q1,q2".parse().unwrap(),
        },
        DfaTransition {
            s1: "q0,q1,q2".parse().unwrap(),
            symbol: "b".parse().unwrap(),
            s2: "q2,q3".parse().unwrap(),
        },
    ];

    for expected_transition in expected_transitions.iter() {
        let transitions = state_transitions
            .get(&expected_transition.s1)
            .expect("Transitions not found");
        let transition = transitions
            .get(&expected_transition.symbol)
            .expect("Transition not found");
        assert_eq!(&expected_transition.s2, transition, "Incorrect transition");
    }
}
