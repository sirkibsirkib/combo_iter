use crate::{ArrayComboIter, BoxComboIter, ComboIter};

impl<'a, T, S: AsRef<[*const T]> + AsMut<[*const T]>> ComboIter<'a, T, S> {
    /// advance to the next combo
    pub fn advance(&mut self) {
        if let Some(next) = self.next.as_mut().map(AsMut::as_mut) {
            for i in (0..next.len()).rev() {
                let n = &mut next[i];
                let range = self.items.as_ptr_range();
                // invariant by assuption ...
                debug_assert!(range.contains(n));
                *n = unsafe {
                    // SAFETY: everything in range.start..=range.end is safe
                    n.add(1)
                };
                if *n < range.end {
                    // ok! the invariant is intact
                    return;
                } else {
                    // uh oh! out of range. Roll over!
                    *n = range.start;
                }
                // ... invariant preserved
                debug_assert!(range.contains(n));
            }
            // rolled over all the way!
            self.next = None;
        }
    }
    pub fn next(&mut self) -> Option<&'a [&'a T]> {
        if !self.ever_advanced {
            // Something of a sentinel. Needed because advancing
            // must _precede_ peeking, but it follows, conceptually.
            self.ever_advanced = true;
        } else {
            self.advance();
        }
        self.peek()
    }
    pub fn peek(&self) -> Option<&'a [&'a T]> {
        let next: &[*const T] = self.next.as_ref().map(AsRef::as_ref)?;
        unsafe {
            // SAFETY:
            // - in bounds because of invariant
            // - transmute between these types is safe
            core::mem::transmute(next)
        }
    }
}

impl<'a, T> BoxComboIter<'a, T> {
    pub fn new(items: &'a [T], combo_len: usize) -> Self {
        let next = if items.is_empty() && combo_len > 0 {
            None
        } else {
            // invariant established: items is non-empty, so items.as_ptr() is in range
            Some(std::iter::repeat(items.as_ptr()).take(combo_len).collect())
        };
        Self { items, next, ever_advanced: false }
    }
}

impl<'a, T, const C: usize> ArrayComboIter<'a, T, C> {
    pub fn new(items: &'a [T]) -> Self {
        let next = if items.is_empty() && C > 0 {
            None
        } else {
            // invariant established: items is non-empty, so items.as_ptr() is in range
            Some([items.as_ptr(); C])
        };
        Self { items, next, ever_advanced: false }
    }
}
