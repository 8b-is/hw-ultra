# Net — Ethernet

## Overview

The `ethernet` module provides Ethernet frame parsing and `EtherType` classification.

## EtherType

```rust
pub enum EtherType {
    Ipv4    = 0x0800,   // Internet Protocol v4
    Arp     = 0x0806,   // Address Resolution Protocol
    Ipv6    = 0x86DD,   // Internet Protocol v6
    Vlan    = 0x8100,   // 802.1Q VLAN tagged
    Unknown = 0xFFFF,   // Unrecognized type
}
```

`from_u16(v: u16) -> Self` converts a raw value. Unrecognized values map to `Unknown`.

## EthernetFrame

```
EthernetFrame {
    dst: [u8; 6]            — destination MAC (6 bytes)
    src: [u8; 6]            — source MAC (6 bytes)
    ether_type: EtherType   — protocol indicator
    payload_len: usize      — payload length (total - 14 byte header)
}
```

## Parsing

`EthernetFrame::parse(data: &[u8]) -> Option<Self>`

| Offset | Length | Field |
|--------|--------|-------|
| 0 | 6 | Destination MAC |
| 6 | 6 | Source MAC |
| 12 | 2 | EtherType (big-endian) |
| 14 | ... | Payload |

Returns `None` if `data.len() < 14`.

## Note

This module does not handle 802.1Q VLAN tag parsing (4 extra bytes after src MAC). VLAN-tagged frames will have `ether_type == Vlan` and the real EtherType at offset 16.
