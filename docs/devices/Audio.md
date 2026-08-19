# Audio Module

The `audio` module provides hardware abstraction for audio controllers, codecs, mixers, and stream management.

## Source Layout

| File | Purpose |
|------|---------|
| `device.rs` | `AudioController` struct, controller-level state |
| `codec.rs` | HDA codec verb encoding, `CodecNode` management |
| `hw.rs` | MMIO register offsets, low-level register access |
| `detection.rs` | PCI and device-tree scanning for audio hardware |
| `format.rs` | Sample rate, bit depth, channel layout encoding |
| `mixer.rs` | Gain control, mute/unmute, master volume |
| `stream.rs` | `StreamDescriptor`, start/stop/reset, buffer setup |
| `drivers/` | Vendor-specific driver stubs |
| `lifecycle.rs` | Architecture-specific init dispatch |

## Key Types

### `AudioController`

Central state for one audio device:
- `vendor_id`, `device_id` — PCI identification
- `codec` — detected `AudioCodec` variant
- `hda_bar` — MMIO base address for HDA registers
- `input_streams`, `output_streams` — stream count

### `AudioCodec` (enum)

| Variant | Description |
|---------|-------------|
| `Hda` | Intel High Definition Audio |
| `Ac97` | Legacy AC'97 codec |
| `I2s` | Inter-IC Sound (embedded) |
| `Multimedia` | Generic multimedia audio |
| `DspProcessor` | DSP-based audio |
| `Codec` | Generic codec class |
| `Unknown` | Unrecognized audio device |

### `CodecNode`

Represents one node in the HDA codec tree:
- `codec_id`, `node_id` — addressing within the codec
- `node_type` — `CodecType` (AudioOutput, AudioInput, AudioMixer, PinComplex, etc.)
- `vendor_id`, `subsystem_id` — codec vendor identification

### Stream Types

`StreamDescriptor` holds per-stream state: index, direction (`Output`/`Input`), buffer address and length, format register value, running flag.

`SampleRate` ranges from 8 kHz to 192 kHz. `BitDepth` supports 8 to 32-bit. `Channels` go from Mono to Surround 7.1.

## Detection

`detect()` scans PCI bus for audio class devices (class `0x04`) and queries the device tree for `audio-controller` / `sound` compatibility strings. Returns a populated `AudioController` with codec type inferred from vendor/device IDs.

## Hardware Registers

All register access goes through `hw::read_reg()` / `hw::write_reg()` using volatile MMIO. Key register offsets:

| Register | Description |
|----------|-------------|
| `GCAP` | Global capabilities |
| `GCTL` | Global control |
| `GSTS` | Global status |
| `INTCTL` | Interrupt control |

## Codec Verbs

The codec subsystem builds HDA verb commands:
- `verb_get_parameter()` — query codec node parameters
- `verb_set_stream()` — assign a stream to a converter
- `verb_set_pin_control()` — configure pin widget direction
- `verb_set_power_state()` — manage codec node power

## Mixer

Gain control via `set_gain()`, `set_master_gain()`. Mute management with `mute()`, `unmute()`, `mute_all()`. All gain values stored as atomics for safe concurrent access.

## Stream Management

Each stream supports `start()`, `stop()`, `reset()`, `set_format()`, and `set_buffer()`. The format register is encoded by `encode_hda_format()` from sample rate, bit depth, and channel count.

## Lifecycle

`lifecycle::init()` dispatches to `x86_64::audio::init_audio()` or `aarch64::audio::init_audio()` with the controller's MMIO base address. State transitions signaled via atomic XOR.

## Constants

- `MAX_CHANNELS` = 8
- Stream direction discriminated by `StreamDirection::Output` / `StreamDirection::Input`
