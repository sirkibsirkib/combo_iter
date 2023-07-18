use crate::ComboIter;

fn check_combo(a: &[&u16], b: &[u16]) {
    assert!(a.len() == b.len());
    for (a, b) in a.iter().zip(b.iter()) {
        assert_eq!(*a, b)
    }
}

fn check<const C: usize, S: AsRef<[*const u16]> + AsMut<[*const u16]>>(
    mut combo_iter: ComboIter<u16, S>,
    expect_combos: &[[u16; C]],
) {
    let mut expect_combo_iter = expect_combos.iter();
    while let Some(a) = combo_iter.next() {
        let b = expect_combo_iter.next().unwrap();
        check_combo(a, b);
    }
    assert!(expect_combo_iter.next().is_none());
}

mod boxed {
    fn check<const C: usize>(num_items: u16, expect_combos: &[[u16; C]]) {
        let items: Vec<_> = (0u16..num_items).collect();
        let combo_iter = crate::BoxComboIter::new(&items, C);
        super::check(combo_iter, expect_combos)
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
}

mod arrayed {
    fn check<const C: usize>(num_items: u16, expect_combos: &[[u16; C]]) {
        let items: Vec<_> = (0u16..num_items).collect();
        let combo_iter = crate::ArrayComboIter::<_, C>::new(&items);
        super::check(combo_iter, expect_combos)
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
}
