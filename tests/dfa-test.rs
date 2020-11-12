use dfa_rs::{Dfa, State, Symbol};

struct DfaTransition {
    pub s1: State,
    pub symbol: Symbol,
    pub s2: State,
}

#[test]
fn deserialize() {
    let json_string = r#"{
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
              "0": "q1",
              "1": "q2"
            },
            "q2": {
              "0": "q3",
              "1": "q2"
            },
            "q3": {
              "0": "q2",
              "1": "q2"
            }
          }
        }"#;

    let dfa = Dfa::new(json_string);

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
