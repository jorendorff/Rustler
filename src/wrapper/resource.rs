use super::{RawNifEnv, RawNifTerm};
use super::nif_interface;
use super::nif_interface::{ NifResourceDtor, NifResourceFlags, NIF_RESOURCE_TYPE, NIF_RESOURCE_HANDLE };

use std::ffi::CString;
use std::mem;
use std::ptr;

pub fn open_resource_type<'a>(env: RawNifEnv<'a>, name: &str, dtor: Option<NifResourceDtor>, flags: NifResourceFlags
                          ) -> Option<NIF_RESOURCE_TYPE> {
    // Currently unused as per erlang nif documentation
    let module_p: *const u8 = ptr::null();
    let name_p: *const u8 = CString::new(name).unwrap().as_bytes_with_nul().as_ptr();
    let res = unsafe {
        let mut tried: NifResourceFlags = mem::uninitialized();
        nif_interface::enif_open_resource_type(env.0, module_p, name_p, dtor, flags,
                                               (&mut tried as *mut NifResourceFlags))
    };

    if res.is_null() {
        None
    } else {
        Some(res)
    }
}

// Functionally incomplete
pub fn get_resource<'a>(env: RawNifEnv<'a>, term: RawNifTerm<'a>, typ: NIF_RESOURCE_TYPE) -> Option<NIF_RESOURCE_HANDLE> {
    unsafe {
        let mut ret_obj: NIF_RESOURCE_HANDLE = mem::uninitialized();
        let res = nif_interface::enif_get_resource(env.0, term.0, typ, &mut ret_obj as *mut NIF_RESOURCE_HANDLE);

        if res == 0 {
            None
        } else {
            Some(ret_obj)
        }
    }
}

pub fn alloc_resource(typ: NIF_RESOURCE_TYPE, size: usize) -> NIF_RESOURCE_HANDLE {
    unsafe { nif_interface::enif_alloc_resource(typ, size) }
}

pub fn make_resource<'a>(env: RawNifEnv<'a>, obj: NIF_RESOURCE_HANDLE) -> RawNifTerm<'a> {
    unsafe { RawNifTerm::new(nif_interface::enif_make_resource(env.0, obj)) }
}

pub fn release_resource(obj: NIF_RESOURCE_HANDLE) {
    unsafe { nif_interface::enif_release_resource(obj) }
}

pub fn keep_resource(obj: NIF_RESOURCE_HANDLE) {
    unsafe { nif_interface::enif_keep_resource(obj) }
}
