use super::NifEnv;
use super::wrapper::nif_interface::enif_consume_timeslice;

pub fn consume_timeslice<'a>(env: NifEnv<'a>, percent: i32) -> bool {
    let success = unsafe { enif_consume_timeslice(env.raw().as_c_arg(), percent) };
    success == 1
}
