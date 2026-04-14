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
