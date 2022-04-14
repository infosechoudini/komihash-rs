use std::borrow::Borrow;
use std::collections::{hash_map, HashMap};
use std::fmt::{self, Debug};
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut, Index};
use std::panic::UnwindSafe;
use std::hash::{Hasher, BuildHasher, BuildHasherDefault, Hash};

use crate::Komihash;

/// A [`HashMap`](std::collections::HashMap) using [`random`](crate::random) to hash the items.
/// (Requires the `std` feature to be enabled.)
#[derive(Clone)]
pub struct KomiHashMap<K, V, S = KomiHashBuilder>(HashMap<K, V, S>);


impl<K, V> From<HashMap<K, V, KomiHashBuilder>> for KomiHashMap<K, V> {
    fn from(item: HashMap<K, V, KomiHashBuilder>) -> Self {
        KomiHashMap(item)
    }
}

impl<K, V, const N: usize> From<[(K,V); N]> for KomiHashMap<K, V>
where
    K: Eq + Hash,
{
    /// # Examples
    ///
    /// ```
    /// use komihash_rs::KomiHashMap;
    ///
    /// let map1 = KomiHashMap::from([(1, 2), (3, 4)]);
    /// let map2: KomiHashMap<_, _> = [(1, 2), (3, 4)].into();
    /// assert_eq!(map1, map2);
    /// ```
    fn from(arr: [(K, V); N]) -> Self {
        Self::from_iter(arr)
    }
}



impl<K , V > Into<HashMap<K, V, KomiHashBuilder>> for KomiHashMap<K , V> {
    fn into(self) -> HashMap<K , V, KomiHashBuilder> {
        self.0
    }
}

impl<K , V> KomiHashMap<K, V, KomiHashBuilder> {
    pub fn new() -> Self {
        KomiHashMap(HashMap::with_hasher(KomiHashBuilder::default()))
    }

    pub fn with_capacity(capacity: usize) -> Self {
        KomiHashMap(HashMap::with_capacity_and_hasher(capacity, KomiHashBuilder::default()))
    }
}


impl<K, V, S> KomiHashMap<K, V, S>
where
    S: BuildHasher,
{
    pub fn with_hasher(hash_builder: S) -> Self {
        KomiHashMap(HashMap::with_hasher(hash_builder))
    }

    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> Self {
        KomiHashMap(HashMap::with_capacity_and_hasher(capacity, hash_builder))
    }
}

impl<K, V, S> KomiHashMap<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    /// Returns a reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.get(&1), Some(&"a"));
    /// assert_eq!(map.get(&2), None);
    /// ```
    #[inline(always)]
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get(k)
    }

    /// Returns the key-value pair corresponding to the supplied key.
    ///
    /// The supplied key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.get_key_value(&1), Some((&1, &"a")));
    /// assert_eq!(map.get_key_value(&2), None);
    /// ```
    #[inline(always)]
    pub fn get_key_value<Q: ?Sized>(&self, k: &Q) -> Option<(&K,&V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get_key_value(k)
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// if let Some(x) = map.get_mut(&1) {
    ///     *x = "b";
    /// }
    /// assert_eq!(map[&1], "b");
    /// ```
    #[inline(always)]
    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.get_mut(k)
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, [`None`] is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned. The key is not updated, though; this matters for
    /// types that can be `==` without being identical. See the [module-level
    /// documentation] for more.
    ///
    /// [module-level documentation]: crate::collections#insert-and-complex-keys
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// assert_eq!(map.insert(37, "a"), None);
    /// assert_eq!(map.is_empty(), false);
    ///
    /// map.insert(37, "b");
    /// assert_eq!(map.insert(37, "c"), Some("b"));
    /// assert_eq!(map[&37], "c");
    /// ```
    #[inline(always)]
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.0.insert(k , v)
    }

    /// Removes a key from the map, returning the value at the key if the key
    /// was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but
    /// [`Hash`] and [`Eq`] on the borrowed form *must* match those for
    /// the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert(1, "a");
    /// assert_eq!(map.remove(&1), Some("a"));
    /// assert_eq!(map.remove(&1), None);
    /// ```
    #[inline(always)]
    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        self.0.remove(k)
    }
}

impl<K, V, S> Deref for KomiHashMap<K, V, S>{
    type Target = HashMap<K, V, S>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V, S> DerefMut for KomiHashMap<K, V, S>{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V, S> UnwindSafe for KomiHashMap<K, V, S>
where
    K: UnwindSafe,
    V: UnwindSafe,
{
}

impl<K, V, S> PartialEq for KomiHashMap<K, V, S>
where
    K: Eq + Hash,
    V: PartialEq,
    S: BuildHasher,
{
    fn eq(&self, other: &KomiHashMap<K, V, S>) -> bool {
        self.0.eq(&other.0)
    }
}

impl<K, V, S> Eq for KomiHashMap<K, V, S>
where
    K: Eq + Hash,
    V: Eq,
    S: BuildHasher,
{
}

impl<K,Q: ?Sized, V, S > Index<&Q> for KomiHashMap<K, V, S>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash,
    S: BuildHasher,
{
    type Output = V;

    /// Returns a reference to the value corresponding to the supplied key.
    ///
    /// # Panics
    ///
    /// Panics if the key is not present in the `HashMap`.
    #[inline(always)]
    fn index(&self, key: &Q) -> &V {
        self.0.index(key)
    }
}

impl<K, V, S> Debug for KomiHashMap<K, V, S>
where
    K: Debug,
    V: Debug,
    S: BuildHasher,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(fmt)
    }
}

