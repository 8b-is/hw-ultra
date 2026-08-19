use core::sync::atomic::{AtomicU8, Ordering};

#[derive(Clone, Copy, PartialEq)]
pub enum NetworkType {
    None,
    Gsm,
    Gprs,
    Edge,
    Umts,
    Hspa,
    HspaPlus,
    Lte,
    LteAdvanced,
    Nr5g,
}

#[derive(Clone, Copy, PartialEq)]
pub enum RegistrationState {
    NotRegistered,
    RegisteredHome,
    Searching,
    Denied,
    Unknown,
    RegisteredRoaming,
}

static NET_TYPE: AtomicU8 = AtomicU8::new(0);
static REG_STATE: AtomicU8 = AtomicU8::new(0);

fn net_to_u8(n: NetworkType) -> u8 {
    match n {
        NetworkType::None => 0,
        NetworkType::Gsm => 1,
        NetworkType::Gprs => 2,
        NetworkType::Edge => 3,
        NetworkType::Umts => 4,
        NetworkType::Hspa => 5,
        NetworkType::HspaPlus => 6,
        NetworkType::Lte => 7,
        NetworkType::LteAdvanced => 8,
        NetworkType::Nr5g => 9,
    }
}

fn u8_to_net(v: u8) -> NetworkType {
    match v {
        1 => NetworkType::Gsm,
        2 => NetworkType::Gprs,
        3 => NetworkType::Edge,
        4 => NetworkType::Umts,
        5 => NetworkType::Hspa,
        6 => NetworkType::HspaPlus,
        7 => NetworkType::Lte,
        8 => NetworkType::LteAdvanced,
        9 => NetworkType::Nr5g,
        _ => NetworkType::None,
    }
}

fn reg_to_u8(r: RegistrationState) -> u8 {
    match r {
        RegistrationState::NotRegistered => 0,
        RegistrationState::RegisteredHome => 1,
        RegistrationState::Searching => 2,
        RegistrationState::Denied => 3,
        RegistrationState::Unknown => 4,
        RegistrationState::RegisteredRoaming => 5,
    }
}

fn u8_to_reg(v: u8) -> RegistrationState {
    match v {
        1 => RegistrationState::RegisteredHome,
        2 => RegistrationState::Searching,
        3 => RegistrationState::Denied,
        4 => RegistrationState::Unknown,
        5 => RegistrationState::RegisteredRoaming,
        _ => RegistrationState::NotRegistered,
    }
}

pub fn set_network_type(net: NetworkType) {
    NET_TYPE.store(net_to_u8(net), Ordering::Release);
}

pub fn current_network_type() -> NetworkType {
    u8_to_net(NET_TYPE.load(Ordering::Acquire))
}

pub fn set_registration(state: RegistrationState) {
    REG_STATE.store(reg_to_u8(state), Ordering::Release);
}

pub fn registration_state() -> RegistrationState {
    u8_to_reg(REG_STATE.load(Ordering::Acquire))
}

pub fn is_connected() -> bool {
    matches!(
        registration_state(),
        RegistrationState::RegisteredHome | RegistrationState::RegisteredRoaming
    )
}

pub fn is_data_capable() -> bool {
    if !is_connected() {
        return false;
    }
    !matches!(current_network_type(), NetworkType::None | NetworkType::Gsm)
}

pub fn parse_creg_stat(stat_byte: u8) -> RegistrationState {
    match stat_byte {
        b'0' => RegistrationState::NotRegistered,
        b'1' => RegistrationState::RegisteredHome,
        b'2' => RegistrationState::Searching,
        b'3' => RegistrationState::Denied,
        b'4' => RegistrationState::Unknown,
        b'5' => RegistrationState::RegisteredRoaming,
        _ => RegistrationState::Unknown,
    }
}
