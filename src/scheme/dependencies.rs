use std::collections::HashSet;

#[derive(Debug)]
pub struct Dependencies<T> {
    downstream: HashSet<T>,
    upstream: HashSet<T>,
    key: T,
}

impl<T> Dependencies<T> {
    /// Dependencies related to a Task
    /// Contains info on both
    /// Entities that are directly dependent on this entity.
    /// Entities that this entity depends on.

    pub fn new(upstream: HashSet<T>, downstream: HashSet<T>, key: T) -> Self {
        Self {
            upstream,
            downstream,
            key,
        }
    }

    /// Entities that are directly dependent on this entity.
    pub fn get_downstream(&self) -> &HashSet<T> {
        &self.downstream
    }

    /// Entities that this entity depends on.
    pub fn get_upstream(&self) -> &HashSet<T> {
        &self.upstream
    }

    /// Has Any Downstream Dependencies Yet?
    pub fn has_downstream(&self) -> bool {
        !self.downstream.is_empty()
    }

    /// Hash Any Upstream Dependencies Yet?
    pub fn has_upstream(&self) -> bool {
        !self.upstream.is_empty()
    }
}
