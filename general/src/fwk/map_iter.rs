//! Iterator utilities for maps. Used in the implementation of [super::map_ext_owned] and [super::map_ext_ref].

pub fn map_entries<'a, K, V, K1, V1>(
    iter: impl Iterator<Item = (&'a K, &'a V)> + 'a,
    mut f: impl FnMut(&K, &V) -> (K1, V1) + 'a,
) -> impl Iterator<Item = (K1, V1)> + 'a
where
    K: 'a,
    V: 'a,
{
    iter.map(move |(k, v)| f(k, v))
}

pub fn map_values<'a, K, V, V1>(
    iter: impl Iterator<Item = (&'a K, &'a V)> + 'a,
    mut f: impl FnMut(&V) -> V1 + 'a,
) -> impl Iterator<Item = (K, V1)> + 'a
where
    K: Clone + 'a,
    V: 'a,
{
    iter.map(move |(k, v)| (k.clone(), f(v)))
}

pub fn filter<'a, K, V>(
    iter: impl Iterator<Item = (&'a K, &'a V)> + 'a,
    mut f: impl FnMut(&K, &V) -> bool + 'a,
) -> impl Iterator<Item = (K, V)> + 'a
where
    K: 'a + Clone,
    V: 'a + Clone,
{
    iter.filter(move |(k, v)| f(k, v))
        .map(|(k, v)| (k.clone(), v.clone()))
}
