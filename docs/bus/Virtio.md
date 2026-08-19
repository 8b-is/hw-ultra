# VirtIO — Virtual I/O Devices

## Overview

The `virtio` module handles detection and initialization of VirtIO paravirtualized devices, commonly found in QEMU/KVM virtual machines.

## Struct

```
Virtio {
    device_id: u32     — VirtIO device type (1=net, 2=block, etc.)
    features: usize    — negotiated feature bits
}
```

## API

| Function / Method | Description |
|-------------------|-------------|
| `new(device_id)` | Creates a VirtIO device handle |
| `negotiate_features(host_features)` | ANDs guest and host feature bitmasks |
| `init()` | Runs the VirtIO initialization sequence |
| `detect_virtio_devices()` | Scans for VirtIO devices, returns count |
| `virtio_device_count()` | Number of detected VirtIO devices |

## Device types

| ID | Device type |
|----|-------------|
| 1 | Network |
| 2 | Block |
| 3 | Console |
| 4 | Entropy (RNG) |
| 5 | Balloon |
| 16 | GPU |

## Initialization sequence

1. Reset the device (write 0 to status)
2. Set ACKNOWLEDGE status bit
3. Set DRIVER status bit
4. Read host features, negotiate with `negotiate_features()`
5. Set FEATURES_OK
6. Verify FEATURES_OK is still set
7. Set DRIVER_OK — device is live
