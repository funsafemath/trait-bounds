use crate::ne;

/// `true` if `This` and `Other` types are equal, `false` otherwise
pub type const EQ<This, Other>: bool = const {ne::<This, Other>()};

/// `true` if `This` and `Other` types are not equal, `false` otherwise
pub type const NE<This, Other>: bool = const {ne::<This, Other>()};
