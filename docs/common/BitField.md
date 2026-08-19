# BitField

Utility for extracting and inserting bit fields within 32-bit and 64-bit register values.

## Structure

```rust
pub struct BitField {
    offset: u8,  // bit position of the least significant bit
    width: u8,   // number of bits in the field
}
```

## Methods

```rust
pub fn new(offset: u8, width: u8) -> Self
pub fn mask_u32(&self) -> u32       // bitmask for 32-bit value
pub fn mask_u64(&self) -> u64       // bitmask for 64-bit value
pub fn extract_u32(&self, value: u32) -> u32  // extract field from 32-bit value
pub fn extract_u64(&self, value: u64) -> u64  // extract field from 64-bit value
pub fn insert_u32(&self, original: u32, field_val: u32) -> u32  // insert into 32-bit value
pub fn insert_u64(&self, original: u64, field_val: u64) -> u64  // insert into 64-bit value
```

## Example

Reading the APIC ID from CPUID leaf 1, bits [31:24]:

```rust
let apic_id_field = BitField::new(24, 8);
let (_, ebx, _, _) = cpuid_count(1, 0);
let apic_id = apic_id_field.extract_u32(ebx);
```

## How it works

- `mask`: `((1 << width) - 1) << offset`
- `extract`: `(value >> offset) & ((1 << width) - 1)`
- `insert`: `(original & !mask) | ((field_val << offset) & mask)`
