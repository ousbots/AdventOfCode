use day17::*;

#[test]
fn provieded_tests() {
    assert_eq!(
        find_highest_launch(&parse("target area: x=20..30, y=-10..-5")),
        45
    );
}
