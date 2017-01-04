use super::{ RawNifEnv, RawNifTerm };
use super::nif_interface;
use super::nif_interface::NIF_TERM;

/// Tell the Erlang VM to raise an error exception with the term `reason` when
/// the current NIF returns.
///
/// The return value from `raise_exception` can only be used as the return
/// value from the NIF that invoked it. It must not be passed to any other
/// function.
///
pub fn raise_exception<'a>(env: RawNifEnv<'a>, reason: RawNifTerm<'a>) -> NIF_TERM {
    unsafe { nif_interface::enif_raise_exception(env.0, reason.0) }
}

/// Tell the Erlang VM to raise a `badarg` exception when the current NIF returns.
///
/// The return value from `raise_badarg` can only be used as the return
/// value from the NIF that invoked it. It must not be passed to any other
/// function.
///
pub fn raise_badarg<'a>(env: RawNifEnv<'a>) -> NIF_TERM {
    unsafe { nif_interface::enif_make_badarg(env.0) }
}
