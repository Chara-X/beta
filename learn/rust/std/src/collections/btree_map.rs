use std::{borrow, collections::btree_map, marker, ops};
/// [std::collections::BTreeMap]
pub struct BTreeMap<K, V> {
    _data: marker::PhantomData<(K, V)>,
}
impl<K, V> BTreeMap<K, V> {
    /// [std::collections::BTreeMap::contains_key]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: borrow::Borrow<Q> + Ord,
        Q: Ord + ?Sized,
    {
        todo!()
    }
    /// [std::collections::BTreeMap::first_key_value]
    pub fn first_key_value(&self) -> Option<(&K, &V)>
    where
        K: Ord,
    {
        todo!()
    }
    /// [std::collections::BTreeMap::last_key_value]
    pub fn last_key_value(&self) -> Option<(&K, &V)>
    where
        K: Ord,
    {
        todo!()
    }
    /// [std::collections::BTreeMap::pop_first]
    pub fn pop_first(&mut self) -> Option<(K, V)>
    where
        K: Ord,
    {
        todo!()
    }
    /// [std::collections::BTreeMap::pop_last]
    pub fn pop_last(&mut self) -> Option<(K, V)>
    where
        K: Ord,
    {
        todo!()
    }
    /// [std::collections::BTreeMap::range]
    pub fn range<T, R>(&self, range: R) -> btree_map::Range<'_, K, V>
    where
        T: Ord + ?Sized,
        K: borrow::Borrow<T> + Ord,
        R: ops::RangeBounds<T>,
    {
        todo!()
    }
    /// [std::collections::BTreeMap::range_mut]
    pub fn range_mut<T, R>(&mut self, range: R) -> btree_map::RangeMut<'_, K, V>
    where
        T: Ord + ?Sized,
        K: borrow::Borrow<T> + Ord,
        R: ops::RangeBounds<T>,
    {
        todo!()
    }
    /// [std::collections::BTreeMap::keys]
    pub fn keys(&self) -> btree_map::Keys<'_, K, V> {
        todo!()
    }
    /// [std::collections::BTreeMap::keys]
    pub fn into_keys(self) -> btree_map::IntoKeys<K, V> {
        todo!()
    }
    /// [std::collections::BTreeMap::values]
    pub fn values(&self) -> btree_map::Values<'_, K, V> {
        todo!()
    }
    /// [std::collections::BTreeMap::values_mut]
    pub fn values_mut(&mut self) -> btree_map::ValuesMut<'_, K, V> {
        todo!()
    }
    /// [std::collections::BTreeMap::values]
    pub fn into_values(self) -> btree_map::IntoValues<K, V> {
        todo!()
    }
}
impl<K, V, const N: usize> From<[(K, V); N]> for BTreeMap<K, V>
where
    K: Ord,
{
    fn from(value: [(K, V); N]) -> Self {
        todo!()
    }
}
