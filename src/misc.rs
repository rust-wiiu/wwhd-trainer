#![allow(dead_code)]

pub const SOFT_RESET: *mut u8 = 0x1098f293 as *mut u8;

pub const LINK_PTR: *mut usize = 0x10976de4 as *mut usize;

pub const COLLISION_OFFSET: usize = 2060 + 40;

pub const STORAGE: *mut u8 = 0x10976543 as *mut u8;

pub fn door_cancel(enabled: bool) {
    unsafe {
        let ptr = (core::ptr::read(LINK_PTR) + COLLISION_OFFSET) as *mut u32;

        if wut::ptr::is_valid(ptr) {
            if enabled {
                let value = core::ptr::read(ptr) | 0x4004;
                core::ptr::write(ptr, value);
            } else {
                let value = core::ptr::read(ptr) & !0x4004;
                core::ptr::write(ptr, value);
            }
        }
    }
}

pub fn chest_storage(enabled: bool) {
    unsafe {
        let ptr = (core::ptr::read(LINK_PTR) + COLLISION_OFFSET) as *mut u32;

        if wut::ptr::is_valid(ptr) {
            if enabled {
                let value = core::ptr::read(ptr) | 0x4;
                core::ptr::write(ptr, value);
            } else {
                let value = core::ptr::read(ptr) & !0x4;
                core::ptr::write(ptr, value);
            }
        }
    }
}

pub fn storage(enabled: bool) {
    unsafe {
        core::ptr::write(STORAGE, if enabled { 1 } else { 0 });
    }
}

pub fn soft_reset() {
    unsafe {
        core::ptr::write(SOFT_RESET, 1);
    }
}
