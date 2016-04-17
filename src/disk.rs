use std::io::{Read,Seek,Error,SeekFrom,Write};
use std::io;
use std::cell::RefCell;
use std::sync::{Mutex,LockResult};
use std::ops::Deref;
use std::boxed::Box;
use std::borrow::Borrow;
use std::ops::DerefMut;

use Cluster;
use VolumeBootRecord;

pub struct Disk< T: Read + Seek + Write > {
    disk: Mutex<RefCell<T>>,
}

impl<T: Read + Seek + Write > Disk<T> {
    pub fn new(descriptor: T ) -> Disk<T> {
        Disk {
            disk: Mutex::new(RefCell::new(descriptor)),
        }
    }

    pub fn cluster(&mut self, begin: u64, size: u64) -> Cluster<T> {
        Cluster::new(self, begin, size)
    }

    pub fn volume_boot_record(&mut self) -> io::Result<VolumeBootRecord> {
        VolumeBootRecord::new(self)
    }
}

impl <T: Read + Seek + Write> Read for Disk<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize>{
        let mut b = self.disk.lock().unwrap();
        let mut r : &RefCell<T> = b.borrow();
        let mut d = r.borrow_mut();
        let mut a = d.deref_mut();
        a.read(buf)
    }
}

impl <T: Read + Seek + Write > Seek for Disk<T> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let mut b = self.disk.lock().unwrap();
        let mut r : &RefCell<T> = b.borrow();
        let mut d = r.borrow_mut();
        let mut a = d.deref_mut();
        a.seek(pos)
    }
}


impl  <T: Read + Seek + Write > Write for Disk<T> {
     fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
         Err(Error::from_raw_os_error(-1))
     }

     fn flush(&mut self) -> io::Result<()> {
         Err(Error::from_raw_os_error(-1))
     }
}
