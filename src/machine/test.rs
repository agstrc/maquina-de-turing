use super::*;

/// Define, em JSON, uma máquina de turing "zero n, um n", que busca *n* símbolos zero
/// seguidos de *n* símbolos um.
static JSON: &str = r#"{"alphabet":["0","1","X","Y","B"],"blank_symbol":"B","input_symbols":["0","1"],"states":["q0","q1","q2","q3","q4"],"initial_state":"q0","final_states":["q3"],"transitions":[{"from_state":"q0","read_symbol":"0","write_symbol":"X","move_to":"R","next_state":"q1"},{"from_state":"q0","read_symbol":"B","write_symbol":"B","move_to":"R","next_state":"q3"},{"from_state":"q0","read_symbol":"Y","write_symbol":"Y","move_to":"R","next_state":"q4"},{"from_state":"q1","read_symbol":"0","write_symbol":"0","move_to":"R","next_state":"q1"},{"from_state":"q1","read_symbol":"Y","write_symbol":"Y","move_to":"R","next_state":"q1"},{"from_state":"q1","read_symbol":"1","write_symbol":"Y","move_to":"L","next_state":"q2"},{"from_state":"q2","read_symbol":"0","write_symbol":"0","move_to":"L","next_state":"q2"},{"from_state":"q2","read_symbol":"Y","write_symbol":"Y","move_to":"L","next_state":"q2"},{"from_state":"q2","read_symbol":"X","write_symbol":"X","move_to":"R","next_state":"q0"},{"from_state":"q4","read_symbol":"Y","write_symbol":"Y","move_to":"R","next_state":"q4"},{"from_state":"q4","read_symbol":"B","write_symbol":"B","move_to":"R","next_state":"q3"}]}"#;

#[test]
fn test_zeron_onen() {
    let septuple = Septuple::from_json(JSON).unwrap();

    let mut tm = Machine::new(&septuple, vec!['0', '1']).unwrap();
    while tm.transition().is_none() {}
    assert_eq!(tm.transition().unwrap(), Acceptance::Accepted);

    let mut tm = Machine::new(&septuple, vec!['1', '0']).unwrap();
    while tm.transition().is_none() {}
    assert_eq!(tm.transition().unwrap(), Acceptance::Rejected);

    let mut tm = Machine::new(&septuple, vec!['0', '0', '0', '1', '1', '1']).unwrap();
    while tm.transition().is_none() {}
    assert_eq!(tm.transition().unwrap(), Acceptance::Accepted);
}

#[test]
fn test_undo() {
    let septuple = Septuple::from_json(JSON).unwrap();
    let initial_tape = vec!['0', '0', '1', '1'];
    let mut tm = Machine::new(&septuple, initial_tape.clone()).unwrap();

    // roda a máquina até seu estado de aceitação.
    while tm.transition().is_none() {}
    assert_eq!(tm.tape, vec!['X', 'X', 'Y', 'Y', 'B', 'B']);
    assert_eq!(tm.transition().unwrap(), Acceptance::Accepted);

    // desfaz as transições da máquina até retornar ao estado inicial
    while tm.undo_transition().is_ok() {}
    assert_eq!(tm.tape, initial_tape);
    assert_eq!(tm.current_state, &tm.septuple.initial_state);
    assert_eq!(tm.current_position, 0);
}
