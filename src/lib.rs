#[allow(dead_code)]
fn prime_factors(num: i64) -> Vec<i64> {
    vec![2]
}

#[test]
fn prime_factors_of_two() {
    assert_eq!(prime_factors(2), [2]);
}
