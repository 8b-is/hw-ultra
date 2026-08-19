#[derive(Copy, Clone, PartialEq)]
pub enum AudioCodec {
    Hda,
    Ac97,
    I2s,
    Multimedia,
    DspProcessor,
    Codec,
    Unknown,
}

#[derive(Copy, Clone)]
pub struct AudioController {
    pub vendor_id: u16,
    pub device_id: u16,
    pub codec: AudioCodec,
    pub hda_bar: u32,
    pub output_streams: u8,
    pub input_streams: u8,
    pub bus: u8,
    pub dev: u8,
    pub func: u8,
    pub pci: bool,
    pub reg_base: u64,
}

pub fn detect(out: &mut [AudioController]) -> usize {
    if out.is_empty() {
        return 0;
    }
    let mut found = 0usize;
    found += scan_pci(&mut out[found..]);
    found += scan_dt(&mut out[found..]);
    found
}

fn scan_pci(out: &mut [AudioController]) -> usize {
    let mut devs = [crate::bus::pci::device::PciDevice {
        bus: 0,
        device: 0,
        function: 0,
        vendor_id: 0,
        device_id: 0,
        class: 0,
        subclass: 0,
    }; 64];
    let total = crate::bus::pci::device::scan_all(&mut devs);
    let mut found = 0usize;
    let mut i = 0usize;
    while i < total && found < out.len() {
        if devs[i].class == 0x04 {
            let reg08 = crate::bus::pci::read_config_u32(
                devs[i].bus,
                devs[i].device,
                devs[i].function,
                0x08,
            )
            .unwrap_or(0);
            let prog_if = ((reg08 >> 8) & 0xFF) as u8;

            let bar0 = crate::bus::pci::read_config_u32(
                devs[i].bus,
                devs[i].device,
                devs[i].function,
                0x10,
            )
            .unwrap_or(0);

            let (codec, ostreams, istreams) = classify_audio_pci(devs[i].subclass, prog_if, bar0);
            out[found] = AudioController {
                vendor_id: devs[i].vendor_id,
                device_id: devs[i].device_id,
                codec,
                hda_bar: bar0,
                output_streams: ostreams,
                input_streams: istreams,
                bus: devs[i].bus,
                dev: devs[i].device,
                func: devs[i].function,
                pci: true,
                reg_base: 0,
            };
            found += 1;
        }
        i += 1;
    }
    found
}

fn classify_audio_pci(subclass: u8, prog_if: u8, _bar0: u32) -> (AudioCodec, u8, u8) {
    match subclass {
        0x03 => {
            let codec = if prog_if == 0x00 {
                AudioCodec::Hda
            } else {
                AudioCodec::Unknown
            };
            (codec, 0, 0)
        }
        0x01 => (AudioCodec::Multimedia, 0, 0),
        0x02 => (AudioCodec::Ac97, 2, 1),
        0x80 => (AudioCodec::DspProcessor, 0, 0),
        _ => (AudioCodec::Unknown, 0, 0),
    }
}

fn scan_dt(out: &mut [AudioController]) -> usize {
    if out.is_empty() {
        return 0;
    }
    let mut blob = [0u8; 4096];
    let blen = crate::firmware::devicetree::load_fdt_blob(&mut blob);
    if blen < 40 {
        return 0;
    }
    let mut entries = [crate::firmware::devicetree::DtDeviceEntry {
        name: [0u8; 64],
        name_len: 0,
        reg_base: 0,
        reg_size: 0,
        irq: 0,
        compatible: [0u8; 128],
        compatible_len: 0,
    }; 64];
    let count = crate::firmware::devicetree::enumerate_devices(&blob[..blen], &mut entries);
    let mut found = 0usize;
    let mut i = 0usize;
    while i < count && found < out.len() {
        let compat = &entries[i].compatible[..entries[i].compatible_len];
        let codec = classify_audio_compat(compat);
        if !matches!(codec, AudioCodec::Unknown) {
            out[found] = AudioController {
                vendor_id: 0,
                device_id: 0,
                codec,
                hda_bar: 0,
                output_streams: 0,
                input_streams: 0,
                bus: 0,
                dev: 0,
                func: 0,
                pci: false,
                reg_base: entries[i].reg_base,
            };
            found += 1;
        }
        i += 1;
    }
    found
}

fn classify_audio_compat(compat: &[u8]) -> AudioCodec {
    if contains(compat, b"audio-codec") || contains(compat, b"codec") {
        return AudioCodec::Codec;
    }
    if contains(compat, b"i2s") || contains(compat, b"dai") {
        return AudioCodec::I2s;
    }
    if contains(compat, b"sound") || contains(compat, b"audio") {
        return AudioCodec::Multimedia;
    }
    if contains(compat, b"hda") || contains(compat, b"hdaudio") {
        return AudioCodec::Hda;
    }
    AudioCodec::Unknown
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.len() > haystack.len() {
        return false;
    }
    let mut i = 0usize;
    while i + needle.len() <= haystack.len() {
        let mut ok = true;
        let mut j = 0usize;
        while j < needle.len() {
            if haystack[i + j] != needle[j] {
                ok = false;
                break;
            }
            j += 1;
        }
        if ok {
            return true;
        }
        i += 1;
    }
    false
}
