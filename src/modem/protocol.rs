pub const AT_PREFIX: &[u8] = b"AT";
pub const AT_CR: u8 = b'\r';

pub const MAX_CMD_LEN: usize = 128;
pub const MAX_RESPONSE_LEN: usize = 256;

#[derive(Clone, Copy, PartialEq)]
pub enum AtResult {
    Ok,
    Error,
    Timeout,
    Incomplete,
}

pub struct AtCommand {
    pub buf: [u8; MAX_CMD_LEN],
    pub len: usize,
}

pub struct AtResponse {
    pub buf: [u8; MAX_RESPONSE_LEN],
    pub len: usize,
    pub result: AtResult,
}

impl AtCommand {
    pub const fn empty() -> Self {
        AtCommand {
            buf: [0u8; MAX_CMD_LEN],
            len: 0,
        }
    }

    pub fn from_bytes(cmd: &[u8]) -> Self {
        let mut at = AtCommand::empty();
        let mut i = 0;
        while i < AT_PREFIX.len() && i < MAX_CMD_LEN {
            at.buf[i] = AT_PREFIX[i];
            i += 1;
        }
        let mut j = 0;
        while j < cmd.len() && i < MAX_CMD_LEN {
            at.buf[i] = cmd[j];
            i += 1;
            j += 1;
        }
        if i < MAX_CMD_LEN {
            at.buf[i] = AT_CR;
            i += 1;
        }
        at.len = i;
        at
    }

    pub fn bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

impl AtResponse {
    pub const fn empty() -> Self {
        AtResponse {
            buf: [0u8; MAX_RESPONSE_LEN],
            len: 0,
            result: AtResult::Incomplete,
        }
    }

    pub fn push_byte(&mut self, b: u8) -> bool {
        if self.len >= MAX_RESPONSE_LEN {
            return false;
        }
        self.buf[self.len] = b;
        self.len += 1;
        true
    }

    pub fn check_complete(&mut self) {
        if self.len >= 4 {
            let tail = &self.buf[self.len - 4..self.len];
            if tail == b"\r\nOK" || (self.len >= 2 && &self.buf[self.len - 2..self.len] == b"OK") {
                self.result = AtResult::Ok;
                return;
            }
        }
        if self.len >= 7 {
            let tail = &self.buf[self.len - 7..self.len];
            if tail == b"\r\nERROR" || tail == b"ERROR\r\n" {
                self.result = AtResult::Error;
            }
        }
    }

    pub fn bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }
}

pub fn cmd_signal_quality() -> AtCommand {
    AtCommand::from_bytes(b"+CSQ")
}

pub fn cmd_network_registration() -> AtCommand {
    AtCommand::from_bytes(b"+CREG?")
}

pub fn cmd_operator() -> AtCommand {
    AtCommand::from_bytes(b"+COPS?")
}

pub fn cmd_sim_status() -> AtCommand {
    AtCommand::from_bytes(b"+CPIN?")
}

pub fn cmd_imei() -> AtCommand {
    AtCommand::from_bytes(b"+GSN")
}

pub fn cmd_model() -> AtCommand {
    AtCommand::from_bytes(b"+CGMM")
}
