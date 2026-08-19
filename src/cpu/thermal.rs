pub fn read_core_temperatures(out: &mut [Option<i32>]) -> usize {
    let mut written = 0usize;

    let tjmax = crate::arch::shim::read_msr(0x1A2).map(|v| ((v >> 16) & 0xff) as i32);

    for out_slot in out.iter_mut() {
        let value = crate::arch::shim::read_msr(0x19C);
        if let Some(msr) = value {
            let digital = ((msr >> 16) & 0x7f) as i32;
            if let Some(tj) = tjmax {
                let temp = tj - digital;
                *out_slot = Some(temp);
            } else {
                *out_slot = Some(digital);
            }
            written += 1;
        } else {
            *out_slot = None;
        }
    }

    written
}
