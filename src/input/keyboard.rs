use core::sync::atomic::{AtomicU8, Ordering};

pub const KEY_ESCAPE: u8 = 0x01;
pub const KEY_1: u8 = 0x02;
pub const KEY_2: u8 = 0x03;
pub const KEY_3: u8 = 0x04;
pub const KEY_4: u8 = 0x05;
pub const KEY_5: u8 = 0x06;
pub const KEY_6: u8 = 0x07;
pub const KEY_7: u8 = 0x08;
pub const KEY_8: u8 = 0x09;
pub const KEY_9: u8 = 0x0A;
pub const KEY_0: u8 = 0x0B;
pub const KEY_MINUS: u8 = 0x0C;
pub const KEY_EQUALS: u8 = 0x0D;
pub const KEY_BACKSPACE: u8 = 0x0E;
pub const KEY_TAB: u8 = 0x0F;
pub const KEY_Q: u8 = 0x10;
pub const KEY_W: u8 = 0x11;
pub const KEY_E: u8 = 0x12;
pub const KEY_R: u8 = 0x13;
pub const KEY_T: u8 = 0x14;
pub const KEY_Y: u8 = 0x15;
pub const KEY_U: u8 = 0x16;
pub const KEY_I: u8 = 0x17;
pub const KEY_O: u8 = 0x18;
pub const KEY_P: u8 = 0x19;
pub const KEY_ENTER: u8 = 0x1C;
pub const KEY_A: u8 = 0x1E;
pub const KEY_S: u8 = 0x1F;
pub const KEY_D: u8 = 0x20;
pub const KEY_F: u8 = 0x21;
pub const KEY_G: u8 = 0x22;
pub const KEY_H: u8 = 0x23;
pub const KEY_J: u8 = 0x24;
pub const KEY_K: u8 = 0x25;
pub const KEY_L: u8 = 0x26;
pub const KEY_Z: u8 = 0x2C;
pub const KEY_X: u8 = 0x2D;
pub const KEY_C: u8 = 0x2E;
pub const KEY_V: u8 = 0x2F;
pub const KEY_B: u8 = 0x30;
pub const KEY_N: u8 = 0x31;
pub const KEY_M: u8 = 0x32;
pub const KEY_SPACE: u8 = 0x39;
pub const KEY_LSHIFT: u8 = 0x2A;
pub const KEY_RSHIFT: u8 = 0x36;
pub const KEY_LCTRL: u8 = 0x1D;
pub const KEY_LALT: u8 = 0x38;
pub const KEY_CAPSLOCK: u8 = 0x3A;
pub const KEY_F1: u8 = 0x3B;
pub const KEY_F2: u8 = 0x3C;
pub const KEY_F3: u8 = 0x3D;
pub const KEY_F4: u8 = 0x3E;
pub const KEY_F5: u8 = 0x3F;
pub const KEY_F6: u8 = 0x40;
pub const KEY_F7: u8 = 0x41;
pub const KEY_F8: u8 = 0x42;
pub const KEY_F9: u8 = 0x43;
pub const KEY_F10: u8 = 0x44;
pub const KEY_F11: u8 = 0x57;
pub const KEY_F12: u8 = 0x58;

pub const MODIFIER_SHIFT: u8 = 1 << 0;
pub const MODIFIER_CTRL: u8 = 1 << 1;
pub const MODIFIER_ALT: u8 = 1 << 2;
pub const MODIFIER_CAPSLOCK: u8 = 1 << 3;

static MODIFIERS: AtomicU8 = AtomicU8::new(0);

pub fn is_break_code(scancode: u8) -> bool {
    scancode & 0x80 != 0
}

pub fn make_code(scancode: u8) -> u8 {
    scancode & 0x7F
}

pub fn update_modifiers(scancode: u8) {
    let pressed = !is_break_code(scancode);
    let key = make_code(scancode);
    let mut mods = MODIFIERS.load(Ordering::Acquire);

    match key {
        KEY_LSHIFT | KEY_RSHIFT => {
            if pressed {
                mods |= MODIFIER_SHIFT;
            } else {
                mods &= !MODIFIER_SHIFT;
            }
        }
        KEY_LCTRL => {
            if pressed {
                mods |= MODIFIER_CTRL;
            } else {
                mods &= !MODIFIER_CTRL;
            }
        }
        KEY_LALT => {
            if pressed {
                mods |= MODIFIER_ALT;
            } else {
                mods &= !MODIFIER_ALT;
            }
        }
        KEY_CAPSLOCK => {
            if pressed {
                mods ^= MODIFIER_CAPSLOCK;
            }
        }
        _ => {}
    }
    MODIFIERS.store(mods, Ordering::Release);
}

