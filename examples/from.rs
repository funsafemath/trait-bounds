use trait_bounds::Not;

trait Trait {}

struct A {}
struct B {}

impl Trait for A {}
impl Trait for B {}

impl<T: Trait + Not<Self>> From<T> for A {
    fn from(value: T) -> Self {
        Self {}
    }
}

fn main() {
    let a: A = A {};
    let b: B = B {};

    let a: A = a.into(); // core's blanket impl From<T> for T
    let a: A = b.into(); // blanket impl From<T: Trait> for A
}
