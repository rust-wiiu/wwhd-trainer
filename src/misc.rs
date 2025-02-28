#![allow(dead_code)]

pub const SOFT_RESET: *mut u8 = 0x1098f293 as *mut u8;

pub const LINK_PTR: *mut usize = 0x10976de4 as *mut usize;

pub const COLLISION_OFFSET: usize = 2060 + 40;

pub const STORAGE: *mut u8 = 0x10976543 as *mut u8;
