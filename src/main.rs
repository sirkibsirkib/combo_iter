mod iter;
mod test;

pub trait ComboSliceLike<'a, T: 'a>: AsRef<[&'a T]> + AsMut<[&'a T]> {}
impl<'a, T: 'a> ComboSliceLike<'a, T> for Box<[&'a T]> {}
impl<'a, T: 'a, const N: usize> ComboSliceLike<'a, T> for [&'a T; N] {}

pub struct ComboIter<'a, T, S: AsRef<[*const T]> + AsMut<[*const T]>> {
    pub items: &'a [T],
    // INVARIANT: if let State::Next(ptrs) = self.state,
    // then each in ptrs is aligned to an element in self.items.
    pub(crate) next: Option<S>,
    pub ever_advanced: bool,
}

pub type BoxComboIter<'a, T> = ComboIter<'a, T, Box<[*const T]>>;
pub type ArrayComboIter<'a, T, const C: usize> = ComboIter<'a, T, [*const T; C]>;

fn main() {
    let mut x = ArrayComboIter::<_, 2>::new(&[0, 1, 2]);
    while let Some(combo) = x.next() {
        println!("{:?}", combo);
    }
}
