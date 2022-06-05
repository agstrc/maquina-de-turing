pub mod draw;
pub mod machine;
pub mod state;

use std::{error, result};

/// Representa qualquer erro.
pub type Error = Box<dyn error::Error>;

/// Define um [`Result`](result::Result) com qualquer tipo de erro.
pub type Result<T> = result::Result<T, Error>;

/// Representa qualquer um dos tipos.
pub enum Either<L, R> {
    L(L),
    R(R),
}
