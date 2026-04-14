//! This nightly-only library provides useful primitives for defining trait bounds
//!
//! [`eq`] and [`ne`] functions allow to check whether two types are equal in a `const` context
//!
//! [`type_id`] is similar to [`core::intrinsics::type_id`], but it can be used on [`PointeeSized`](core::marker::PointeeSized) types
//!
//! [`Assert<true>`] implements [`IsTrue`]
//!
//! [`Assert<false>`] implements [`IsFalse`]
//!
//! [`Is<T>`] is implemented for `U` if and only if `U` is equal to `T`
//!
//! [`Not<T>`] is implemented for `U` if and only if `U` is not equal to `T`
//!
//! The [`Not`] trait can be used to define multiple blanket implementations, which is particularly useful with the [`From<T>`] trait,
//! as [`From<T> for T`](From<T>#impl-From<T>-for-T) often collides with user-defined blanket impls
//!
//! ```
#![doc = include_str!("../examples/from_many.rs")]
//! ```
//! ```
#![doc = include_str!("../examples/somehow.rs")]
//! ```
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

/// Returns a [`TypeId`] of a given type
///
/// Unlike [`core::intrinsics::type_id`], this can be used on [`PointeeSized`](core::marker::PointeeSized) types
#[must_use]
#[rustc_nounwind]
#[rustc_intrinsic]
pub const fn type_id<T>() -> TypeId;

/// Returns `true` if `This` and `Other` types are equal, `false` otherwise
#[must_use]
pub const fn eq<This, Other>() -> bool {
    type_id_eq(type_id::<This>(), type_id::<Other>())
}

/// Returns `true` if `This` and `Other` types are not equal, `false` otherwise
#[must_use]
pub const fn ne<This, Other>() -> bool {
    !eq::<This, Other>()
}

/// Implements [`IsTrue`] if and only if `COND` is `true`,
/// implements [`IsFalse`] if and only if `COND` is `false`
pub struct Assert<const COND: bool>;

/// Trait used for [`Assert<bool>`] bounds, implemented by [`Assert<true>`] and not implemented by [`Assert<false>`]
#[fundamental]
pub trait IsTrue {}
impl IsTrue for Assert<true> {}

/// Trait used for [`Assert<bool>`] bounds, implemented by [`Assert<false>`] and not implemented by [`Assert<true>`]
#[fundamental]
pub trait IsFalse {}
impl IsFalse for Assert<false> {}

/// [`Is<T>`] is implemented for `U` if and only if `U` is equal to `T`
#[fundamental]
pub trait Is<T> {}
impl<T> Is<T> for T {}

/// [`Not<T>`] is implemented for `U` if and only if `U` is not equal to `T`
#[fundamental]
pub trait Not<T> {}
impl<This, Other> Not<Other> for This where Assert<{ ne::<This, Other>() }>: IsTrue {}
