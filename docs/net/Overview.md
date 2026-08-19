# Net Module

The `net` module provides basic network protocol structures for Ethernet, IPv4, and TCP.

## Submodules

| File | Description |
|------|-------------|
| `ethernet.rs` | Ethernet frame parsing and `EtherType` enum |
| `ipv4.rs` | IPv4 packet structures |
| `tcp.rs` | TCP segment structures |

## EtherType

```rust
pub enum EtherType {
    Ipv4    = 0x0800,
    Arp     = 0x0806,
    Ipv6    = 0x86DD,
    Vlan    = 0x8100,
    Unknown = 0xFFFF,
}
```

`EtherType::from_u16(v)` converts a raw 16-bit value to the enum.

## EthernetFrame

```
EthernetFrame {
    dst: [u8; 6]            — destination MAC address
    src: [u8; 6]            — source MAC address
    ether_type: EtherType   — protocol type
    payload_len: usize      — payload length in bytes
}
```

`EthernetFrame::parse(data: &[u8]) -> Option<Self>`

Parses a raw byte slice into an Ethernet frame. Returns `None` if the data is too short (< 14 bytes).

## Scope

The net module provides protocol data structures only. It does not implement:
- Network interface drivers (handled by device drivers)
- IP routing or ARP resolution
- Socket API
- Full TCP state machine

These structures are used for packet inspection and basic protocol handling in conjunction with NIC device drivers.
