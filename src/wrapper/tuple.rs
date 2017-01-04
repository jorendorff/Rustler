use super::{RawNifEnv, RawNifTerm, nif_interface};
use super::nif_interface::{ c_int, NIF_TERM, NIF_ERROR };
use std::mem;

pub fn get_tuple<'a>(env: RawNifEnv<'a>, term: RawNifTerm<'a>) -> Result<&'a [RawNifTerm<'a>], NIF_ERROR> {
    let mut arity: c_int = 0;
    let mut array_ptr: *const NIF_TERM = unsafe { mem::uninitialized() };
    let success = unsafe { nif_interface::enif_get_tuple(env.0, term.0,
                                                         &mut arity as *mut c_int,
                                                         &mut array_ptr as *mut *const NIF_TERM) };
    if success != 1 {
        return Err(NIF_ERROR::BAD_ARG);
    }
    let term_array = unsafe { ::std::slice::from_raw_parts(array_ptr as *const RawNifTerm<'a>, arity as usize) };
    Ok(term_array)
}

pub fn make_tuple<'a>(env: RawNifEnv<'a>, terms: &[RawNifTerm<'a>]) -> RawNifTerm<'a> {
    unsafe {
        RawNifTerm::new(
            nif_interface::enif_make_tuple_from_array(env.0, terms.as_ptr() as *const NIF_TERM, terms.len() as u32))
    }
}
