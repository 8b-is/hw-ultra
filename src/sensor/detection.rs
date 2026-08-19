#[derive(Copy, Clone, PartialEq)]
pub enum SensorKind {
    Accelerometer,
    Gyroscope,
    Magnetometer,
    Barometer,
    Proximity,
    Light,
    Imu,
    Temperature,
    Humidity,
    Gravity,
    Unknown,
}

#[derive(Copy, Clone)]
pub struct SensorDevice {
    pub kind: SensorKind,
    pub reg_base: u64,
    pub irq: u32,
    pub compat: [u8; 64],
    pub compat_len: usize,
}

pub fn detect(out: &mut [SensorDevice]) -> usize {
    if out.is_empty() {
        return 0;
    }
    scan_dt(out)
}

fn scan_dt(out: &mut [SensorDevice]) -> usize {
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
        let kind = classify_sensor(compat, &entries[i].name[..entries[i].name_len]);
        if !matches!(kind, SensorKind::Unknown) {
            let mut compat_buf = [0u8; 64];
            let clen = copy_min(compat, &mut compat_buf);
            out[found] = SensorDevice {
                kind,
                reg_base: entries[i].reg_base,
                irq: entries[i].irq,
                compat: compat_buf,
                compat_len: clen,
            };
            found += 1;
        }
        i += 1;
    }
    found
}

fn classify_sensor(compat: &[u8], name: &[u8]) -> SensorKind {
    if contains(compat, b"mpu6050")
        || contains(compat, b"mpu6500")
        || contains(compat, b"mpu9250")
        || contains(compat, b"icm20")
        || contains(compat, b"bmi160")
        || contains(compat, b"bmi270")
        || contains(compat, b"lsm6ds")
        || contains(name, b"imu")
    {
        return SensorKind::Imu;
    }
    if contains(compat, b"bma")
        || contains(compat, b"lis3dh")
        || contains(compat, b"lis2dh")
        || contains(compat, b"kxtj")
        || contains(compat, b"adxl")
        || contains(compat, b"mc3")
        || contains(name, b"accel")
    {
        return SensorKind::Accelerometer;
    }
    if contains(compat, b"l3g") || contains(compat, b"bmg") || contains(name, b"gyro") {
        return SensorKind::Gyroscope;
    }
    if contains(compat, b"ak8963")
        || contains(compat, b"ak09911")
        || contains(compat, b"mmc5603")
        || contains(compat, b"hmc5883")
        || contains(compat, b"bmm150")
        || contains(name, b"magn")
    {
        return SensorKind::Magnetometer;
    }
    if contains(compat, b"bmp")
        || contains(compat, b"ms5611")
        || contains(compat, b"lps22")
        || contains(compat, b"hp206c")
        || contains(name, b"baro")
    {
        return SensorKind::Barometer;
    }
    if contains(compat, b"stk3")
        || contains(compat, b"apds9")
        || contains(compat, b"vcnl4")
        || contains(name, b"prox")
    {
        return SensorKind::Proximity;
    }
    if contains(compat, b"tsl2")
        || contains(compat, b"bh1750")
        || contains(compat, b"veml")
        || contains(compat, b"ltr5")
        || contains(name, b"light")
        || contains(name, b"als")
    {
        return SensorKind::Light;
    }
    if contains(compat, b"tmp1")
        || contains(compat, b"lm75")
        || contains(compat, b"aht")
        || contains(name, b"temp")
    {
        return SensorKind::Temperature;
    }
    if contains(compat, b"hdc1")
        || contains(compat, b"sht")
        || contains(compat, b"si7021")
        || contains(name, b"humid")
    {
        return SensorKind::Humidity;
    }
    SensorKind::Unknown
}

fn copy_min(src: &[u8], dst: &mut [u8; 64]) -> usize {
    let n = if src.len() < 64 { src.len() } else { 64 };
    let mut i = 0usize;
    while i < n {
        dst[i] = src[i];
        i += 1;
    }
    n
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
