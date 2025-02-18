#![allow(dead_code)]

// Status

pub const AIR: *mut u32 = 0x10976dfc as *mut u32;

pub const HEALTH: *mut u8 = 0x1506b503 as *mut u8;

pub const CONTAINERS: *mut u8 = 0x1506b501 as *mut u8;

pub const MAGIC: *mut u8 = 0x1506b514 as *mut u8;

pub const MAX_MAGIC: *mut u8 = 0x1506b513 as *mut u8;

pub const RUPEES: *mut u16 = 0x1506b504 as *mut u16;

pub const ARROWS: *mut u8 = 0x1506b569 as *mut u8;

pub const MAX_ARROWS: *mut u8 = 0x1506b56f as *mut u8;

pub const BOMBS: *mut u8 = 0x1506b56a as *mut u8;

pub const MAX_BOMBS: *mut u8 = 0x1506b570 as *mut u8;

pub const SUPER_CROUCH: *mut u32 = 0x10537550 as *mut u32;

pub const OVERWORLD_MAP: *mut [u8; 49] = 0x1506b604 as *mut [u8; 49];

// Equipped items

pub const BUTTON_X: *mut u8 = 0x10976e6b as *mut u8;

pub const BUTTON_Y: *mut u8 = 0x10976e6c as *mut u8;

pub const BUTTON_R: *mut u8 = 0x10976e6d as *mut u8;

// Location

pub mod position {
    pub const X: *mut f32 = 0x1096ef48 as *mut f32;

    pub const Y: *mut f32 = 0x1096ef50 as *mut f32;

    pub const Z: *mut f32 = 0x1096ef4c as *mut f32;

    pub const ANGLE: *mut u32 = 0x1096ef10 as *mut u32;

    // *(0x10989C74 as *mut f32) + 26936 (WWHD Trainer rewrite decompile)
    pub const SPEED_PTR: *mut usize = 0x10989C74 as *mut usize;
    pub const SPEED_OFFSET: usize = 0x6938;
}

pub const HOVER_PTR: *mut usize = 0x10976ab4 as *mut usize;
pub const HOVER_OFFSET: usize = 0x340;

// 15, 07, 36, 94 - magic

// 15, 07, 36, 83 - health

// 15, 07, 36, e9 - arrows

// 15, 07, 36, ef - max arrows

// 15, 07, 36, ea - bombs

// 15, 07, 36, f0 - max bombs

// 10, 97, 6d, fc <- 00, 00, 0d, ff - super swim

// 02000031 15073784
// 00000003 00000001
// 00000000 00000000
// map

//  09020000 102f48a8 // AND 102f48a8 & 0x20
//  00000020 00000000
//  30000000 10976ab4 // LD PTR 10976ab4 [48700000 - 48800000]
//  48700000 48800000
//  00120340 42100000 // WRITE
//  d0000000 deadcafe
