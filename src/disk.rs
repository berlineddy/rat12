use std::io::{Read, Seek, SeekFrom, Write};
use std::io;
use std::cell::RefCell;
use std::sync::Mutex;
use std::borrow::Borrow;
use std::ops::DerefMut;

use FatBootSector;


pub struct Disk<T: Read + Seek + Write> {
    disk: Mutex<RefCell<T>>,
}

impl<T: Read + Seek + Write> Disk<T> {
    pub fn new(descriptor: T) -> Disk<T> {
        Disk { disk: Mutex::new(RefCell::new(descriptor)) }
    }

    pub fn fat_boot_sector(&mut self) -> FatBootSector {
        FatBootSector::new(self).expect("failed to read fat_boot_sector")
    }
}

impl<T: Read + Seek + Write> Read for Disk<T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let b = self.disk.lock().unwrap();
        let r: &RefCell<T> = b.borrow();
        let mut d = r.borrow_mut();
        let mut a = d.deref_mut();
        a.read(buf)
    }
}

impl<T: Read + Seek + Write> Seek for Disk<T> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        let b = self.disk.lock().unwrap();
        let r: &RefCell<T> = b.borrow();
        let mut d = r.borrow_mut();
        let mut a = d.deref_mut();
        a.seek(pos)
    }
}


impl<T: Read + Seek + Write> Write for Disk<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let b = self.disk.lock().unwrap();
        let r: &RefCell<T> = b.borrow();
        let mut d = r.borrow_mut();
        let mut a = d.deref_mut();
        a.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        let b = self.disk.lock().unwrap();
        let r: &RefCell<T> = b.borrow();
        let mut d = r.borrow_mut();
        let mut a = d.deref_mut();
        a.flush()
    }
}
