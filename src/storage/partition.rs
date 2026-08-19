pub const MBR_SIGNATURE: u16 = 0xAA55;
pub const MBR_PARTITION_TABLE_OFFSET: usize = 446;
pub const MBR_ENTRY_SIZE: usize = 16;
pub const MBR_MAX_PARTITIONS: usize = 4;

pub const GPT_SIGNATURE: u64 = 0x5452415020494645;
pub const GPT_HEADER_LBA: u64 = 1;
pub const GPT_ENTRY_SIZE: usize = 128;
pub const GPT_MAX_PARTITIONS: usize = 128;

#[derive(Clone, Copy)]
pub enum PartitionScheme {
    Mbr,
    Gpt,
    Unknown,
}

#[derive(Clone, Copy)]
pub struct MbrEntry {
    pub status: u8,
    pub partition_type: u8,
    pub start_lba: u32,
    pub sector_count: u32,
}

#[derive(Clone, Copy)]
pub struct GptEntry {
    pub type_guid: [u8; 16],
    pub unique_guid: [u8; 16],
    pub start_lba: u64,
    pub end_lba: u64,
    pub attributes: u64,
}

impl MbrEntry {
    pub fn is_active(&self) -> bool {
        self.status == 0x80
    }

    pub fn is_empty(&self) -> bool {
        self.partition_type == 0
    }

    pub fn size_bytes(&self, sector_size: u32) -> u64 {
        self.sector_count as u64 * sector_size as u64
    }
}

impl GptEntry {
    pub fn is_empty(&self) -> bool {
        let mut all_zero = true;
        let mut i = 0;
        while i < 16 {
            if self.type_guid[i] != 0 {
                all_zero = false;
            }
            i += 1;
        }
        all_zero
    }

    pub fn size_lba(&self) -> u64 {
        if self.end_lba >= self.start_lba {
            self.end_lba - self.start_lba + 1
        } else {
            0
        }
    }

    pub fn size_bytes(&self, sector_size: u64) -> u64 {
        self.size_lba() * sector_size
    }
}

pub fn detect_scheme(sector0: &[u8]) -> PartitionScheme {
    if sector0.len() < 512 {
        return PartitionScheme::Unknown;
    }
    let sig = (sector0[511] as u16) << 8 | sector0[510] as u16;
    if sig != MBR_SIGNATURE {
        return PartitionScheme::Unknown;
    }
    let off = MBR_PARTITION_TABLE_OFFSET;
    if sector0[off + 4] == 0xEE {
        return PartitionScheme::Gpt;
    }
    PartitionScheme::Mbr
}

pub fn parse_mbr_entry(sector0: &[u8], index: usize) -> Option<MbrEntry> {
    if index >= MBR_MAX_PARTITIONS || sector0.len() < 512 {
        return None;
    }
    let off = MBR_PARTITION_TABLE_OFFSET + index * MBR_ENTRY_SIZE;
    let status = sector0[off];
    let partition_type = sector0[off + 4];
    let start_lba = u32::from_le_bytes([
        sector0[off + 8],
        sector0[off + 9],
        sector0[off + 10],
        sector0[off + 11],
    ]);
    let sector_count = u32::from_le_bytes([
        sector0[off + 12],
        sector0[off + 13],
        sector0[off + 14],
        sector0[off + 15],
    ]);
    Some(MbrEntry {
        status,
        partition_type,
        start_lba,
        sector_count,
    })
}

pub fn parse_gpt_entry(entry_data: &[u8]) -> Option<GptEntry> {
    if entry_data.len() < GPT_ENTRY_SIZE {
        return None;
    }
    let mut type_guid = [0u8; 16];
    let mut unique_guid = [0u8; 16];
    let mut i = 0;
    while i < 16 {
        type_guid[i] = entry_data[i];
        unique_guid[i] = entry_data[16 + i];
        i += 1;
    }
    let start_lba = u64::from_le_bytes([
        entry_data[32],
        entry_data[33],
        entry_data[34],
        entry_data[35],
        entry_data[36],
        entry_data[37],
        entry_data[38],
        entry_data[39],
    ]);
    let end_lba = u64::from_le_bytes([
        entry_data[40],
        entry_data[41],
        entry_data[42],
        entry_data[43],
        entry_data[44],
        entry_data[45],
        entry_data[46],
        entry_data[47],
    ]);
    let attributes = u64::from_le_bytes([
        entry_data[48],
        entry_data[49],
        entry_data[50],
        entry_data[51],
        entry_data[52],
        entry_data[53],
        entry_data[54],
        entry_data[55],
    ]);
    Some(GptEntry {
        type_guid,
        unique_guid,
        start_lba,
        end_lba,
        attributes,
    })
}
