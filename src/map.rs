//! Utilities used to access and create Erlang maps.

use super::{ NifEnv, NifTerm, NifResult, NifError, NifDecoder };
use ::wrapper::map;

pub fn map_new<'a>(env: NifEnv<'a>) -> NifTerm<'a> {
    NifTerm::new(env, map::map_new(env.raw()))
}

/// ## Map terms
impl<'a> NifTerm<'a> {

    /// Gets the value corresponding to a key in a map term.
    ///
    /// Returns Err(NifError::BadArg) if the term is not a map or if
    /// key doesn't exist in the map.
    ///
    /// ### Elixir equivalent
    /// ```elixir
    /// Map.get(self_term, key)
    /// ```
    pub fn map_get(self, key: NifTerm<'a>) -> NifResult<NifTerm<'a>> {
        let env = self.get_env();
        match ::wrapper::get_map_value(env.raw(), self.raw(), key.raw()) {
            Some(value) => Ok(NifTerm::new(env, value)),
            None => Err(NifError::BadArg),
        }
    }

    /// Gets the size of a map term.
    ///
    /// Returns Err(NifError::BadArg) if the term is not a map.
    ///
    /// ### Elixir equivalent
    /// ```elixir
    /// map_size(self_term)
    /// ```
    pub fn map_size(self) -> NifResult<usize> {
        let env = self.get_env();
        map::get_map_size(env.raw(), self.raw()).ok_or(NifError::BadArg)
    }

    /// Makes a copy of the self map term and sets key to value.
    /// If the value already exists, it is overwritten.
    ///
    /// Returns Err(NifError::BadArg) if the term is not a map.
    ///
    /// ### Elixir equivalent
    /// ```elixir
    /// Map.put(self_term, key, value)
    /// ```
    pub fn map_put(self, key: NifTerm<'a>, value: NifTerm<'a>) -> NifResult<NifTerm<'a>> {
        let map_env = self.get_env();

        assert!(map_env == key.get_env(), "key is from different environment as map");
        assert!(map_env == value.get_env(), "value is from different environment as map");

        match map::map_put(map_env.raw(), self.raw(), key.raw(), value.raw()) {
            Some(inner) => Ok(NifTerm::new(map_env, inner)),
            None => Err(NifError::BadArg),
        }
    }

    /// Makes a copy of the self map term and removes key. If the key
    /// doesn't exist, the original map is returned.
    ///
    /// Returns Err(NifError::BadArg) if the term is not a map.
    ///
    /// ### Elixir equivalent
    /// ```elixir
    /// Map.delete(self_term, key)
    /// ```
    pub fn map_remove(self, key: NifTerm<'a>) -> NifResult<NifTerm<'a>> {
        let map_env = self.get_env();

        assert!(map_env == key.get_env(), "key is from different environment as map");

        match map::map_remove(map_env.raw(), self.raw(), key.raw()) {
            Some(inner) => Ok(NifTerm::new(map_env, inner)),
            None => Err(NifError::BadArg),
        }
    }

    /// Makes a copy of the self map term where key is set to value.
    ///
    /// Returns Err(NifError::BadArg) if the term is not a map of if key
    /// doesn't exist.
    pub fn map_update(self, key: NifTerm<'a>, new_value: NifTerm<'a>) -> NifResult<NifTerm<'a>> {
        let map_env = self.get_env();

        assert!(map_env == key.get_env(), "key is from different environment as map");
        assert!(map_env == new_value.get_env(), "value is from different environment as map");

        match map::map_update(map_env.raw(), self.raw(), key.raw(), new_value.raw()) {
            Some(inner) => Ok(NifTerm::new(map_env, inner)),
            None => Err(NifError::BadArg),
        }
    }

}

pub struct NifMapIterator<'a> {
    env: NifEnv<'a>,
    iter: map::RawNifMapIterator<'a>
}

impl<'a> NifMapIterator<'a> {
    pub fn new(map: NifTerm<'a>) -> Option<NifMapIterator<'a>> {
        let env = map.get_env();
        map::map_iterator_create(env.raw(), map.raw())
            .map(|iter| NifMapIterator { env: env, iter: iter })
    }
}

impl<'a> Drop for NifMapIterator<'a> {
    fn drop(&mut self) {
        unsafe {
            map::map_iterator_destroy(self.env.raw(), &mut self.iter);
        }
    }
}

impl<'a> Iterator for NifMapIterator<'a> {
    type Item = (NifTerm<'a>, NifTerm<'a>);

    fn next(&mut self) -> Option<(NifTerm<'a>, NifTerm<'a>)> {
        map::map_iterator_get_pair(self.env.raw(), &mut self.iter)
            .map(|(key, value)| {
                map::map_iterator_next(self.env.raw(), &mut self.iter);
                (NifTerm::new(self.env, key),
                 NifTerm::new(self.env, value))
            })
    }
}

impl<'a> NifDecoder<'a> for NifMapIterator<'a> {
    fn decode(term: NifTerm<'a>) -> NifResult<Self> {
        match NifMapIterator::new(term) {
            Some(iter) => Ok(iter),
            None => Err(NifError::BadArg)
        }
    }
}
