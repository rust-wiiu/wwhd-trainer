#![allow(dead_code)]

pub mod collision {
    pub const ADDRESS: *mut usize = 0x10976de4 as *mut usize;
    pub const OFFSET: usize = 2060 + 40;

    pub use Version::*;

    pub enum Version {
        ChestStorage = 0x4,
        DoorCancel = 0x4004,
    }

    #[inline]
    pub fn set(value: u32) {
        unsafe {
            let ptr = core::ptr::read(ADDRESS);
            let ptr = (ptr + OFFSET) as *mut u32;

            if wut::ptr::is_valid(ptr) {
                core::ptr::write(ptr, value);
            }
        }
    }

    #[inline]
    pub fn get() -> u32 {
        unsafe {
            let ptr = core::ptr::read(ADDRESS);
            let ptr = (ptr + OFFSET) as *mut u32;

            if wut::ptr::is_valid(ptr) {
                core::ptr::read(ptr)
            } else {
                0
            }
        }
    }

    #[inline]
    pub fn enable(version: Version) {
        set(get() | version as u32)
    }

    #[inline]
    pub fn disable(version: Version) {
        set(get() & !(version as u32))
    }
}

pub mod storage {
    pub const ADDRESS: *mut u8 = 0x10976543 as *mut u8;

    #[inline]
    pub fn set(value: u8) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn get() -> u8 {
        unsafe { core::ptr::read(ADDRESS) }
    }

    #[inline]
    pub fn enable() {
        set(1);
    }

    #[inline]
    pub fn disable() {
        set(0);
    }
}

pub mod soft_reset {
    pub const ADDRESS: *mut u8 = 0x1098f293 as *mut u8;

    #[inline]
    pub fn set(value: u8) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn get() -> u8 {
        unsafe { core::ptr::read(ADDRESS) }
    }

    #[inline]
    pub fn activate() {
        set(1);
    }
}
