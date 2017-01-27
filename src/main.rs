extern crate rat12;

use std::fs::File;
use rat12::Partition;


fn main() {
    let f = File::open("dev.bin").unwrap();

    let mut part = Partition::new(f);
    {
        // let d = part.fat_boot_sector();
        // println!("{:#?}", d);
        let p = part.fat_parameters();
        println!("{:#?}", p);


    }

}