pub fn current_modifiers() -> u8 {
    MODIFIERS.load(Ordering::Acquire)
}

pub fn shift_active() -> bool {
    let m = current_modifiers();
    (m & MODIFIER_SHIFT != 0) ^ (m & MODIFIER_CAPSLOCK != 0)
}

pub fn scancode_to_ascii(scancode: u8) -> Option<u8> {
    let key = make_code(scancode);
    if is_break_code(scancode) {
        return None;
    }
    let shifted = shift_active();
    let ch = match key {
        KEY_1 => {
            if shifted {
                b'!'
            } else {
                b'1'
            }
        }
        KEY_2 => {
            if shifted {
                b'@'
            } else {
                b'2'
            }
        }
        KEY_3 => {
            if shifted {
                b'#'
            } else {
                b'3'
            }
        }
        KEY_4 => {
            if shifted {
                b'$'
            } else {
                b'4'
            }
        }
        KEY_5 => {
            if shifted {
                b'%'
            } else {
                b'5'
            }
        }
        KEY_6 => {
            if shifted {
                b'^'
            } else {
                b'6'
            }
        }
        KEY_7 => {
            if shifted {
                b'&'
            } else {
                b'7'
            }
        }
        KEY_8 => {
            if shifted {
                b'*'
            } else {
                b'8'
            }
        }
        KEY_9 => {
            if shifted {
                b'('
            } else {
                b'9'
            }
        }
        KEY_0 => {
            if shifted {
                b')'
            } else {
                b'0'
            }
        }
        KEY_MINUS => {
            if shifted {
                b'_'
            } else {
                b'-'
            }
        }
        KEY_EQUALS => {
            if shifted {
                b'+'
            } else {
                b'='
            }
        }
        KEY_Q => {
            if shifted {
                b'Q'
            } else {
                b'q'
            }
        }
        KEY_W => {
            if shifted {
                b'W'
            } else {
                b'w'
            }
        }
        KEY_E => {
            if shifted {
                b'E'
            } else {
                b'e'
            }
        }
        KEY_R => {
            if shifted {
                b'R'
            } else {
                b'r'
            }
        }
        KEY_T => {
            if shifted {
                b'T'
            } else {
                b't'
            }
        }
        KEY_Y => {
            if shifted {
                b'Y'
            } else {
                b'y'
            }
        }
        KEY_U => {
            if shifted {
                b'U'
            } else {
                b'u'
            }
        }
        KEY_I => {
            if shifted {
                b'I'
            } else {
                b'i'
            }
        }
        KEY_O => {
            if shifted {
                b'O'
            } else {
                b'o'
            }
        }
        KEY_P => {
            if shifted {
                b'P'
            } else {
                b'p'
            }
        }
        KEY_A => {
            if shifted {
                b'A'
            } else {
                b'a'
            }
        }
        KEY_S => {
            if shifted {
                b'S'
            } else {
                b's'
            }
        }
        KEY_D => {
            if shifted {
                b'D'
            } else {
                b'd'
            }
        }
        KEY_F => {
            if shifted {
                b'F'
            } else {
                b'f'
            }
        }
        KEY_G => {
            if shifted {
                b'G'
            } else {
                b'g'
            }
        }
        KEY_H => {
            if shifted {
                b'H'
            } else {
                b'h'
            }
        }
        KEY_J => {
            if shifted {
                b'J'
            } else {
                b'j'
            }
        }
        KEY_K => {
            if shifted {
                b'K'
            } else {
                b'k'
            }
        }
        KEY_L => {
            if shifted {
                b'L'
            } else {
                b'l'
            }
        }
        KEY_Z => {
            if shifted {
                b'Z'
            } else {
                b'z'
            }
        }
        KEY_X => {
            if shifted {
                b'X'
            } else {
                b'x'
            }
        }
        KEY_C => {
            if shifted {
                b'C'
            } else {
                b'c'
            }
        }
        KEY_V => {
            if shifted {
                b'V'
            } else {
                b'v'
            }
        }
        KEY_B => {
            if shifted {
                b'B'
            } else {
                b'b'
            }
        }
        KEY_N => {
            if shifted {
                b'N'
            } else {
                b'n'
            }
        }
        KEY_M => {
            if shifted {
                b'M'
            } else {
                b'm'
            }
        }
        KEY_SPACE => b' ',
        KEY_ENTER => b'\n',
        KEY_TAB => b'\t',
        KEY_BACKSPACE => 0x08,
        _ => return None,
    };
    Some(ch)
}
