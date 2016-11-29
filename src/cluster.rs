use std::io::{Read, Seek, Error, SeekFrom, Write};
use std::io;
use std::cell::RefCell;
use std::sync::Arc;
use Disk;

pub struct Cluster<'a, T: Read + Seek + Write + 'a> {
    begin: u64,
    size: u64,
    seek: u64,
    disk: &'a mut Disk<T>,
}

impl<'a, T: Read + Seek + Write + 'a> Cluster<'a, T> {
    pub fn new(descriptor: &mut Disk<T>, begin: u64, size: u64) -> Cluster<T> {
        Cluster {
            begin: begin,
            size: size,
            disk: descriptor,
            seek: 0,
        }
    }
}


impl<'a, T: Read + Seek + Write + 'a> Read for Cluster<'a, T> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let len = buf.len();
        Err(Error::from_raw_os_error(-1))
    }
}


impl<'a, T: Read + Seek + Write + 'a> Seek for Cluster<'a, T> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        Err(Error::from_raw_os_error(-1))
    }
}
