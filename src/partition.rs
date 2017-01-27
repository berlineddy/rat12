use std::io::{Read, Seek, Write};
use std::sync::Arc;
use std::sync::RwLock;
use std::ops::DerefMut;

use FatBootSector;
use FatParameters;

pub trait Volume: Read + Seek + Write {}
impl<T> Volume for T where T: Read + Seek + Write {}

pub struct Partition<T: Volume> {
    handle: Arc<RwLock<T>>,
}

impl<T: Volume> Partition<T> {
    pub fn new(descriptor: T) -> Partition<T> {
        Partition { handle: Arc::new(RwLock::new(descriptor)) }
    }

    pub fn fat_boot_sector(&mut self) -> FatBootSector {
        FatBootSector::new(self.handle.as_ref().write().expect("foo").deref_mut())
            .expect("failed to read fat_boot_sector")
    }

    pub fn fat_parameters(&mut self) -> FatParameters {
        FatParameters::new(&self.fat_boot_sector())
    }

    pub fn get_volume(&mut self) -> Arc<RwLock<T>> {
        return self.handle.clone();
    }
}
