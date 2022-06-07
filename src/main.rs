use std::{env, fs, io, process};

use crossterm::{cursor, terminal};
use tm::{
    machine::{sep::Septuple, Machine},
    state, Result,
};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Passe como argumento apenas o path do arquivo da sétupla");
        process::exit(1);
    }
    let file_path = &args[1];
    let file_contents = fs::read_to_string(file_path)?;
    let sep = Septuple::from_json(&file_contents)?;
    if let Err(err) = sep.valid() {
        eprintln!("Erro de definição da sétupla: {err}");
        process::exit(1);
    }

    if let Err(err) = run_tui(sep) {
        eprintln!("Um erro ocorreu: {err}");
        process::exit(1)
    }
    Ok(())
}

fn run_tui(sep: Septuple) -> Result<()> {
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
    let backend = CrosstermBackend::new(stdout);
    let term = Terminal::new(backend)?;

    let mut app = App {
        term,
        sep: &sep,
        machine: None,
    };

    // loop principal -- muda de estado até o usuário sair do programa
    let mut func = StateFunction(read_tape);
    loop {
        func = match func.0(&mut app)? {
            Some(func) => func,
            None => break,
        }
    }

    crossterm::execute!(
        app.term.backend_mut(),
        terminal::LeaveAlternateScreen,
        cursor::Show
    )?;
    terminal::disable_raw_mode()?;
    Ok(())
}

/// Representa uma função de estado. Esse tipo de função toma como argumento uma
/// referência mutável de [`App`] e retorna, embrulhado em um [`Result`], uma função
/// opcional do mesmo tipo.
struct StateFunction<B: Backend>(fn(&mut App<B>) -> Result<Option<StateFunction<B>>>);

/// Mantém algumas variáveis que podem ser alteradas pelos estados.
struct App<'app, B: Backend> {
    term: Terminal<B>,
    sep: &'app Septuple,
    // opcional pois a máquina é inicializada depois
    machine: Option<Machine<'app>>,
}

/// Lê o input para a fita até receber uma fita válida ou sair do programa.
fn read_tape<B: Backend>(app: &mut App<B>) -> Result<Option<StateFunction<B>>> {
    match state::read_valid_tape(&mut app.term, &app.sep.input_symbols)? {
        tm::Either::L(tape) => {
            app.machine = Some(Machine::new(app.sep, tape).unwrap());

            // "transforma" [`process_machine`] em uma função do tipo correto.
            let alias: StateFunction<B> = StateFunction(process_machine);

            Ok(Some(alias))
        }
        tm::Either::R(_) => Ok(None),
    }
}

/// Processa o input da máquina até o usuário retornar à tela de input de fita.
fn process_machine<B: Backend>(app: &mut App<B>) -> Result<Option<StateFunction<B>>> {
    // essa função só será chamada depois da máquina necessariamente ter sido
    // criada; portanto unwrap é seguro.
    let machine = app.machine.as_mut().unwrap();
    state::process_machine(&mut app.term, machine)?;

    // "transforma" [`process_machine`] em uma função do tipo correto.
    let alias: StateFunction<B> = StateFunction(read_tape);
    Ok(Some(alias))
}
