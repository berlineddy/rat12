
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::Read;
use std::io;
use std::io::{Seek, SeekFrom};

#[derive(Debug)]
pub enum MediaDescriptor {
    Unknowen,
    HardDisk,
    FloppyDisk(u8),
}

#[derive(Debug)]
pub enum Filesystem {
    Unknowen,
    Fat12,
    Fat16,
    Fat32,
}


#[derive(Debug)]
pub struct BiosParamterBlock {
    pub bytes_per_sector: u16,
    pub sectord_per_cluster: u8,
    pub reserved_sectors: u16,
    pub number_of_fat_copies: u8,
    pub number_of_possible_root_entries: u16,
    pub small_number_of_sectors: u16,
    pub media_descriptor: MediaDescriptor,
    pub sectors_per_fat: u16,
    pub sectors_per_track: u16,
    pub number_if_heads: u16,
    pub number_of_hidden_sectors: u32,
    pub large_number_of_sectors: u32,
}
impl BiosParamterBlock {
    pub fn new<T: Read + Seek>(descriptor: &mut T) -> io::Result<BiosParamterBlock> {
        Ok(BiosParamterBlock {
            bytes_per_sector: descriptor.read_u16::<LittleEndian>()?,
            sectord_per_cluster: descriptor.read_u8()?,
            reserved_sectors: descriptor.read_u16::<LittleEndian>()?,
            number_of_fat_copies: descriptor.read_u8()?,
            number_of_possible_root_entries: descriptor.read_u16::<LittleEndian>()?,
            small_number_of_sectors: descriptor.read_u16::<LittleEndian>()?,
            media_descriptor: match descriptor.read_u8()? {
                0xF8 => MediaDescriptor::HardDisk,
                v @ 0xF9...0xFF => MediaDescriptor::FloppyDisk(v),
                v @ 0xF0 => MediaDescriptor::FloppyDisk(v),
                _ => MediaDescriptor::Unknowen,
            },
            sectors_per_fat: descriptor.read_u16::<LittleEndian>()?,
            sectors_per_track: descriptor.read_u16::<LittleEndian>()?,
            number_if_heads: descriptor.read_u16::<LittleEndian>()?,
            number_of_hidden_sectors: descriptor.read_u32::<LittleEndian>()?,
            large_number_of_sectors: descriptor.read_u32::<LittleEndian>()?,
        })
    }
}


#[derive(Debug)]
pub struct ExtendedBiosParamterBlock {
    pub drive_number: u8,
    pub check_disk_integrity: u8,
    pub extended_boot_signature: u8,
    pub volume_serial_number: String,
    pub volume_label: String,
    pub file_system_type: Filesystem,
}
impl ExtendedBiosParamterBlock {
    pub fn new<T: Read + Seek>(descriptor: &mut T) -> io::Result<ExtendedBiosParamterBlock> {
        Ok(ExtendedBiosParamterBlock {
            drive_number: descriptor.read_u8()?,
            check_disk_integrity: descriptor.read_u8()?,
            extended_boot_signature: descriptor.read_u8()?,
            volume_serial_number: {
                let mut buffer: [u8; 0x4] = [0; 0x4];
                descriptor.read(&mut buffer)?;
                format!("{:X}{:X}{:X}{:X}",
                        buffer[0],
                        buffer[1],
                        buffer[2],
                        buffer[3])
            },
            volume_label: {
                let mut buffer: [u8; 0xB] = [0; 0xB];
                descriptor.read(&mut buffer)?;
                String::from_utf8_lossy(&buffer[0..0x0B]).into_owned()
            },
            file_system_type: {
                let mut _buffer: [u8; 0x8] = [0; 0x8];
                descriptor.read(&mut _buffer)?;
                match &*String::from_utf8_lossy(&_buffer[0..0x8]) {
                    "FAT12   " => Filesystem::Fat12,
                    "FAT16   " => Filesystem::Fat16,
                    "FAT32   " => Filesystem::Fat32,
                    _ => Filesystem::Unknowen,
                }
            },
        })
    }
}

#[derive(Debug)]
pub struct FatBootSector {
    pub code: Vec<u8>,
    pub os_name: String,
    pub bios_paramters: BiosParamterBlock,
    pub extended_bios_paramter: ExtendedBiosParamterBlock,
    pub operation_system: Vec<u8>,
    pub signature: u16,
}
impl FatBootSector {
    pub fn new<T: Read + Seek>(descriptor: &mut T) -> io::Result<FatBootSector> {
        descriptor.seek(SeekFrom::Start(0));

        Ok(FatBootSector {
            code: {
                let mut buffer: [u8; 0x3] = [0; 0x3];
                descriptor.read(&mut buffer)?;
                assert!(buffer == [0xeb, 0x3c, 0x90], "no valid FAT entrypoint!");
                buffer.to_vec()
            },
            os_name: {
                let mut buffer: [u8; 0x8] = [0; 0x8];
                descriptor.read(&mut buffer)?;
                String::from_utf8_lossy(&buffer[0..0x08]).into_owned()
            },
            bios_paramters: BiosParamterBlock::new(descriptor)?,
            extended_bios_paramter: ExtendedBiosParamterBlock::new(descriptor)?,
            operation_system: {
                let mut buffer: [u8; 0x1C0] = [0; 0x1C0];
                descriptor.read(&mut buffer)?;
                buffer.to_vec()
            },
            signature: descriptor.read_u16::<LittleEndian>()?,
        })
    }
}
