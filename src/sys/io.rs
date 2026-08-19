use super::file::sys_write_fd;

pub fn write_stderr(msg: &[u8]) {
    sys_write_fd(2, msg);
}

pub fn write_stderr_str(s: &str) {
    write_stderr(s.as_bytes());
}

pub fn write_stderr_u64(n: u64) {
    let mut buf = [0u8; 20];
    let s = fmt_u64(n, &mut buf);
    write_stderr(s);
}

pub fn fmt_u64(mut n: u64, buf: &mut [u8; 20]) -> &[u8] {
    if n == 0 {
        buf[19] = b'0';
        return &buf[19..20];
    }
    let mut pos = 20;
    while n > 0 && pos > 0 {
        pos -= 1;
        buf[pos] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    &buf[pos..20]
}

pub fn parse_decimal_u64(buf: &[u8]) -> u64 {
    let mut val: u64 = 0;
    let mut i = 0;
    while i < buf.len() && buf[i].is_ascii_digit() {
        val = val * 10 + (buf[i] - b'0') as u64;
        i += 1;
    }
    val
}

pub fn parse_hex_u16_pub(buf: &[u8]) -> u16 {
    let mut val: u16 = 0;
    let start = if buf.len() >= 2 && buf[0] == b'0' && (buf[1] == b'x' || buf[1] == b'X') {
        2
    } else {
        0
    };
    let mut i = start;
    while i < buf.len() {
        let digit = match buf[i] {
            b'0'..=b'9' => buf[i] - b'0',
            b'a'..=b'f' => buf[i] - b'a' + 10,
            b'A'..=b'F' => buf[i] - b'A' + 10,
            _ => break,
        };
        val = val.wrapping_shl(4) | digit as u16;
        i += 1;
    }
    val
}
