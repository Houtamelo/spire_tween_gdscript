use smallvec::{Array, SmallVec};

pub struct SmolSet<A: Array>
where A::Item: PartialEq
{
    inner: SmallVec<A>,
}

impl<A: Array> Default for SmolSet<A>
where A::Item: PartialEq
{
    fn default() -> Self { Self::new() }
}

impl<A: Array> SmolSet<A>
where A::Item: PartialEq
{
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: SmallVec::new(),
        }
    }

    #[inline]
    pub fn insert(&mut self, val: A::Item) -> bool {
        if self.inner.contains(&val) {
            false
        } else {
            self.inner.push(val);
            true
        }
    }

    #[inline]
    pub fn remove(&mut self, val: &A::Item) -> bool {
        if let Some(pos) = self.inner.iter().position(|x| x == val) {
            self.inner.swap_remove(pos);
            true
        } else {
            false
        }
    }

    #[inline]
    pub fn contains(&self, val: &A::Item) -> bool { self.inner.contains(val) }

    #[inline]
    pub fn iter(&self) -> core::slice::Iter<'_, A::Item> { self.inner.iter() }

    #[inline]
    pub fn clear(&mut self) { self.inner.clear(); }

    #[inline]
    pub fn is_empty(&self) -> bool { self.inner.is_empty() }

    #[inline]
    pub fn len(&self) -> usize { self.inner.len() }

    #[inline]
    pub fn retain<F: FnMut(&mut A::Item) -> bool>(&mut self, f: F) { self.inner.retain(f); }
}

impl<'a, A: Array> IntoIterator for &'a SmolSet<A>
where A::Item: PartialEq
{
    type Item = &'a A::Item;
    type IntoIter = core::slice::Iter<'a, A::Item>;

    fn into_iter(self) -> Self::IntoIter { self.inner.iter() }
}
