use super::wrapper_simplified::Wrapper;
use std::{borrow::Borrow, collections::BTreeMap};

/// Wrapper of [BTreeMap] that provides an additional [`map_values`](Self::map_values) method.
/// As this type [Deref](std::ops::Deref)s to [BTreeMap] and implements [IntoIterator]s with the same results as
/// those of [BTreeMap], it supports `for` loops and all immutable [BTreeMap] methods.
pub type BTreeMapExt<K, V> = Wrapper<BTreeMap<K, V>>;

impl<K, V> IntoIterator for BTreeMapExt<K, V> {
    type Item = (K, V);
    type IntoIter = <BTreeMap<K, V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a BTreeMapExt<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = <&'a BTreeMap<K, V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut BTreeMapExt<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = <&'a mut BTreeMap<K, V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl<K, V> BTreeMapExt<K, V> {
    /// Returns a new [BTreeMapExt] with the same keys as `self` and values corresponding to the
    /// invocation of function `f` on the original values.
    pub fn map_values<V1, BV>(&self, mut f: impl FnMut(&BV) -> V1) -> BTreeMapExt<K, V1>
    where
        K: Ord + Clone,
        V: Borrow<BV>,
    {
        self.0
            .iter()
            .map(|(k, v)| (k.clone(), f(v.borrow())))
            .collect::<BTreeMap<_, _>>()
            .into()
    }
}

// Confirm BTreeMapExt::map_values works with f: impl FnMut(&V) -> V1.
fn _type_check<K, V, V1>(b: BTreeMapExt<K, V>, f: impl FnMut(&V) -> V1)
where
    K: Ord + Clone,
{
    _ = b.map_values(f);
}
