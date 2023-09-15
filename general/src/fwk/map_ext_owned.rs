//! Wrappers for [`HashMap`] and [`BTreeMap`] that provide direct map and filter methods, including the
//! conversion between these two map types. The wrappers own the wrapped data.

use super::map_iter::{filter, map_entries, map_values};
use std::{
    borrow::Borrow,
    collections::{BTreeMap, HashMap},
    fmt::Debug,
    hash::Hash,
    ops::Deref,
};

#[derive(PartialEq, Eq, Clone)]
pub struct BTreeMapExt<K, V>(pub BTreeMap<K, V>);

impl<K, V> Debug for BTreeMapExt<K, V>
where
    K: Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (&self.0 as &dyn Debug).fmt(f)
    }
}

impl<K, V> From<BTreeMap<K, V>> for BTreeMapExt<K, V> {
    fn from(value: BTreeMap<K, V>) -> Self {
        Self(value)
    }
}

impl<K, V> Deref for BTreeMapExt<K, V> {
    type Target = BTreeMap<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

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
        (&self.0).into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut BTreeMapExt<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = <&'a mut BTreeMap<K, V> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).into_iter()
    }
}

impl<K, V> BTreeMapExt<K, V> {
    pub fn map_to_btree_map<K1, V1>(&self, f: impl FnMut(&K, &V) -> (K1, V1)) -> BTreeMapExt<K1, V1>
    where
        K1: Ord,
    {
        map_entries(self.0.iter(), f)
            .collect::<BTreeMap<_, _>>()
            .into()
    }

    pub fn map_to_hash_map<K1, V1>(&self, f: impl FnMut(&K, &V) -> (K1, V1)) -> HashMapExt<K1, V1>
    where
        K1: Eq + Hash,
    {
        map_entries(self.0.iter(), f)
            .collect::<HashMap<_, _>>()
            .into()
    }

    pub fn map_values<V1>(&self, f: impl FnMut(&V) -> V1) -> BTreeMapExt<K, V1>
    where
        K: Ord + Clone,
    {
        map_values(self.0.iter(), f)
            .collect::<BTreeMap<_, _>>()
            .into()
    }

    pub fn filter(&self, f: impl FnMut(&K, &V) -> bool) -> BTreeMapExt<K, V>
    where
        K: Ord + Clone,
        V: Clone,
    {
        filter(self.0.iter(), f).collect::<BTreeMap<_, _>>().into()
    }

    pub fn aggregate_by<G, V1>(
        &self,
        mut key_grouper: impl FnMut(&K) -> G,
        mut value_aggregator: impl FnMut(&mut V1, &V),
        seed: V1,
    ) -> BTreeMapExt<G, V1>
    where
        G: Ord + Clone,
        V1: Clone,
    {
        let mut res = BTreeMap::<G, V1>::new();
        for (k, v) in &self.0 {
            let g = key_grouper(k);
            let v1 = match res.get_mut(&g) {
                Some(v1) => v1,
                None => {
                    res.insert(g.clone(), seed.clone());
                    res.get_mut(&g).unwrap()
                }
            };
            value_aggregator(v1, v);
        }
        res.into()
    }
}

pub struct HashMapExt<K, V>(pub HashMap<K, V>);

impl<K, V> From<HashMap<K, V>> for HashMapExt<K, V> {
    fn from(value: HashMap<K, V>) -> Self {
        Self(value)
    }
}

impl<K, V> Borrow<HashMap<K, V>> for HashMapExt<K, V> {
    fn borrow(&self) -> &HashMap<K, V> {
        &self.0
    }
}

impl<K, V> AsRef<HashMap<K, V>> for HashMapExt<K, V> {
    fn as_ref(&self) -> &HashMap<K, V> {
        &self.0
    }
}

impl<K, V> Deref for HashMapExt<K, V> {
    type Target = HashMap<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> HashMapExt<K, V> {
    pub fn map_to_btree_map<K1, V1>(&self, f: impl FnMut(&K, &V) -> (K1, V1)) -> BTreeMap<K1, V1>
    where
        K1: Ord,
    {
        map_entries(self.0.iter(), f).collect::<BTreeMap<_, _>>()
    }

    pub fn map_to_hash_map<K1, V1>(&self, f: impl FnMut(&K, &V) -> (K1, V1)) -> HashMap<K1, V1>
    where
        K1: Eq + Hash,
    {
        map_entries(self.0.iter(), f).collect::<HashMap<_, _>>()
    }

    pub fn map_values<V1>(&self, f: impl FnMut(&V) -> V1) -> HashMap<K, V1>
    where
        K: Eq + Hash + Clone,
    {
        map_values(self.0.iter(), f).collect::<HashMap<_, _>>()
    }

    pub fn filter(&self, f: impl FnMut(&K, &V) -> bool) -> HashMap<K, V>
    where
        K: Eq + Hash + Clone,
        V: Clone,
    {
        filter(self.0.iter(), f).collect::<HashMap<_, _>>()
    }
}
