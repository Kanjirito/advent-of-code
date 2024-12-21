/// Division and modulo at once
///
/// For negative numbers use [`signed_divmod()`][signed_divmod].
pub fn divmod<T>(first: T, second: T) -> (T, T)
where
    T: std::ops::Div<Output = T> + std::ops::Rem<Output = T> + Copy,
{
    (first / second, first % second)
}

/// Divmod for negative numbers
///
/// Rust's modulo operation works differently for negative numbers so [`divmod()`][divmod] probably
/// doesn't work as expected. This function uses [`isize::div_euclid()`][isize::div_euclid] and
/// [`isize::rem_euclid()`][isize::rem_euclid] instead.
pub fn signed_divmod(first: isize, second: isize) -> (isize, isize) {
    (first.div_euclid(second), first.rem_euclid(second))
}

/// Greatest common divisor
///
/// A number that both of the arguments can be divided by.
pub fn gcd(first: usize, second: usize) -> usize {
    let (bigger, smaller) = if first >= second {
        (first, second)
    } else {
        (second, first)
    };
    let rem = bigger % smaller;
    if rem == 0 {
        smaller
    } else {
        gcd(smaller, rem)
    }
}

/// Least common multiple
///
/// A number that can be divided by both of the arguments.
pub fn lcm(first: usize, second: usize) -> usize {
    (first * second) / gcd(first, second)
}

pub fn manhattan_distance(first: (usize, usize), second: (usize, usize)) -> usize {
    first.0.abs_diff(second.0) + first.1.abs_diff(second.1)
}

#[cfg(test)]
mod divmod_tests {
    use super::*;

    macro_rules! div_tests_gen {
        ($type:ty) => {
            assert_eq!(divmod::<$type>(10, 3), (3, 1));
            assert_eq!(divmod::<$type>(11, 3), (3, 2));
            assert_eq!(divmod::<$type>(12, 3), (4, 0));
            assert_eq!(divmod::<$type>(3, 3), (1, 0));
            assert_eq!(divmod::<$type>(1, 3), (0, 1));
            assert_eq!(divmod::<$type>(0, 3), (0, 0));
        };
    }

    #[test]
    fn divmod_unsigned_tests() {
        div_tests_gen!(u8);
        div_tests_gen!(u16);
        div_tests_gen!(u32);
        div_tests_gen!(u64);
        div_tests_gen!(u128);
        div_tests_gen!(usize);
    }

    #[test]
    fn divmod_signed_tests() {
        div_tests_gen!(i8);
        div_tests_gen!(i16);
        div_tests_gen!(i32);
        div_tests_gen!(i64);
        div_tests_gen!(i128);
        div_tests_gen!(isize);
    }

    #[test]
    fn divmod_negative() {
        assert_eq!(divmod::<i8>(10, -3), (-3, 1));
        assert_eq!(divmod::<i8>(11, -3), (-3, 2));
        assert_eq!(divmod::<i8>(-10, 3), (-3, -1));
        assert_eq!(divmod::<i8>(-11, 3), (-3, -2));
        assert_eq!(divmod::<i8>(-10, -3), (3, -1));
        assert_eq!(divmod::<i8>(-11, -3), (3, -2));

        assert_eq!(divmod::<i8>(3, -3), (-1, 0));
        assert_eq!(divmod::<i8>(2, -3), (0, 2));
        assert_eq!(divmod::<i8>(0, -3), (0, 0));
        assert_eq!(divmod::<i8>(0, 3), (0, 0));
    }

    #[test]
    #[should_panic]
    fn divmod_by_zero() {
        divmod(1, 0);
    }

    #[test]
    fn signed_divmod_tests() {
        assert_eq!(signed_divmod(10, 3), (3, 1));
        assert_eq!(signed_divmod(11, 3), (3, 2));
        assert_eq!(signed_divmod(12, 3), (4, 0));
        assert_eq!(signed_divmod(3, 3), (1, 0));
        assert_eq!(signed_divmod(1, 3), (0, 1));
        assert_eq!(signed_divmod(0, 3), (0, 0));
    }

    #[test]
    fn signed_divmod_negative() {
        assert_eq!(signed_divmod(10, -3), (-3, 1));
        assert_eq!(signed_divmod(11, -3), (-3, 2));
        assert_eq!(signed_divmod(-10, 3), (-4, 2));
        assert_eq!(signed_divmod(-11, 3), (-4, 1));
        assert_eq!(signed_divmod(-10, -3), (4, 2));
        assert_eq!(signed_divmod(-11, -3), (4, 1));

        assert_eq!(signed_divmod(3, -3), (-1, 0));
        assert_eq!(signed_divmod(2, -3), (0, 2));
        assert_eq!(signed_divmod(0, -3), (0, 0));
        assert_eq!(signed_divmod(0, 3), (0, 0));
    }

    #[test]
    #[should_panic]
    fn singed_divmod_by_zero() {
        signed_divmod(1, 0);
    }

    #[test]
    #[should_panic]
    fn singed_divmod_min_by_negative() {
        signed_divmod(isize::MIN, -1);
    }
}
