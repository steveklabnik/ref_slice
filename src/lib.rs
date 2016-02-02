/// Converts a reference to A into a slice of length 1 (without copying).
#[inline]
pub fn ref_slice<A>(s: &A) -> &[A] {
    unsafe {
        std::slice::from_raw_parts(s, 1)
    }
}

/// Creates a slice of length 0 out of thin air
#[inline]
pub fn empty_slice<A>() -> &[A] {
    unsafe {
        std::slice::from_raw_parts(std::ptr::null(), 0)
    }
}

/// Converts a reference to A into a slice of length 1 (without copying).
#[inline]
pub fn mut_ref_slice<A>(s: &mut A) -> &mut [A] {
    unsafe {
        std::slice::from_raw_parts_mut(s, 1)
    }
}

/// Creates a mutable slice of length 0 out of thin air
#[inline]
pub fn mut_empty_slice<A>() -> &mut [A] {
    unsafe {
        std::slice::from_raw_parts_mut(std::ptr::null_mut(), 0)
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
        
        let noxs = empty_slice::<u32>();
        assert!(noxs.is_empty());
    }

    #[test]
    fn check_mut() {
        let x = &mut 5;
        let xs = mut_ref_slice(x);

        let result: &mut [i32] = &mut [5];

        assert_eq!(result, xs);
        
        let noxs = mut_empty_slice::<u32>();
        assert!(noxs.is_empty());
    }
}
