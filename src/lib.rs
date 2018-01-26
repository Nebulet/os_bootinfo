#![no_std]

extern crate x86_64;
extern crate arrayvec;

use x86_64::PhysAddr;
use arrayvec::ArrayVec;

pub struct BootInfo {
    pub memory_map: ArrayVec<[MemoryRegion; 32]>,
}

impl BootInfo {
    pub fn new() -> Self {
        BootInfo {
            memory_map: ArrayVec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemoryRegion {
    pub start_addr: PhysAddr,
    pub len: u64,
    pub region_type: MemoryRegionType
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryRegionType {
    /// free RAM
    Usable,
    /// used RAM
    InUse,
    /// unusable
    Reserved,
    /// ACPI reclaimable memory
    AcpiReclaimable,
    /// ACPI NVS memory
    AcpiNvs,
    /// Area containing bad memory
    BadMemory,
}

#[repr(C)]
pub struct E820MemoryRegion {
    pub start_addr: u64,
    pub len: u64,
    pub region_type: u32,
    pub acpi_extended_attributes: u32,
}

impl From<E820MemoryRegion> for MemoryRegion {
    fn from(region: E820MemoryRegion) -> MemoryRegion {
        let region_type = match region.region_type {
            1 => MemoryRegionType::Usable,
            2 => MemoryRegionType::Reserved,
            3 => MemoryRegionType::AcpiReclaimable,
            4 => MemoryRegionType::AcpiNvs,
            5 => MemoryRegionType::BadMemory,
            t => panic!("invalid region type {}", t),
        };
        MemoryRegion {
            start_addr: PhysAddr::new(region.start_addr),
            len: region.len,
            region_type
        }
    }
}

#[repr(u32)]
pub enum E820MemoryRegionType {
    /// (normal) RAM
    Usable = 1,
    /// unusable
    Reserved = 2,
    /// ACPI reclaimable memory
    AcpiReclaimable = 3,
    /// ACPI NVS memory
    AcpiNvs = 4,
    /// Area containing bad memory
    BadMemory = 5,
}