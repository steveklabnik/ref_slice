#![no_std]

pub use self::ref_slice_mut as mut_ref_slice;
pub use self::opt_slice_mut as mut_opt_slice;

/// Converts a reference to `A` into a slice of length 1 (without copying).
pub fn ref_slice<A>(s: &A) -> &[A] {
    unsafe {
        core::slice::from_raw_parts(s, 1)
    }
}

/// Converts a reference to `A` into a slice of length 1 (without copying).
pub fn ref_slice_mut<A>(s: &mut A) -> &mut [A] {
    unsafe {
        core::slice::from_raw_parts_mut(s, 1)
    }
}

/// Converts a reference to `Option<A>` into a slice of length 0 or 1 (without copying).
pub fn opt_slice<A>(opt: &Option<A>) -> &[A]
{
    match *opt {
        Some(ref val) => ref_slice(val),
        None => &[],
    }
}

/// Converts a reference to `Option<A>` into a slice of length 0 or 1 (without copying).
pub fn opt_slice_mut<A>(opt: &mut Option<A>) -> &mut [A]
{
    match *opt {
        Some(ref mut val) => mut_ref_slice(val),
        None => &mut [],
    }
}

#[cfg(test)]
mod tests {
    use super::ref_slice;
    use super::mut_ref_slice;
    use super::opt_slice;
    use super::mut_opt_slice;

    #[test]
    fn check() {
        let x = &5;
        let xs = ref_slice(x);

        let result: &[i32] = &[5];

        assert_eq!(result, xs);
    }

    #[test]
    fn check_mut() {
        let x = &mut 5;
        let xs = mut_ref_slice(x);

        let result: &mut [i32] = &mut [5];

        assert_eq!(result, xs);
    }

    #[test]
    fn check_opt() {
        let x = &Some(42);
        let n = &None;
        let xs = opt_slice(x);
        let ns = opt_slice(n);

        let result_x: &[i32] = &[42];
        let result_n: &[i32] = &[];

        assert_eq!(result_x, xs);
        assert_eq!(result_n, ns);
    }

    #[test]
    fn check_opt_mut() {
        let x = &mut Some(42);
        let n = &mut None;
        let xs = mut_opt_slice(x);
        let ns = mut_opt_slice(n);

        let result_x: &[i32] = &mut [42];
        let result_n: &[i32] = &mut [];

        assert_eq!(result_x, xs);
        assert_eq!(result_n, ns);
    }
}
