use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use serde::Deserialize;

use self::json::JsonSeptuple;
pub use self::transition_key::TransitionKey;

/// Uma mapa de transição de estados.
/// A chave é, respectivamente, estado e símbolo. O valor é a transição a ser aplicada.
pub type TransitionMap = HashMap<(String, char), Transition>;

/// A sétupla usada para definir uma Máquina de Turing.
/// <https://en.wikipedia.org/wiki/Turing_machine#Formal_definition>
#[derive(Clone, Debug)]
pub struct Septuple {
    /// Símbolos do alfabeto da fita.
    pub alphabet: HashSet<char>,
    /// Símbolo branco -- o único símbolo que pode ocorrer infinitamente em qualquer etapa
    /// durante a computação
    pub blank_symbol: char,
    /// O conjunto de símbolos que pode estar inicialmente presente na fita.
    pub input_symbols: HashSet<char>,

    /// O conjunto de estados que a máquina pode tomar.
    pub states: HashSet<String>,
    /// O estado inicial da máquina.
    pub initial_state: String,
    /// O conjunto de estados de aceitação da máquina.
    pub final_states: HashSet<String>,

    /// Um _mapa_ de transição, usado para representar a _função_ de transição da máquina.
    pub transition_map: TransitionMap,
}

impl Septuple {
    /// Cria uma sétupla à partir de um JSON.
    pub fn from_json(json: &str) -> Result<Septuple, serde_json::Error> {
        let json_septuple: JsonSeptuple = serde_json::from_str(json)?;
        Ok(Septuple::from(json_septuple))
    }

    /// Verifica se a sétupla é válida. As condições para ela ser inválida são descreitas
    /// pelos membros de [`SepError`].
    pub fn valid(&self) -> Result<(), SepError> {
        if !self.alphabet.contains(&self.blank_symbol) {
            return Err(SepError::BlankNotInAlph);
        }
        if !self.input_symbols.is_subset(&self.alphabet) {
            return Err(SepError::InputNotSubAlph);
        }
        if !self.states.contains(&self.initial_state) {
            return Err(SepError::InitNotInStates);
        }
        if !self.final_states.is_subset(&self.states) {
            return Err(SepError::FinalNotSubStates);
        }

        for key in self.transition_map.keys() {
            let (state, symbol) = key;
            let transition = self.transition_map.get(key).unwrap();

            if !self.states.contains(state) {
                return Err(SepError::TransitionStateNotInStates);
            }
            if !self.alphabet.contains(symbol) {
                return Err(SepError::TransitionSymbolNotInAlphabet);
            }

            if !self.states.contains(&transition.next_state) {
                return Err(SepError::TransitionStateNotInStates);
            }
            if !self.alphabet.contains(&transition.write_symbol) {
                return Err(SepError::TransitionSymbolNotInAlphabet);
            }
        }

        Ok(())
    }
}

/// Define os erros que podem ocorrer durante a validação de uma [`Septuple`].
#[derive(Debug)]
pub enum SepError {
    /// O símbolo branco não está no alfabeto.
    BlankNotInAlph,
    /// O conjunto de _input symbols_ não é um subconjunto do alfabeto.
    InputNotSubAlph,
    /// Estado inicial não está no conjunto de estados.
    InitNotInStates,
    /// O conjunto de estados finais não é um subconjunto dos estados.
    FinalNotSubStates,
    /// Um estado definido nas transições não está no conjunto de estados.
    TransitionStateNotInStates,
    /// Um símbolo definido nas transições não está no alfabeto.
    TransitionSymbolNotInAlphabet,
}

impl Display for SepError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SepError::BlankNotInAlph => write!(f, "símbolo branco não está contido no alfabeto"),
            SepError::InputNotSubAlph => {
                let msg = "alfabeto de símbolos não é subconjunto do alfabeto da fita";
                write!(f, "{msg}")
            }
            SepError::InitNotInStates => {
                write!(f, "estado inicial não está contido no conjunto de estados")
            }
            SepError::FinalNotSubStates => {
                let msg = "conjunto de estados finais não é um subconjunto do conjunto de estados";
                write!(f, "{msg}")
            }
            SepError::TransitionStateNotInStates => {
                let msg = "um estado de transição não está contido no conjunto de estados";
                write!(f, "{msg}")
            }
            SepError::TransitionSymbolNotInAlphabet => {
                write!(f, "um símbolo de transição não está contido no alfabeto")
            }
        }
    }
}

impl std::error::Error for SepError {}

