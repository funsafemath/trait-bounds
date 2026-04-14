# trait-bounds

This nightly-only library provides useful primitives for defining trait bounds

[`eq`] and [`ne`] functions allow to check whether two types are equal in a `const` context

[`type_id`] is similar to [`core::intrinsics::type_id`], but it can be used on [`PointeeSized`](https://doc.rust-lang.org/nightly/core/marker/trait.PointeeSized.html) types

[`Assert<true>`] implements [`IsTrue`]

[`Assert<false>`] implements [`IsFalse`]

[`Is<T>`] is implemented for `U` if and only if `U` is equal to `T`

[`Not<T>`] is implemented for `U` if and only if `U` is not equal to `T`

The [`Not`] trait can be used to define multiple blanket implementations, which is particularly useful with the [`From<T>`] trait,
as [`From<T> for T`](https://doc.rust-lang.org/nightly/core/convert/trait.From.html#impl-From%3CT%3E-for-T) often collides with user-defined blanket impls

```rust
use trait_bounds::Not;

trait Trait {}

struct A {}
struct B {}
struct C {}

impl Trait for A {}
impl Trait for B {}
impl Trait for C {}

impl From<B> for A {
    fn from(value: B) -> Self {
        Self {}
    }
}

impl<T: Trait + Not<Self> + Not<B>> From<T> for A {
    fn from(value: T) -> Self {
        Self {}
    }
}

fn main() {
    let a: A = A {};
    let b: B = B {};
    let c: C = C {};

    let a: A = a.into(); // core's blanket impl From<T> for T
    let a: A = b.into(); // specific impl From<B> for A
    let a: A = c.into(); // blanket impl From<T: Trait> for A
}
```

```rust
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
```
