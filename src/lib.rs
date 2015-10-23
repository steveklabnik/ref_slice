/// Converts a reference to A into a slice of length 1 (without copying).
pub fn ref_slice<A>(s: &A) -> &[A] {
    unsafe {
        std::slice::from_raw_parts(s, 1)
    }
}

/// Converts a reference to A into a slice of length 1 (without copying).
pub fn mut_ref_slice<A>(s: &mut A) -> &mut [A] {
    unsafe {
        std::slice::from_raw_parts_mut(s, 1)
    }
}

#[cfg(test)]
mod tests {
    use super::ref_slice;
    use super::mut_ref_slice;

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
}
