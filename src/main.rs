pub trait SliceLike<T> {
    fn slice(&self) -> &[T];
    fn slice_mut(&mut self) -> &mut [T];
}
impl<T> SliceLike<T> for Box<[T]> {
    fn slice(&self) -> &[T] {
        self
    }
    fn slice_mut(&mut self) -> &mut [T] {
        self
    }
}
impl<T, const N: usize> SliceLike<T> for [T; N] {
    fn slice(&self) -> &[T] {
        self
    }
    fn slice_mut(&mut self) -> &mut [T] {
        self
    }
}

pub struct SliceItemComboIterX<'a, T, S: SliceLike<*const T>> {
    items: &'a [T],
    // INVARIANT: if let State::Next(ptrs) = self.state,
    // then each in ptrs is aligned to an element in self.items.
    next: Option<S>,
    ever_advanced: bool,
}

pub type SliceItemComboIterDyn<'a, T> = SliceItemComboIterX<'a, T, Box<[*const T]>>;
impl<'a, T> SliceItemComboIterDyn<'a, T> {
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

pub type SliceItemComboIterStat<'a, T, const C: usize> = SliceItemComboIterX<'a, T, [*const T; C]>;
impl<'a, T, const C: usize> SliceItemComboIterStat<'a, T, C> {
    pub fn new(items: &'a [T], combo_len: usize) -> Self {
        let next = if items.is_empty() && combo_len > 0 {
            None
        } else {
            // invariant established: items is non-empty, so items.as_ptr() is in range
            Some([items.as_ptr(); C])
        };
        Self { items, next, ever_advanced: false }
    }
}

impl<'a, T, S: SliceLike<*const T>> SliceItemComboIterX<'a, T, S> {
    pub fn advance(&mut self) {
        if !self.ever_advanced {
            // Something of a sentinel. Needed because advancing
            // must _precede_ peeking, but it follows, conceptually.
            self.ever_advanced = true;
            return;
        }
        if let Some(next) = self.next.as_mut().map(SliceLike::slice_mut) {
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
    pub fn next(&mut self) -> Option<&[&T]> {
        self.advance();
        self.peek()
    }
    pub fn peek(&self) -> Option<&[&T]> {
        let next: &[*const T] = self.next.as_ref().map(SliceLike::slice)?;
        unsafe {
            // SAFETY:
            // - in bounds because of invariant
            // - transmute between these types is safe
            core::mem::transmute(next)
        }
    }
}

fn main() {
    let mut x = SliceItemComboIterDyn::new(&[0, 1, 2], 2);
    while let Some(combo) = x.next() {
        println!("{:?}", combo);
    }
}

fn check<const C: usize>(num_items: u16, expect_combos: &[[u16; C]]) {
    fn check_combo(a: &[&u16], b: &[u16]) {
        assert!(a.len() == b.len());
        for (a, b) in a.iter().zip(b.iter()) {
            assert_eq!(*a, b)
        }
    }
    let items: Vec<_> = (0u16..num_items).collect();
    let mut combo_iter = SliceItemComboIterDyn::new(&items, C);
    let mut expect_combo_iter = expect_combos.iter();
    while let Some(a) = combo_iter.next() {
        let b = expect_combo_iter.next().unwrap();
        check_combo(a, b);
    }
    assert!(expect_combo_iter.next().is_none());
}

#[test]
fn a() {
    check(1, &[[0]])
}

#[test]
fn b() {
    check(1, &[[]])
}

#[test]
fn c() {
    check(0, &[[]])
}

#[test]
fn d() {
    check(2, &[[0], [1]])
}

#[test]
fn e() {
    check(2, &[[0, 0], [0, 1], [1, 0], [1, 1]])
}

#[test]
fn f() {
    check(3, &[[]])
}

#[test]
fn g() {
    check(3, &[[0], [1], [2]])
}

#[test]
fn h() {
    check(3, &[[0, 0], [0, 1], [0, 2], [1, 0], [1, 1], [1, 2], [2, 0], [2, 1], [2, 2]])
}
