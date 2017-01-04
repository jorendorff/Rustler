use super::{ NifEnv, NifTerm, NifError };
use wrapper::RawNifTerm;

pub fn get_tuple<'a>(term: NifTerm<'a>) -> Result<Vec<NifTerm<'a>>, NifError> {
    let env = term.get_env();
    match ::wrapper::get_tuple(env.raw(), term.raw()) {
        Ok(terms) => Ok(terms.iter().map(|x| NifTerm::new(env, *x)).collect()),
        Err(_error) => Err(NifError::BadArg)
    }
}

pub fn make_tuple<'a>(env: NifEnv<'a>, terms: &[NifTerm<'a>]) -> NifTerm<'a> {
    let c_terms: Vec<RawNifTerm<'a>> = terms.iter().map(|term| term.raw()).collect();
    NifTerm::new(env, ::wrapper::tuple::make_tuple(env.raw(), &c_terms))
}

