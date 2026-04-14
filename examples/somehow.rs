use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
    fs::File,
};

use trait_bounds::Not;

#[derive(Debug)]
struct Somehow {
    inner: Box<dyn Error>,
}

impl Display for Somehow {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

// note that it does implement Error, unlike anyhow/eyre error types
impl Error for Somehow {}

// and excluding Self from the trait bound allows to implement a blanket From<E: Error>, even though a blanket impl From<T> for T is defined in the core
impl<E: Error + Not<Self> + 'static> From<E> for Somehow {
    fn from(value: E) -> Self {
        Self {
            inner: Box::new(value),
        }
    }
}

fn fallible() -> Result<(), Somehow> {
    File::open("/nix")?;
    Ok(())
}

fn main() -> Result<(), Somehow> {
    fallible()?;
    Ok(())
}
