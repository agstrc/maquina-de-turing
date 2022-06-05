//! Esse submódulo de [`draw`](super) possui funções auxiliares, destinadas à construção
//! de elementos a serem desenhados.

use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

use crate::machine::sep::{Movement, Septuple};

/// Divide a tela em quatro sub-áreas da área passada, sendo elas duas pequenas fitas no
/// topo, uma área para a sétupla no meio, e uma área para ajuda abaixo.
pub fn four_split(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ]
            .as_ref(),
        )
        .split(area)
}

/// Cria um [`Spans`] à partir de uma fita e a posição atual. O valor retornado pode ser
/// usado como um [`Paragraph`](tui::widgets::Paragraph) para exibir o estado da fita em
/// processamento.
pub fn active_tape_spans(tape: &[char], current_position: usize) -> Spans<'static> {
    // tipo de retorno `'static` pois todos Span tem uma String (e não &str)
    let span_vec: Vec<_> = tape
        .iter()
        .enumerate()
        .flat_map(|(i, c)| {
            let style = if i % 2 == 0 {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::Rgb(160, 160, 160))
            };

            if i == current_position {
                let left_bracket = Span::styled("[", style);
                let char = Span::styled(c.to_string(), style);
                let right_bracket = Span::styled("]", style);
                return vec![left_bracket, char, right_bracket];
            } else {
                return vec![Span::styled(c.to_string(), style)];
            }
        })
        .collect();

    Spans(span_vec)
}

/// Constrói um parágrafo para exibir a sétupla.
pub fn septuple_paragraph(sep: &Septuple) -> Paragraph<'static> {
    // tipo de retorno `'static` pois todos Parágrafos tem uma String (e não &str)
    let alphabet = format!("{:?}", sep.alphabet);
    let blank_symbol = sep.blank_symbol.to_string();
    let input_symbols = format!("{:?}", sep.input_symbols);
    let states = format!("{:?}", sep.states);
    let initial_state = sep.initial_state.clone();
    let final_states = format!("{:?}", sep.final_states);
    let mut transition_map = String::new();

    for ((state, symbol), transition) in &sep.transition_map {
        let movement = match transition.move_to {
            Some(side) => match side {
                Movement::R => "R",
                Movement::L => "L",
            },
            None => "-",
        };
        let transition = format!(
            "(δ ({state}, {symbol}) = ({}, {}, {}))\n",
            transition.next_state, transition.write_symbol, movement
        );
        // TODO: verificar performance disso -- muitas strings criadas
        transition_map += &transition;
    }

    let sep_string = format!("Alfabeto: {}\nSímbolo branco: {}\nSímbolos de entrada: {}\nEstados: {}\nEstado inicial: {}\nEstados finais: {}\nTransições:\n{}", 
    alphabet, blank_symbol, input_symbols, states, initial_state, final_states, &transition_map[..]);

    Paragraph::new(sep_string).wrap(Wrap { trim: true })
}

/// Retorna o [`Spans`] usado para representar a fita original. Os valores de cada
/// [`Span`] são *owned*.
pub fn original_tape_spans(tape: &[char]) -> Spans<'static> {
    let spans: Vec<_> = tape
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let style = if i % 2 == 0 {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::Rgb(160, 160, 160))
            };
            Span::styled(c.to_string(), style)
        })
        .collect();
    spans.into()
}

pub fn help_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
}
