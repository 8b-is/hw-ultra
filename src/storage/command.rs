#[derive(Copy, Clone, PartialEq)]
pub enum CommandOpcode {
    Read,
    Write,
    Flush,
    Trim,
    Identify,
}

#[derive(Copy, Clone)]
pub struct Command {
    pub opcode: CommandOpcode,
    pub lba: u64,
    pub sector_count: u32,
    pub buffer_addr: u64,
    pub flags: u32,
}

pub fn read_cmd(lba: u64, sectors: u32, buf: u64) -> Command {
    Command {
        opcode: CommandOpcode::Read,
        lba,
        sector_count: sectors,
        buffer_addr: buf,
        flags: 0,
    }
}

pub fn write_cmd(lba: u64, sectors: u32, buf: u64) -> Command {
    Command {
        opcode: CommandOpcode::Write,
        lba,
        sector_count: sectors,
        buffer_addr: buf,
        flags: 0,
    }
}

pub fn flush_cmd() -> Command {
    Command {
        opcode: CommandOpcode::Flush,
        lba: 0,
        sector_count: 0,
        buffer_addr: 0,
        flags: 0,
    }
}

pub fn trim_cmd(lba: u64, sectors: u32) -> Command {
    Command {
        opcode: CommandOpcode::Trim,
        lba,
        sector_count: sectors,
        buffer_addr: 0,
        flags: 0,
    }
}

pub fn identify_cmd(buf: u64) -> Command {
    Command {
        opcode: CommandOpcode::Identify,
        lba: 0,
        sector_count: 1,
        buffer_addr: buf,
        flags: 0,
    }
}
