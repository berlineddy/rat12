use FatBootSector;
use Filesystem;
use ExtendedBiosParamterBlock;
use BiosParamterBlock;

#[derive(Debug,Default,Clone,Copy)]
pub struct FatParameters {
    pub root_directory_secotrs: u32,
    pub first_data_secotr: u32,
    pub cluster_count: u32,
    pub first_root_sector: u32,
    pub fat_table_size: u32,
    pub number_of_fat_entries: u32,
    pub bios_paramters: BiosParamterBlock,
    pub extended_bios_paramters: ExtendedBiosParamterBlock,
}
impl FatParameters {
    pub fn new(boot_sector: &FatBootSector) -> FatParameters {
        let frs: u32 = boot_sector.bios_paramters.reserved_sectors as u32 +
                       boot_sector.bios_paramters.number_of_fat_copies as u32 +
                       boot_sector.bios_paramters.sectors_per_fat as u32;
        let rds: u32 = (boot_sector.bios_paramters.number_of_possible_root_entries as u32 * 32) /
                       512;
        let fds: u32 = frs as u32 + rds as u32;
        let sectors: u32 = {
            if boot_sector.bios_paramters.large_number_of_sectors != 0 {
                boot_sector.bios_paramters.large_number_of_sectors as u32
            } else {
                boot_sector.bios_paramters.small_number_of_sectors as u32
            }
        };

        let cc: u32 = (sectors as u32 - fds) /
                      boot_sector.bios_paramters.sectors_per_cluster as u32;

        let fat_size: u32 = match boot_sector.extended_bios_paramters.file_system_type {
            Filesystem::Fat12 => 12,
            Filesystem::Fat16 => 16,
            Filesystem::Fat32 => 32,
            _ => 32,
        };

        let fts: u32 = boot_sector.bios_paramters.number_of_fat_copies as u32 * cc as u32 *
                       fat_size as u32;
        let ne: u32 = fts as u32 / boot_sector.bios_paramters.number_of_fat_copies as u32 /
                      fat_size as u32;
        FatParameters {
            root_directory_secotrs: rds,
            first_data_secotr: fds,
            cluster_count: cc,
            first_root_sector: frs,
            fat_table_size: fts,
            number_of_fat_entries: ne,
            bios_paramters: boot_sector.bios_paramters,
            extended_bios_paramters: boot_sector.extended_bios_paramters,
        }
    }
}
