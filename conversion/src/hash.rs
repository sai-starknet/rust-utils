use core::hash::{BuildHasher, Hash};

use hashbrown::{HashMap, HashSet};

use crate::ElementInto;

/// Extension trait for converting the keys and/or values of a [`HashMap`] via [`Into`].
pub trait HashMapInto<K, V, S> {
    /// Convert the keys of a `HashMap` into new types.
    fn key_into<KR>(self) -> HashMap<KR, V, S>
    where
        K: Into<KR>,
        KR: Eq + Hash;
    /// Convert the values of a `HashMap` into new types.
    fn value_into<VR>(self) -> HashMap<K, VR, S>
    where
        V: Into<VR>;
    /// Convert both the keys and values of a `HashMap` into new types.
    fn entries_into<KR, VR>(self) -> HashMap<KR, VR, S>
    where
        K: Into<KR>,
        KR: Eq + Hash,
        V: Into<VR>;
}

impl<K: Eq + Hash, V, S: BuildHasher + Default> HashMapInto<K, V, S> for HashMap<K, V, S> {
    fn key_into<KR>(self) -> HashMap<KR, V, S>
    where
        K: Into<KR>,
        KR: Eq + Hash,
    {
        self.into_iter().map(|(k, v)| (k.into(), v)).collect()
    }
    fn value_into<VR>(self) -> HashMap<K, VR, S>
    where
        V: Into<VR>,
    {
        self.into_iter().map(|(k, v)| (k, v.into())).collect()
    }
    fn entries_into<KR, VR>(self) -> HashMap<KR, VR, S>
    where
        K: Into<KR>,
        KR: Eq + Hash,
        V: Into<VR>,
    {
        self.into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect()
    }
}

/// Extension trait for converting an iterator of key-value pairs into a [`HashMap`].
pub trait IntoHashMap<K, V, S> {
    /// Convert an iterator of key-value pairs into a `HashMap` with converted keys and values.
    fn into_hashmap(self) -> HashMap<K, V, S>;
}

#[cfg(feature = "vec")]
impl<K: Eq + Hash, V, S: BuildHasher + Default, T> IntoHashMap<K, V, S> for alloc::vec::Vec<T>
where
    T: Into<(K, V)>,
{
    fn into_hashmap(self) -> HashMap<K, V, S> {
        self.into_iter().map(Into::into).collect()
    }
}

/// Extension trait for converting the values of a [`HashSet`] via [`Into`].
pub trait HashSetInto<V, S> {
    /// Convert the values of a `HashSet` into new types.
    fn set_into<VR>(self) -> HashSet<VR, S>
    where
        V: Into<VR>,
        VR: Eq + Hash;
}

impl<V: Eq + Hash, S: BuildHasher + Default> HashSetInto<V, S> for HashSet<V, S> {
    fn set_into<VR>(self) -> HashSet<VR, S>
    where
        V: Into<VR>,
        VR: Eq + Hash,
    {
        self.into_iter().map(Into::into).collect()
    }
}

impl<T, TR> ElementInto<HashSet<TR>> for HashSet<T>
where
    T: Into<TR>,
    TR: Eq + Hash,
{
    fn element_into(self) -> HashSet<TR> {
        self.into_iter().map(Into::into).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_into_converts_keys() {
        let mut map = HashMap::new();
        map.insert(1u8, "a");
        map.insert(2u8, "b");
        let converted: HashMap<u16, &str> = map.key_into();
        assert_eq!(converted.get(&1u16), Some(&"a"));
        assert_eq!(converted.get(&2u16), Some(&"b"));
        assert_eq!(converted.len(), 2);
    }

    #[test]
    fn value_into_converts_values() {
        let mut map = HashMap::new();
        map.insert("x", 1u8);
        map.insert("y", 2u8);
        let converted: HashMap<&str, u16> = map.value_into();
        assert_eq!(converted.get("x"), Some(&1u16));
        assert_eq!(converted.get("y"), Some(&2u16));
    }

    #[test]
    fn hashmap_into_converts_both() {
        let mut map = HashMap::new();
        map.insert(1u8, 10u8);
        let converted: HashMap<u16, u16> = map.entries_into();
        assert_eq!(converted.get(&1u16), Some(&10u16));
    }

    #[test]
    fn hashmap_into_empty() {
        let map: HashMap<u8, u8> = HashMap::new();
        let converted: HashMap<u16, u16> = map.entries_into();
        assert!(converted.is_empty());
    }

    #[test]
    fn set_into_converts_values() {
        let mut set = HashSet::new();
        set.insert(1u8);
        set.insert(2u8);
        set.insert(3u8);
        let converted: HashSet<u16> = set.set_into();
        assert!(converted.contains(&1u16));
        assert!(converted.contains(&2u16));
        assert!(converted.contains(&3u16));
        assert_eq!(converted.len(), 3);
    }

    #[test]
    fn set_into_empty() {
        let set: HashSet<u8> = HashSet::new();
        let converted: HashSet<u16> = set.set_into();
        assert!(converted.is_empty());
    }
}
