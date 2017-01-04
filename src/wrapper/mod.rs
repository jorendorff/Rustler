//! Contains basic wrappers for the Erlang NIF api. Should not be used directly.
//!
//! While the nif_interface module should directly export unsafe nif helper functions,
//! this module should preform validation and make them (reasonably) safe and easy to
//! use from rust. This module should try to be as nonopinionated as possible, and
//! should try to stick as close as possible to the original C api.
//! 
//! Making the apis nice to use from rust should be done in the root rustler crate.


pub mod nif_interface;

pub mod tuple;
pub use self::tuple::{ get_tuple };

pub mod map;
pub use self::map::{ get_map_value, get_map_size, map_new, map_put };

pub mod atom;

pub mod binary;

pub mod exception;

pub mod resource;

pub mod list;

pub mod check;

use std::marker::PhantomData;

use ::wrapper::nif_interface::{ NIF_ENV, NIF_TERM, enif_make_copy };


/// A type representing an Erlang environment.
/// No two environments have the same `EnvId` type.
/// Each `EnvId` type has a different lifetime.
type EnvId<'a> = PhantomData<*mut &'a u8>;

/// `RawNifEnv` is exactly the same as `NIF_ENV`, but it's tagged with a
/// lifetime, to make sure terms are always used with matching environments.
/// It has almost no methods; its only use is to interface safely with unsafe
/// `enif_` code.
///
/// Lifetimes have no effect on the compiled machine code, so this type is just
/// as fast as a `NIF_ENV`. The purpose of adding a lifetime is to have Rust
/// statically check that terms are not used after the associated environment
/// is freed or mixed across environments.
#[derive(Clone, Copy)]
pub struct RawNifEnv<'a>(NIF_ENV, EnvId<'a>);

impl<'a> RawNifEnv<'a> {
    /// Create a new `RawNifEnv` for the given environment.
    ///
    /// # Unsafe
    ///
    /// The caller must ensure that no two `RawNifEnv`s are created with the
    /// same lifetime parameter. Otherwise it would be possible to use terms
    /// after their associated environment is freed, or mix values across
    /// environments, either of which could crash the whole VM.
    pub unsafe fn new(env: NIF_ENV) -> RawNifEnv<'a> {
        RawNifEnv(env, PhantomData)
    }

    pub fn as_c_arg(&self) -> NIF_ENV {
        self.0
    }
}

/// Support comparing two environments even if they do not have the same
/// lifetime.
impl<'a, 'b> PartialEq<RawNifEnv<'b>> for RawNifEnv<'a> {
    fn eq(&self, other: &RawNifEnv<'b>) -> bool {
        self.0 == other.0
    }
}

/// `RawNifTerm` has exactly the same layout as `NIF_TERM` (just an integer),
/// but it's tagged with a lifetime, to make sure we only use terms with
/// matching environments. It has almost no methods; its only use is to
/// interface safely with unsafe `enif_` code.
#[derive(Clone, Copy, PartialEq)]
pub struct RawNifTerm<'a>(NIF_TERM, EnvId<'a>);

impl<'a> RawNifTerm<'a> {
    /// Create a new term.
    ///
    /// # Unsafe
    ///
    /// All terms belong to an environment. The caller must ensure that `term`
    /// belongs to the environment with lifetime `'a`.
    pub unsafe fn new(term: NIF_TERM) -> RawNifTerm<'a> {
        RawNifTerm(term, PhantomData)
    }

    pub fn as_c_arg(&self) -> NIF_TERM {
        self.0
    }
}

pub fn copy_term<'src, 'dst>(dst: RawNifEnv<'dst>, src_term: RawNifTerm<'src>) -> RawNifTerm<'dst> {
    unsafe { RawNifTerm::new(enif_make_copy(dst.0, src_term.0)) }
}
