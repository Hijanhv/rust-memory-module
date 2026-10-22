//! Page size from the operating system.
//!
//! The standard library doesn't expose it, so we call the C function
//! `getpagesize` directly. Calling a foreign function is one of unsafe Rust's
//! five superpowers: the compiler can't check what C code does.

#[cfg(unix)]
extern "C" {
    fn getpagesize() -> i32;
}

pub fn size() -> usize {
    #[cfg(unix)]
    {
        // SAFETY: getpagesize takes no arguments and only returns a number
        unsafe { getpagesize() as usize }
    }
    #[cfg(not(unix))]
    {
        4096
    }
}

/// Which page an address is on, and how far into that page it is.
pub fn split(addr: usize) -> (usize, usize) {
    let ps = size();
    (addr / ps, addr % ps)
}

#[cfg(test)]
mod tests {
    #[test]
    fn page_size_is_a_power_of_two() {
        let ps = super::size();
        assert!(ps >= 4096 && ps.is_power_of_two(), "{ps}");
        assert_eq!(super::split(ps * 3 + 10), (3, 10));
    }
}