impl<K, V, S> FromIterator<(K,V)> for KomiHashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (K,V)>>(iter: T) -> Self {
        KomiHashMap(HashMap::from_iter(iter))
    }
}

impl<'a, K, V, S> IntoIterator for &'a KomiHashMap<K, V, S> {
    type Item = (&'a K,&'a V);
    type IntoIter = hash_map::Iter<'a, K,V>;
    fn into_iter(self) -> Self::IntoIter {
        (&self.0).iter()
    }
}

impl<'a, K, V, S> IntoIterator for &'a mut KomiHashMap<K, V, S> {
    type Item = (&'a K,&'a mut V);
    type IntoIter = hash_map::IterMut<'a, K,V>;
    fn into_iter(self) -> Self::IntoIter {
        (&mut self.0).iter_mut()
    }
}

impl<K, V, S> IntoIterator for KomiHashMap<K, V, S> {
    type Item = (K,V);
    type IntoIter = hash_map::IntoIter<K,V>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<K, V, S> Extend<(K,V)> for KomiHashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    #[inline(always)]
    fn extend<T: IntoIterator<Item = (K,V)>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}

impl<'a, K, V, S> Extend<(&'a K,&'a V)> for KomiHashMap<K, V, S>
where
    K: Eq + Hash + Copy + 'a,
    V: Copy + 'a,
    S: BuildHasher,
{
    #[inline(always)]
    fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
        self.0.extend(iter)
    }
}


impl<K,V> Default for KomiHashMap<K, V, KomiHashBuilder> {
    #[inline(always)]
    fn default() -> KomiHashMap<K, V, KomiHashBuilder> {
        KomiHashMap::new()
    }
}

#[cfg(feature = "serde")]
impl<K,V> Serialize for KomiHashMap<K,V>
where
    K: Serialize + Eq + Hash,
    V: Serialize,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::OK,S::Error> {
        self.deref().serialize(serializer)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct KomiHashBuilder {
    _seed: u64,
    hasher: Komihash,
}

impl KomiHashBuilder {
    #[inline(always)]
    ///Creates builder with provided `seed`
    pub fn new(seed: u64) -> Self {
        Self {
            _seed: seed,
            hasher: Komihash::new(seed)
        }
    }

    #[inline(always)]
    ///Creates hasher.
    pub fn build(&self) -> Komihash {
        let mut kh = self.hasher;
        kh.komihash_hashround();
        kh
    }
}


impl Default for KomiHashBuilder {

    #[inline(always)]
    fn default()  -> Self {
        KomiHashBuilder::new(0)
    }
}


impl BuildHasher for KomiHashBuilder {
    type Hasher = Komihash;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        self.build()
        
    }
}

pub trait HashMapExt {
    /// Constructs a new HashMap
    #[inline(always)]
    fn new() -> Self;


    /// Constructs a new HashMap with a given initial capacity
    #[inline(always)]
    fn with_capacity(capacity: usize) -> Self;
}

impl<K, V, S> HashMapExt for HashMap<K, V, S>
where
    S: BuildHasher + Default,
{
    #[inline(always)]
    fn new() -> Self {
        HashMap::with_hasher(S::default())
    }

    #[inline(always)]
    fn with_capacity(capacity: usize) -> Self {
        HashMap::with_capacity_and_hasher(capacity, S::default())
    }
}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_borrow() {
        let mut map: KomiHashMap<String, String> = KomiHashMap::new();
        map.insert("foo".to_string(), "Bar".to_string());
        map.insert("Bar".to_string(), map.get("foo").unwrap().to_owned());
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde() {
        let mut map = KomiHashMap::new();
        map.insert("for".to_string(), 0);
        map.insert("bar".to_string(), 1);
        let mut serialization = serde_json::to_string(&map).unwrap();
        let mut deserialization: KomiHashMap<String, u64> = serde_json::from_str(&serialization).unwrap();
        assert_eq!(deserialization, map);

        map.insert("baz".to_string(), 2);
        serialization = serde_json::to_string(&map).unwrap();
        let mut deserializer = serde_json::Deserializer::from_str(&serialization);
        KomiHashMap::deserialize_in_place(&mut deserializer, &mut deserialization).unwrap();
        assert_eq!(deserialization, map);
    }
}