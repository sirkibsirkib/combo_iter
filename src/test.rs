use crate::BoxComboIter;

fn check_boxed<const C: usize>(num_items: u16, expect_combos: &[[u16; C]]) {
    fn check_combo(a: &[&u16], b: &[u16]) {
        assert!(a.len() == b.len());
        for (a, b) in a.iter().zip(b.iter()) {
            assert_eq!(*a, b)
        }
    }
    let items: Vec<_> = (0u16..num_items).collect();
    let mut combo_iter = BoxComboIter::new(&items, C);
    let mut expect_combo_iter = expect_combos.iter();
    while let Some(a) = combo_iter.next() {
        let b = expect_combo_iter.next().unwrap();
        check_combo(a, b);
    }
    assert!(expect_combo_iter.next().is_none());
}

#[test]
fn a() {
    check_boxed(1, &[[0]])
}

#[test]
fn b() {
    check_boxed(1, &[[]])
}

#[test]
fn c() {
    check_boxed(0, &[[]])
}

#[test]
fn d() {
    check_boxed(2, &[[0], [1]])
}

#[test]
fn e() {
    check_boxed(2, &[[0, 0], [0, 1], [1, 0], [1, 1]])
}

#[test]
fn f() {
    check_boxed(3, &[[]])
}

#[test]
fn g() {
    check_boxed(3, &[[0], [1], [2]])
}

#[test]
fn h() {
    check_boxed(3, &[[0, 0], [0, 1], [0, 2], [1, 0], [1, 1], [1, 2], [2, 0], [2, 1], [2, 2]])
}
