use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};
pub struct Scheme<K>
where
    K: Hash,
{
    inner: HashMap<K, HashSet<K>>,
}
