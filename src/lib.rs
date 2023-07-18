mod iter;

#[cfg(test)]
mod test;

pub struct ComboIter<'a, T, S: AsRef<[*const T]> + AsMut<[*const T]>> {
    pub items: &'a [T],
    // INVARIANT: pointers in self.next.as_ref() are aligned to items
    pub(crate) next: Option<S>,
    pub ever_advanced: bool,
}

pub type BoxComboIter<'a, T> = ComboIter<'a, T, Box<[*const T]>>;
pub type ArrayComboIter<'a, T, const C: usize> = ComboIter<'a, T, [*const T; C]>;
