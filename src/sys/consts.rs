use crate::arch::shim::os;

pub const O_RDONLY: i32 = 0;
pub const O_WRONLY: i32 = 1;
pub const O_RDWR: i32 = 2;
pub const AF_UNIX: i32 = 1;
pub const SOCK_STREAM: i32 = 1;
pub const F_SETFL: i32 = 4;

pub fn o_creat() -> i32 {
    os::os_o_creat() as i32
}
pub fn o_trunc() -> i32 {
    os::os_o_trunc() as i32
}
pub fn o_nonblock() -> i32 {
    os::os_o_nonblock() as i32
}
pub fn o_excl() -> i32 {
    os::os_o_excl() as i32
}
pub fn o_directory() -> i32 {
    os::os_o_directory() as i32
}
