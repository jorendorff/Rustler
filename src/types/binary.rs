use super::{ NifEnv, NifResult, NifTerm, NifEncoder, NifDecoder };
use ::NifError;
use std::mem;
use std::ptr;
use ::wrapper::binary::{self, ErlNifBinary};

pub struct OwnedNifBinary {
    inner: ErlNifBinary,
}

pub struct NifBinary<'a> {
    inner: ErlNifBinary,
    term: NifTerm<'a>,
}

impl Drop for OwnedNifBinary {
    fn drop(&mut self) {
        unsafe {
            binary::release_binary(&mut self.inner);
        }
    }
}

impl OwnedNifBinary {
    pub fn alloc(size: usize) -> Option<OwnedNifBinary> {
        binary::alloc_binary(size)
            .map(|binary| {
                OwnedNifBinary {
                    inner: binary,
                }
            })
    }

    pub fn as_slice(&self) -> &[u8] {
        unsafe { ::std::slice::from_raw_parts(self.inner.data, self.inner.size as usize) }
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { ::std::slice::from_raw_parts_mut(self.inner.data, self.inner.size as usize) }
    }

    pub fn release<'a>(self, env: NifEnv<'a>) -> NifBinary<'a> {
        NifBinary::from_owned(self, env)
    }
}

impl<'a> NifBinary<'a> {
    pub fn from_owned(mut bin: OwnedNifBinary, env: NifEnv<'a>) -> Self {
        let term = NifTerm::new(env, unsafe { binary::make_binary(env.raw(), &mut bin.inner) });
        let new_bin = NifBinary {
            inner: unsafe { ptr::read(&bin.inner) },
            term: term,
        };
        mem::forget(bin);
        new_bin
    }

    pub fn from_term(term: NifTerm<'a>) -> Result<Self, NifError> {
        match binary::inspect_binary(term.get_env().raw(), term.raw()) {
            None => Err(NifError::BadArg),
            Some(binary) => Ok(NifBinary {
                inner: binary,
                term: term,
            })
        }
    }

    pub fn as_slice(&self) -> &'a [u8] {
        unsafe { ::std::slice::from_raw_parts(self.inner.data, self.inner.size as usize) }
    }

    pub fn get_term<'b>(&self, env: NifEnv<'b>) -> NifTerm<'b> {
        self.term.in_env(env)
    }

    pub fn make_subbinary(&self, offset: usize, length: usize) -> NifResult<NifBinary<'a>> {
        let min_len = length.checked_add(offset);
        if try!(min_len.ok_or(NifError::BadArg)) > self.inner.size {
            return Err(NifError::BadArg);
        }

        let raw_term = unsafe {
            binary::make_sub_binary(self.term.get_env().raw(), self.term.raw(), offset, length)
        };

        // This should never fail, as we are always passing in a binary term.
        Ok(NifBinary::from_term(NifTerm::new(self.term.get_env(), raw_term)).ok().unwrap())
    }
}

impl<'a> NifDecoder<'a> for NifBinary<'a> {
    fn decode(term: NifTerm<'a>) -> Result<Self, NifError> {
        NifBinary::from_term(term)
    }
}

impl<'a> NifEncoder for NifBinary<'a> {
    fn encode<'b>(&self, env: NifEnv<'b>) -> NifTerm<'b> {
        self.get_term(env)
    }
}

/// ## Binary terms
impl<'a> NifTerm<'a> {

    pub fn into_binary(self) -> NifResult<NifBinary<'a>> {
        NifBinary::from_term(self)
    }

}
