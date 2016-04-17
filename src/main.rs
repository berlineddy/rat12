extern crate rat12;

use std::fs::File;
use std::io::Read;
use std::cell::RefCell;
use std::clone::Clone;
use rat12::{VolumeBootRecord,Cluster,Disk};
use std::thread;


fn main() {
    let f = File::open("dev.bin").unwrap();
    //let fat = VolumeBootRecord::new(&mut f);


    let mut disk = Disk::new(f);
    {
        let d = disk.volume_boot_record();
        let x = disk.volume_boot_record();
        println!("{:#?}",d);
        println!("First FAT: {:#?}",disk.firstFat());
        println!("Second FAT: {:#?}",disk.secFat());
        println!("Root Dir: {:#?}",disk.rootDir());
        println!("Data Area: {:#?}",disk.dataArea());
    }
    //let c1 = Cluster::new(f, 1024, 512).unwrap();
    //let c2 = Cluster::new(f, 1124, 512).unwrap();

    //c1.chain(c2);


    //println!("{:#?}",fat);
}
