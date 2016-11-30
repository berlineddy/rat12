use std::io::{Read, Seek, Error, SeekFrom, Write};
use std::io;
use Disk;

#[derive(Debug)]
pub enum ClusterHeader {
    Free,
    Next(u32),
    Defect,
    Last,
}
impl Default for ClusterHeader {
    fn default() -> ClusterHeader {
        ClusterHeader::Free
    }
}

pub struct Cluster<'a, T: Read + Seek + Write + 'a> {
    pub header: ClusterHeader,
    pub address: u32,
    pub disk: &'a mut Disk<T>,
}

impl<'a, T: Read + Seek + Write + 'a> Cluster<'a, T> {
    pub fn new(descriptor: &mut Disk<T>, address: u32) -> Cluster<T> {
        Cluster {
            header: ClusterHeader::Free,
            address: address,
            disk: descriptor,
        }
    }
}


impl<'a, T: Read + Seek + Write + 'a> Read for Cluster<'a, T> {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(Error::from_raw_os_error(-1))
    }
}


impl<'a, T: Read + Seek + Write + 'a> Seek for Cluster<'a, T> {
    fn seek(&mut self, _pos: SeekFrom) -> io::Result<u64> {
        Err(Error::from_raw_os_error(-1))
    }
}
