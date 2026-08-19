use core::sync::atomic::{AtomicU8, Ordering};

pub const MAX_PORTS: usize = 16;

#[derive(Clone, Copy, PartialEq)]
pub enum PortState {
    Disconnected,
    Connected,
    Enabled,
    Suspended,
    Error,
}

#[derive(Clone, Copy, PartialEq)]
pub enum HubSpeed {
    Full,
    High,
    Super,
}

static PORT_STATES: [AtomicU8; MAX_PORTS] = [const { AtomicU8::new(0) }; MAX_PORTS];

static HUB_PORT_COUNT: AtomicU8 = AtomicU8::new(0);

fn state_to_u8(s: PortState) -> u8 {
    match s {
        PortState::Disconnected => 0,
        PortState::Connected => 1,
        PortState::Enabled => 2,
        PortState::Suspended => 3,
        PortState::Error => 4,
    }
}

fn u8_to_state(v: u8) -> PortState {
    match v {
        1 => PortState::Connected,
        2 => PortState::Enabled,
        3 => PortState::Suspended,
        4 => PortState::Error,
        _ => PortState::Disconnected,
    }
}

pub fn set_port_count(count: u8) {
    let c = if count as usize > MAX_PORTS {
        MAX_PORTS as u8
    } else {
        count
    };
    HUB_PORT_COUNT.store(c, Ordering::Release);
}

pub fn port_count() -> u8 {
    HUB_PORT_COUNT.load(Ordering::Acquire)
}

pub fn set_port_state(port: u8, state: PortState) {
    if (port as usize) < MAX_PORTS {
        PORT_STATES[port as usize].store(state_to_u8(state), Ordering::Release);
    }
}

pub fn get_port_state(port: u8) -> PortState {
    if (port as usize) < MAX_PORTS {
        u8_to_state(PORT_STATES[port as usize].load(Ordering::Acquire))
    } else {
        PortState::Disconnected
    }
}

pub fn enumerate_ports(base: usize, cap_length: usize, num_ports: u8) {
    set_port_count(num_ports);
    let mut i: u8 = 0;
    while i < num_ports && (i as usize) < MAX_PORTS {
        let connected = crate::usb::hw::port_connected(base, cap_length, i);
        if connected {
            set_port_state(i, PortState::Connected);
        } else {
            set_port_state(i, PortState::Disconnected);
        }
        i += 1;
    }
}

pub fn reset_port(base: usize, cap_length: usize, port: u8) {
    if (port as usize) < MAX_PORTS {
        crate::usb::hw::port_reset(base, cap_length, port);
    }
}
