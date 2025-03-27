use std::collections::hash_map;
use std::hash;
use std::{borrow, marker};
/// [std::collections::HashMap]
pub struct HashMap<K, V> {
    _data: marker::PhantomData<(K, V)>,
}
impl<K, V> HashMap<K, V> {
    /// [std::collections::HashMap::keys]
    pub fn keys(&self) -> hash_map::Keys<'_, K, V> {
        todo!()
    }
    /// [std::collections::HashMap::into_keys]
    pub fn into_keys(self) -> hash_map::IntoKeys<K, V> {
        todo!()
    }
    /// [std::collections::HashMap::values]
    pub fn values(&self) -> hash_map::Values<'_, K, V> {
        todo!()
    }
    /// [std::collections::HashMap::values_mut]
    pub fn values_mut(&mut self) -> hash_map::ValuesMut<'_, K, V> {
        todo!()
    }
    /// [std::collections::HashMap::into_values]
    pub fn into_values(self) -> hash_map::IntoValues<K, V> {
        todo!()
    }
}
impl<K, V> HashMap<K, V>
where
    K: Eq + hash::Hash,
{
    /// [std::collections::HashMap::get_mut]
    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: borrow::Borrow<Q>,
        Q: hash::Hash + Eq + ?Sized,
    {
        todo!()
    }
}
impl<K, V, const N: usize> From<[(K, V); N]> for HashMap<K, V>
where
    K: Eq + hash::Hash,
{
    fn from(value: [(K, V); N]) -> Self {
        todo!()
    }
}
