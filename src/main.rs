extern crate rat12;

use std::fs::File;
use rat12::Disk;


fn main() {
    let f = File::open("dev.bin").unwrap();
    // let fat = VolumeBootRecord::new(&mut f);


    let mut disk = Disk::new(f);
    {
        let d = disk.fat_boot_sector();
        println!("{:#?}", d);
    }
    // let c1 = Cluster::new(f, 1024, 512).unwrap();
    // let c2 = Cluster::new(f, 1124, 512).unwrap();

    // c1.chain(c2);


    // println!("{:#?}",fat);
}
