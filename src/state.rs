use std::{collections::HashSet, time::Duration};

use crossterm::event::{self, Event, KeyCode};
use tui::{backend::Backend, Terminal};

use crate::{
    draw::{self, original_tape_spans},
    machine::Machine,
    Either::{self, L, R},
    Result,
};

pub struct Quit;

/// Entra no estado de leitura e validação de fita. Retorna somente quando a fita
/// inserida tenha apenas símbolos dentro do set de símbolos.
pub fn read_valid_tape<B: Backend>(
    term: &mut Terminal<B>,
    input_symbols: &HashSet<char>,
) -> Result<Either<Vec<char>, Quit>> {
    loop {
        let tape = match read_any_tape(term, input_symbols)? {
            L(vec) => vec,
            R(quit) => {
                return Ok(R(quit));
            }
        };
        if tape.iter().any(|char| !input_symbols.contains(char)) {
            bad_tape(term, input_symbols)?;
        } else {
            return Ok(L(tape));
        }
    }
}

/// Entra no estado de leitura de fita. Retorna após receber a tecla `Enter`.
fn read_any_tape<B: Backend>(
    term: &mut Terminal<B>,
    input_symbols: &HashSet<char>,
) -> Result<Either<Vec<char>, Quit>> {
    let mut buffer = String::new();

    loop {
        term.draw(|frame| draw::tape_input(frame, &buffer[..], input_symbols))?;

        let poll = event::poll(Duration::from_millis(50))?;
        if !poll {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Backspace => {
                    buffer.pop();
                }
                KeyCode::Enter => {
                    return Ok(L(buffer.chars().collect()));
                }
                KeyCode::Char(char) => {
                    buffer.push(char);
                }
                KeyCode::Esc => {
                    return Ok(R(Quit));
                }
                _ => (),
            }
        }
    }
}

/// Entra no estado de erro causado por uma inserção de fita inválida. Deixa o estado
/// após qualquer tecla ser pressionada.
fn bad_tape<B: Backend>(term: &mut Terminal<B>, input_symbols: &HashSet<char>) -> Result<()> {
    loop {
        term.draw(|f| draw::bad_tape(f, input_symbols))?;

        if !event::poll(Duration::from_millis(50))? {
            continue;
        }

        if let Event::Key(_) = event::read()? {
            return Ok(());
        }
    }
}

// Entra no estado de processamento da máquina. Retorna quando o usuário aperta `Esc`.
pub fn process_machine<B: Backend>(term: &mut Terminal<B>, machine: &mut Machine) -> Result<Quit> {
    let og_tape = original_tape_spans(machine.tape());

    loop {
        let acceptance = machine.acceptance();
        term.draw(|f| draw::machine(f, machine, acceptance, og_tape.clone()))?;

        if !event::poll(Duration::from_millis(50))? {
            continue;
        }

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Left => {
                    let _ = machine.undo_transition();
                }
                KeyCode::Right => {
                    let _ = machine.transition();
                }
                KeyCode::Esc => return Ok(Quit),
                _ => (),
            }
        }
    }
}
