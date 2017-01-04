use super::{RawNifEnv, RawNifTerm};
use super::nif_interface;
use super::nif_interface::NIF_TERM;
use std::mem;

pub fn get_list_cell<'a>(env: RawNifEnv<'a>, list: RawNifTerm<'a>)
    -> Option<(RawNifTerm<'a>, RawNifTerm<'a>)>
{
    let mut head: NIF_TERM = unsafe { mem::uninitialized() };
    let mut tail: NIF_TERM = unsafe { mem::uninitialized() };
    let success = unsafe { nif_interface::enif_get_list_cell(env.0, list.0, &mut head as *mut NIF_TERM, &mut tail as *mut NIF_TERM) };

    if success != 1 {
        return None;
    }
    unsafe { Some((RawNifTerm::new(head), RawNifTerm::new(tail))) }
}

pub fn get_list_length<'a>(env: RawNifEnv<'a>, list: RawNifTerm<'a>) -> Option<usize> {
    let mut len: u32 = 0;
    let success = unsafe { nif_interface::enif_get_list_length(env.0, list.0, &mut len as *mut u32) };

    if success != 1 {
        return None;
    }
    Some(len as usize)
}

pub fn make_list<'a>(env: RawNifEnv<'a>, arr: &[RawNifTerm<'a>]) -> RawNifTerm<'a> {
    // FIXME?: Should we downcast like this?
    unsafe {
        RawNifTerm::new(nif_interface::enif_make_list_from_array(
            env.0, arr.as_ptr() as *const NIF_TERM, arr.len() as u32))
    }
}

pub fn make_list_cell<'a>(env: RawNifEnv<'a>, head: RawNifTerm<'a>, tail: RawNifTerm<'a>) -> RawNifTerm<'a> {
    unsafe { RawNifTerm::new(nif_interface::enif_make_list_cell(env.0, head.0, tail.0)) }
}

pub fn make_reverse_list<'a>(env: RawNifEnv<'a>, list: RawNifTerm<'a>) -> Option<RawNifTerm<'a>> {
    let mut list_out: NIF_TERM = unsafe { mem::uninitialized() };
    let success = unsafe { nif_interface::enif_make_reverse_list(env.0, list.0, &mut list_out as *mut NIF_TERM) };

    if success != 1 {
        return None;
    }
    unsafe { Some(RawNifTerm::new(list_out)) }
}
