mod aux;

use std::collections::HashSet;

use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
    Frame,
};

use crate::machine::{Acceptance, Machine};

pub use aux::original_tape_spans;

/// Desenha a tela para o input da fita.
pub fn tape_input<B: Backend>(
    frame: &mut Frame<B>,
    tape_buffer: &str,
    input_symbols: &HashSet<char>,
) {
    let tape_outer_block = Block::default()
        .title("Digite a fita")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let chunks = Layout::default()
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let bold = Style::default().add_modifier(Modifier::BOLD);
    let gray_background = Style::default().bg(Color::Gray);
    let tape_spans = vec![
        Span::styled(tape_buffer, bold),
        Span::styled(" ", gray_background), // "cursor"
    ];
    let tape_spans = Spans(tape_spans);

    let tape_paragraph = Paragraph::new(tape_spans).block(tape_outer_block);
    let help_spans = vec![
        Span::styled("<Esc> ", Style::default().fg(Color::Rgb(255, 140, 0))),
        Span::from("sair "),
        Span::styled("<Enter> ", Style::default().fg(Color::Rgb(255, 140, 0))),
        Span::from("continuar"),
    ];

    frame.render_widget(tape_paragraph, chunks[0]);
    let alphabet_paragraph =
        Paragraph::new(format!("Símbolos de entrada: {input_symbols:?}")).wrap(Wrap { trim: true });
    frame.render_widget(alphabet_paragraph, chunks[1]);
    let help_paragraph = Paragraph::new(Spans(help_spans)).block(aux::help_block());
    frame.render_widget(help_paragraph, chunks[2]);
}

/// Desenha a tela de erro de input da fita.
pub fn bad_tape<B: Backend>(frame: &mut Frame<B>, input_symbols: &HashSet<char>) {
    let message_outer_block = Block::default()
        .title("Digite a fita")
        .title_alignment(Alignment::Left)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red));

    let chunks = Layout::default()
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.size());

    let message_style = Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);
    let error_paragraph = Paragraph::new("A fita digitada possui símbolos inválidos")
        .style(message_style)
        .block(message_outer_block);

    let help_spans = vec![
        Span::styled("<Esc> ", Style::default().fg(Color::Rgb(255, 140, 0))),
        Span::from("voltar "),
    ];

    frame.render_widget(error_paragraph, chunks[0]);
    let alphabet_paragraph =
        Paragraph::new(format!("Símbolos de entrada: {input_symbols:?}")).wrap(Wrap { trim: true });
    frame.render_widget(alphabet_paragraph, chunks[1]);
    let help_paragraph = Paragraph::new(Spans(help_spans)).block(aux::help_block());
    frame.render_widget(help_paragraph, chunks[2]);
}

/// Desenha a tela de processamento da máquina atual.
pub fn machine<B: Backend>(
    frame: &mut Frame<B>,
    machine: &Machine,
    acceptance: Option<Acceptance>,
    original_tape: Spans,
) {
    let outer_block = Block::default()
        .title("Máquina de Turing")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL);
    frame.render_widget(outer_block, frame.size());

    let original_tape_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Double)
        .title("Fita original")
        .title_alignment(Alignment::Left);

    let screen_chunks = aux::four_split(frame.size());
    let active_tape = aux::active_tape_spans(machine.tape(), machine.current_position());
    let mut tape_title = vec![Span::from(format!("Fita @ {}", machine.current_state()))];
    if let Some(accept) = acceptance {
        match accept {
            Acceptance::Accepted => tape_title.push(Span::styled(
                " aceitada",
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD),
            )),
            Acceptance::Rejected => tape_title.push(Span::styled(
                " rejeitada",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )),
        }
    }
    let help_spans = vec![
        Span::styled("<Esc> ", Style::default().fg(Color::Rgb(255, 140, 0))),
        Span::from("voltar "),
        Span::styled("< ⟵ > ", Style::default().fg(Color::Rgb(255, 140, 0))),
        Span::from("desfazer transição "),
        Span::styled("< ⟶ > ", Style::default().fg(Color::Rgb(255, 140, 0))),
        Span::from("aplicar transição "),
    ];
    let help_paragraph = Paragraph::new(Spans(help_spans)).block(aux::help_block());

    let tape_title = Spans::from(tape_title);
    let tape_block = original_tape_block.clone().title(tape_title);

    let active_tape = Paragraph::new(active_tape).block(tape_block);
    frame.render_widget(active_tape, screen_chunks[0]);
    let original_tape = Paragraph::new(original_tape).block(original_tape_block);
    frame.render_widget(original_tape, screen_chunks[1]);
    frame.render_widget(
        aux::septuple_paragraph(machine.septuple()),
        screen_chunks[2],
    );
    frame.render_widget(help_paragraph, screen_chunks[3]);
}
