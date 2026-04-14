#![no_std]
#![expect(internal_features)]
#![expect(incomplete_features)]
#![feature(core_intrinsics)]
#![feature(generic_const_exprs)]
#![feature(fundamental)]
#![feature(rustc_attrs)]
#![feature(intrinsics)]
#![rustc_no_implicit_bounds]

use core::{any::TypeId, intrinsics::type_id_eq};

// T is : ?Sized in core's type_id
#[must_use]
#[rustc_nounwind]
#[rustc_intrinsic]
pub const fn type_id<T>() -> TypeId;

#[must_use]
pub const fn eq<This, Other>() -> bool {
    type_id_eq(type_id::<This>(), type_id::<Other>())
}

#[must_use]
pub const fn ne<This, Other>() -> bool {
    !eq::<This, Other>()
}

pub struct Assert<const COND: bool>;

#[fundamental]
pub trait IsTrue {}
impl IsTrue for Assert<true> {}

#[fundamental]
pub trait IsFalse {}
impl IsFalse for Assert<false> {}

#[fundamental]
pub trait Is<T> {}
impl<T> Is<T> for T {}

#[fundamental]
pub trait Not<T> {}
impl<This, Other> Not<Other> for This where Assert<{ ne::<This, Other>() }>: IsTrue {}
