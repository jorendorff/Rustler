use super::nif_interface;
use super::{RawNifEnv, RawNifTerm};
use super::nif_interface::{ NIF_TERM, ErlNifMapIteratorEntry };
pub use super::nif_interface::ErlNifMapIterator;
use std::mem;
use std::marker::PhantomData;

pub fn get_map_value<'a>(env: RawNifEnv<'a>, map: RawNifTerm<'a>, key: RawNifTerm<'a>) -> Option<RawNifTerm<'a>> {
    let mut result: NIF_TERM = unsafe { mem::uninitialized() };
    let success = unsafe { nif_interface::enif_get_map_value(env.0, map.0, key.0, &mut result as *mut NIF_TERM) };

    if success != 1 {
        return None;
    }
    Some(unsafe { RawNifTerm::new(result) })
}

pub fn get_map_size<'a>(env: RawNifEnv<'a>, map: RawNifTerm<'a>) -> Option<usize> {
    let mut size: nif_interface::size_t = unsafe { mem::uninitialized() };
    let success = unsafe { nif_interface::enif_get_map_size(env.0, map.0, &mut size as *mut nif_interface::size_t) };

    if success != 1 {
        return None;
    }
    Some(size as usize)
}

pub fn map_new<'a>(env: RawNifEnv<'a>) -> RawNifTerm<'a> {
    unsafe { RawNifTerm::new(nif_interface::enif_make_new_map(env.0)) }
}

pub fn map_put<'a>(env: RawNifEnv<'a>, map: RawNifTerm<'a>, key: RawNifTerm<'a>, value: RawNifTerm<'a>) -> Option<RawNifTerm<'a>> {
    let mut result: NIF_TERM = unsafe { mem::uninitialized() };
    let success = unsafe { nif_interface::enif_make_map_put(env.0, map.0, key.0, value.0, &mut result as *mut NIF_TERM) };

    if success != 1 {
        return None;
    }
    Some(unsafe { RawNifTerm::new(result) })
}

pub fn map_remove<'a>(env: RawNifEnv<'a>, map: RawNifTerm<'a>, key: RawNifTerm<'a>) -> Option<RawNifTerm<'a>> {
    let mut result: NIF_TERM = unsafe { mem::uninitialized() };
    let success = unsafe { nif_interface::enif_make_map_remove(env.0, map.0, key.0, &mut result as *mut NIF_TERM) };

    if success != 1 {
        return None;
    }
    Some(unsafe { RawNifTerm::new(result) })
}

pub fn map_update<'a>(env: RawNifEnv<'a>, map: RawNifTerm<'a>, key: RawNifTerm<'a>, new_value: RawNifTerm<'a>) -> Option<RawNifTerm<'a>> {
    let mut result: NIF_TERM = unsafe { mem::uninitialized() };
    let success = unsafe {
        nif_interface::enif_make_map_update(env.0, map.0, key.0, new_value.0, &mut result as *mut NIF_TERM)
    };

    if success != 1 {
        return None;
    }
    Some(unsafe { RawNifTerm::new(result) })
}

pub struct RawNifMapIterator<'a>(
    ErlNifMapIterator,
    PhantomData<&'a u8>);

pub fn map_iterator_create<'a>(env: RawNifEnv<'a>, map: RawNifTerm<'a>) -> Option<RawNifMapIterator<'a>> {
    let mut iter = unsafe { mem::uninitialized() };
    let success = unsafe {
        nif_interface::enif_map_iterator_create(env.0, map.0, &mut iter,
                                                ErlNifMapIteratorEntry::ERL_NIF_MAP_ITERATOR_HEAD)
    };
    if success != 1 {
        return None;
    }
    Some(RawNifMapIterator(iter, PhantomData))
}

pub unsafe fn map_iterator_destroy<'a>(env: RawNifEnv<'a>, iter: &mut RawNifMapIterator<'a>) {
    nif_interface::enif_map_iterator_destroy(env.0, &mut iter.0);
}

pub fn map_iterator_get_pair<'a>(env: RawNifEnv<'a>, iter: &mut RawNifMapIterator<'a>)
    -> Option<(RawNifTerm<'a>, RawNifTerm<'a>)>
{
    let mut key: NIF_TERM = unsafe { mem::uninitialized() };
    let mut value: NIF_TERM = unsafe { mem::uninitialized() };
    let success = unsafe { nif_interface::enif_map_iterator_get_pair(env.0, &mut iter.0, &mut key, &mut value) };
    if success != 1 {
        return None;
    }
    Some(unsafe { (RawNifTerm::new(key), RawNifTerm::new(value)) })
}

pub fn map_iterator_next<'a>(env: RawNifEnv<'a>, iter: &mut RawNifMapIterator<'a>) {
    unsafe {
        nif_interface::enif_map_iterator_next(env.0, &mut iter.0);
    }
}
