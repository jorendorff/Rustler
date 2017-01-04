///! Functions for creating and accessing Erlang terms of type binary.

use std::mem;
use super::nif_interface::{ self, size_t };
pub use super::nif_interface::ErlNifBinary;
use super::{RawNifEnv, RawNifTerm};

/// Allocate a new binary of `size` bytes. The binary must either be released with `release_binary`
/// or ownership transferred to an Erlang term with `make_binary`. An allocated (and owned)
/// `ErlNifBinary` can be kept between NIF calls.
pub fn alloc_binary(size: usize) -> Option<ErlNifBinary> {
    let mut binary: ErlNifBinary = unsafe { mem::uninitialized() };
    if unsafe { nif_interface::enif_alloc_binary(size as size_t, &mut binary) } == 0 {
        None
    } else {
        Some(binary)
    }
}

/// Get read-only access to the data in the binary `bin_term`.  Returns `None` if `bin_term` is not
/// a binary.
///
/// The resulting binary must be treated as read-only and used only during `env`'s lifetime.
pub fn inspect_binary<'a>(env: RawNifEnv<'a>, bin_term: RawNifTerm<'a>) -> Option<ErlNifBinary> {
    let mut binary: ErlNifBinary = unsafe { mem::uninitialized() };
    if unsafe { nif_interface::enif_inspect_binary(env.0, bin_term.0, &mut binary) } == 0 {
        None
    } else {
        Some(binary)
    }
}

/// Make a new term that's a subbinary of `bin_term`, starting at offset `pos` with length `size`
/// bytes.
///
/// # Unsafe
///
/// `bin_term` must be a binary or bitstring. `pos + size` must be `<=` the length of `bin_term` in
/// bytes.
pub unsafe fn make_sub_binary<'a>(env: RawNifEnv<'a>, bin_term: RawNifTerm<'a>, pos: usize, size: usize)
    -> RawNifTerm<'a>
{
    RawNifTerm::new(nif_interface::enif_make_sub_binary(env.0, bin_term.0, pos as size_t, size as size_t))
}

/// Make a binary term from the data pointed to by `binary`. Ownership of the data is transferred
/// to the new term.
///
/// # Unsafe
///
/// `binary` must be treated as read-only afterwards, for the lifetime of `env`, and it must not be
/// used after that.
pub unsafe fn make_binary<'a>(env: RawNifEnv<'a>, binary: &mut ErlNifBinary) -> RawNifTerm<'a> {
    RawNifTerm::new(nif_interface::enif_make_binary(env.0, binary))
}

/// Free a binary allocated using `enif_alloc_binary`.
///
/// # Unsafe
///
/// The binary must be one created with `alloc_binary`, not `inspect_binary`.  And of course it
/// must not used again afterwards.
pub unsafe fn release_binary(binary: &mut ErlNifBinary) {
    nif_interface::enif_release_binary(binary);
}
