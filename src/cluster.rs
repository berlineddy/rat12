use std::io::{Read, Seek, Error, SeekFrom, Write};
use std::io;
use std::sync::Arc;
use std::sync::RwLock;

use FatParameters;
use Volume;

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

#[derive(Debug,Default)]
pub struct Cluster<T: Volume> {
    pub header: ClusterHeader,
    pub address: u32,
    pub partition: Arc<RwLock<T>>,
}


impl<T: Volume> Cluster<T> {
    pub fn new(mut descriptor: Arc<RwLock<T>>,
               parameters: FatParameters,
               number: u32)
               -> Cluster<T> {

        assert!(parameters.cluster_count > number);
        let address = parameters.first_data_secotr +
                      number * parameters.bios_paramters.sectors_per_cluster as u32;

        Cluster {
            header: ClusterHeader::Free,
            address: address,
            partition: descriptor,
        }
    }
}


impl<T: Volume> Read for Cluster<T> {
    fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
        Err(Error::from_raw_os_error(-1))
    }
}
impl<T: Volume> Seek for Cluster<T> {
    fn seek(&mut self, _pos: SeekFrom) -> io::Result<u64> {
        Err(Error::from_raw_os_error(-1))
    }
}
impl<T: Volume> Write for Cluster<T> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Err(Error::from_raw_os_error(-1))
    }
    fn flush(&mut self) -> io::Result<()> {
        Err(Error::from_raw_os_error(-1))
    }
}