/// Define os movimentos que podem ser tomados em transição.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Deserialize)]
pub enum Movement {
    /// Mover para a direita.
    R,
    /// Mover para a esquerda.
    L,
}

/// Define as ações a serem tomadas na aplicação de uma transição.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Transition {
    pub write_symbol: char,
    pub next_state: String,
    pub move_to: Option<Movement>,
}

mod json {
    //! Módulo da representação em JSON da sétupla de definição da Máquina de Turing.

    use super::{Movement, Septuple};
    use serde::Deserialize;
    use std::collections::HashSet;

    /// Uma estrutura similar à [`Septuple`](super::Septuple), porém editada para
    /// permitir a representação em JSON.
    #[derive(Deserialize)]
    pub struct JsonSeptuple {
        alphabet: HashSet<char>,
        blank_symbol: char,
        input_symbols: HashSet<char>,
        states: HashSet<String>,
        initial_state: String,
        final_states: HashSet<String>,
        transitions: HashSet<Transition>,
    }

    /// Uma estrutura similar à [`Transition`](super::Transition), porém definida com o
    /// estado e símbolo de leitura necessários para aplicar a transição.
    ///
    /// Essa estrutura é necessária para permitir que a tabela de transições seja mapeada
    /// em um JSON. Na sétupla original, o mapa de transições possue uma tupla como sua
    /// chave, porém o JSON só aceita chaves que sejam strings.
    #[derive(Deserialize, Hash, PartialEq, Eq)]
    struct Transition {
        from_state: String,
        read_symbol: char,

        write_symbol: char,
        next_state: String,
        move_to: Option<Movement>,
    }

    impl From<JsonSeptuple> for Septuple {
        fn from(json: JsonSeptuple) -> Self {
            let mut transition_map = super::TransitionMap::new();
            for transition in json.transitions {
                transition_map.insert(
                    (transition.from_state, transition.read_symbol),
                    super::Transition {
                        write_symbol: transition.write_symbol,
                        next_state: transition.next_state,
                        move_to: transition.move_to,
                    },
                );
            }

            Septuple {
                alphabet: json.alphabet,
                blank_symbol: json.blank_symbol,
                input_symbols: json.input_symbols,
                states: json.states,
                initial_state: json.initial_state,
                final_states: json.final_states,
                transition_map,
            }
        }
    }
}

mod transition_key {
    //! Esse módulo exporta a trait `TransitionKey`, implementada em `(String, char)`
    //! e `(&String, &char)`. A implementação é baseada em:
    //! <https://stackoverflow.com/a/45795699/13310655>

    use std::borrow::Borrow;
    use std::hash::Hash;

    /// As implementações dessa trait permitem o uso de uma tupla `(&String, &char)` para
    /// um get em `HashMap<(String, char), _>`.
    ///
    /// Na ausência dessa implementação, todo `get` em um mapa do tipo, iria requerer uma
    /// tupla `&(String, char)`. A tupla teria *ownership* da string, dificultando o uso
    /// de referências de strings para chamadas de `get`.
    ///
    /// A implementação é usada diretamente em [`Machine`](crate::machine::Machine), onde
    /// o processamento das transições possui uma referência mutável à maquina. À partir
    /// dessa referência, é possível obter o estado atual (uma `&String`). A
    /// implementação é usada para permitir o uso desse estado atual na busca no mapa de
    /// transições sem a necessidade de um `.clone()` (algo que seria necessário para
    /// passar uma tupla `&(String, char)`).
    pub trait TransitionKey {
        // ao retornar os próprios dados, é possível implementar `Eq` e `Hash` na
        // trait.
        fn string(&self) -> &String;
        fn char(&self) -> &char;
    }

    impl TransitionKey for (String, char) {
        fn string(&self) -> &String {
            &self.0
        }

        fn char(&self) -> &char {
            &self.1
        }
    }

    impl TransitionKey for (&String, &char) {
        fn string(&self) -> &String {
            self.0
        }

        fn char(&self) -> &char {
            self.1
        }
    }

    impl<'a> Borrow<dyn TransitionKey + 'a> for (String, char) {
        fn borrow(&self) -> &(dyn TransitionKey + 'a) {
            self
        }
    }

    impl Hash for dyn TransitionKey + '_ {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            self.string().hash(state);
            self.char().hash(state);
        }
    }

    impl PartialEq for dyn TransitionKey + '_ {
        fn eq(&self, other: &Self) -> bool {
            self.string() == other.string() && self.char() == other.char()
        }
    }

    impl Eq for dyn TransitionKey + '_ {}
}
