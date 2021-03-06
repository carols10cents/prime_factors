#[allow(dead_code)]
fn prime_factors(num: i64) -> Vec<i64> {
    for i in 2..num {
        if num % i == 0 {
            return vec![i] + &prime_factors(num / i)
        }
        i += 1;
    }
    vec![num]
}

#[test]
fn prime_factors_of_two() {
    assert_eq!(prime_factors(2), [2]);
}

#[test]
fn prime_factors_of_three() {
    assert_eq!(prime_factors(3), [3]);
}

#[test]
fn prime_factors_of_four() {
    assert_eq!(prime_factors(4), [2, 2]);
}

#[test]
fn prime_factors_of_five() {
    assert_eq!(prime_factors(5), [5]);
}

#[test]
fn prime_factors_of_six() {
    assert_eq!(prime_factors(6), [2, 3]);
}

#[test]
fn prime_factors_of_seven() {
    assert_eq!(prime_factors(7), [7]);
}

#[test]
fn prime_factors_of_eight() {
    assert_eq!(prime_factors(8), [2, 2, 2]);
}

#[test]
fn prime_factors_of_nine() {
    assert_eq!(prime_factors(9), [3, 3]);
}

#[test]
fn prime_factors_of_48() {
    assert_eq!(prime_factors(48), [2, 2, 2, 2, 3]);
}

#[test]
fn prime_factors_of_101() {
    assert_eq!(prime_factors(101), [101]);
}
