//! `memscope stack`: each call pushes a frame at a lower address.

/// Address of a local in each nested call, outermost first.
pub fn frame_addresses(depth: usize) -> Vec<usize> {
    fn go(level: usize, max: usize, out: &mut Vec<usize>) {
        let local = level; // lives in this call's frame
        out.push(std::hint::black_box(&local) as *const usize as usize);
        if level + 1 < max {
            go(level + 1, max, out);
        }
    }
    let mut out = Vec::with_capacity(depth);
    go(0, depth, &mut out);
    out
}

pub fn run(depth: usize) {
    let addrs = frame_addresses(depth);
    println!("Each nested call's local variable (stack grows DOWN):\n");
    for (i, a) in addrs.iter().enumerate() {
        let step = if i == 0 { String::new() } else { format!("  ({} bytes below the previous frame)", addrs[i - 1] - a) };
        println!("  {}call {i}: {a:#x}{step}", "  ".repeat(i));
    }

    let a: i32 = 1;
    let b: i32 = 2;
    let (pa, pb) = (&a as *const i32 as usize, &b as *const i32 as usize);
    println!("\nTwo i32 locals in one frame: {pa:#x} and {pb:#x}, {} bytes apart (an i32 is 4 bytes)", pa.abs_diff(pb));
}

#[cfg(test)]
mod tests {
    #[test]
    fn deeper_calls_have_lower_addresses() {
        let addrs = super::frame_addresses(6);
        assert_eq!(addrs.len(), 6);
        assert!(addrs.windows(2).all(|w| w[1] < w[0]), "stack grows down: {addrs:x?}");
    }
}
