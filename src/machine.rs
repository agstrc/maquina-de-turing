#[cfg(test)]
mod test;

pub mod sep;

use std::fmt::Display;

use self::sep::{Movement, Septuple, Transition, TransitionKey};

/// Uma Máquina de Turing, finita à esquerda.
#[derive(Debug, Clone)]
pub struct Machine<'machine> {
    septuple: &'machine Septuple,

    current_position: usize,
    current_state: &'machine String,
    tape: Vec<char>,

    undos: Vec<Undo<'machine>>,
}

/// Define aceitação ou rejeição de uma fita para uma Máquina de Turing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acceptance {
    Accepted,
    Rejected,
}

/// Representa um erro retornado quando uma nova máquina possui uma fita com símbolos
/// incompatíveis com sua sétupla.
#[derive(Debug)]
pub struct InvalidSymbolError;

impl Display for InvalidSymbolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tape contains invalid symbols")
    }
}
impl std::error::Error for InvalidSymbolError {}

/// Representa um erro retornado quando não há transições para serem desfeitas em uma
/// máquina.
#[derive(Debug)]
pub struct NoUndoError;

impl Display for NoUndoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "nothing to undo")
    }
}
impl std::error::Error for NoUndoError {}

impl Machine<'_> {
    /// Inicializa uma nova Máquina de Turing. É assumido que `septuple` já foi validada.
    ///
    /// # Erros
    /// Retorna um erro caso a fita possua símbolos não contidos no alfabeto.
    pub fn new(septuple: &Septuple, mut tape: Vec<char>) -> Result<Machine, InvalidSymbolError> {
        let has_invalid_symbol = tape
            .iter()
            .any(|symbol| !septuple.input_symbols.contains(symbol));
        if has_invalid_symbol {
            return Err(InvalidSymbolError);
        }

        if tape.is_empty() {
            tape.push(septuple.blank_symbol);
        }

        Ok(Machine {
            tape,
            septuple,
            current_position: 0,
            current_state: &septuple.initial_state,
            undos: vec![],
        })
    }

    /// Aplica a transição adequada para o estado atual da máquina. Se o estado atual
    /// indicar uma aceitação ou rejeição, retorna [`Some`]. Caso a máquina ainda esteja
    /// processando a fita, retorna [`None`].
    pub fn transition(&mut self) -> Option<Acceptance> {
        if self.in_final_state() {
            return Some(Acceptance::Accepted);
        }

        let transition = match self.get_transition() {
            Some(transition) => transition,
            None => return Some(Acceptance::Rejected), // não há transições para o estado atual
        };

        if self.limited_left(transition) {
            return Some(Acceptance::Rejected);
        }

        let undo = self.apply(transition);
        self.undos.push(undo);
        None
    }

    /// Desfaz a última transição aplicada na máquina.
    /// Caso não haja transição para ser desfeita, retornar [`Err`].
    pub fn undo_transition(&mut self) -> Result<(), NoUndoError> {
        let undo = match self.undos.pop() {
            Some(undo) => undo,
            None => return Err(NoUndoError),
        };
        if undo.pop {
            self.tape.pop();
        }
        if let Some(movement) = undo.movement {
            match movement {
                Movement::R => self.current_position += 1,
                Movement::L => self.current_position -= 1,
            }
        }
        self.tape[self.current_position] = undo.write;
        self.current_state = undo.state;

        Ok(())
    }

    /// Retorna o estado de aceitação de máquina. Caso seja [`None`], a máquina ainda
    /// está em processamento.
    pub fn acceptance(&self) -> Option<Acceptance> {
        if self.in_final_state() {
            return Some(Acceptance::Accepted);
        }
        let transition = match self.get_transition() {
            Some(transition) => transition,
            None => return Some(Acceptance::Rejected),
        };
        if self.limited_left(transition) {
            return Some(Acceptance::Rejected);
        }
        None
    }

    /// Retorna `true` caso a máquina esteja atualmente "limitada pela esquerda", isso é,
    /// caso ela esteja na posição zero da fita e tenha como próxima etapa uma transição
    /// com movimento para a esquerda.
    fn limited_left(&self, transition: &Transition) -> bool {
        if self.current_position == 0 {
            let movement = match transition.move_to {
                Some(movement) => movement,
                None => return false,
            };
            if movement == Movement::L {
                return true;
            }
        }
        false
    }

    /// Retorna `true` se a máquina estiver em um estado final.
    fn in_final_state(&self) -> bool {
        self.septuple.final_states.contains(self.current_state)
    }

    // --- getters

    pub fn septuple(&self) -> &Septuple {
        self.septuple
    }

    pub fn current_position(&self) -> usize {
        self.current_position
    }

    pub fn current_state(&self) -> &String {
        self.current_state
    }

    pub fn tape(&self) -> &Vec<char> {
        &self.tape
    }
}

impl<'machine> Machine<'machine> {
    /// Retorna, caso exista, a transição para o estado atual da máquina.
    fn get_transition(&self) -> Option<&'machine Transition> {
        let current_symbol = self.tape[self.current_position];
        let transition_key = (self.current_state, &current_symbol);
        self.septuple
            .transition_map
            .get(&transition_key as &dyn TransitionKey)
    }

    /// Aplica a transição encontrada e retorna um [`Undo`] equivalente.
    ///
    /// Esse método não deve ser chamado quando a fita estiver na posição 0 e a transição
    /// representar um movimento à esquerda, pois isso causará um underflow no atributo
    /// `current_position` da máquina.
    fn apply(&mut self, transition: &'machine Transition) -> Undo<'machine> {
        // inicializa as variáveis de construção do Undo
        let mut undo_pop = false;
        let mut undo_movement: Option<Movement> = None;
        let undo_write = self.tape[self.current_position];
        let undo_state = self.current_state;

        self.tape[self.current_position] = transition.write_symbol;
        self.current_state = &transition.next_state;
        match transition.move_to {
            Some(movement) => match movement {
                Movement::R => {
                    if self.current_position == self.tape.len() - 1 {
                        undo_pop = true;
                        self.tape.push(self.septuple.blank_symbol);
                    }
                    undo_movement = Some(Movement::L);
                    self.current_position += 1;
                }
                Movement::L => {
                    undo_movement = Some(Movement::R);
                    self.current_position -= 1;
                }
            },
            None => { /* nada a se fazer */ }
        }

        Undo {
            pop: undo_pop,
            movement: undo_movement,
            write: undo_write,
            state: undo_state,
        }
    }
}

/// Define os passos necessários para desfazer uma transição.
/// Devido à natureza da ação "undo", é esperado que um `Undo` sempre declare apenas
/// ações válidas.
#[derive(Debug, Clone)]
struct Undo<'u> {
    /// Caso `true`, um símbolo branco foi adicionado na última transição.
    pop: bool,
    /// Caso `Some`, é o movimento oposto da última transição.
    movement: Option<Movement>,
    /// Indica o char a ser escrito na fita **após** desfazer o último movimento.
    write: char,
    /// Indica o estado da máquina antes da última transição.
    state: &'u String,
}
