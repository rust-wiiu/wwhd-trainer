#![allow(dead_code)]

// Status

// Links ait meter when swimming
pub mod air {
    pub const ADDRESS: *mut u32 = 0x10976dfc as *mut u32;
    pub const MAX: u32 = 900;

    #[inline]
    pub fn write(value: u32) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> u32 {
        unsafe { core::ptr::read(ADDRESS) }
    }
}

/// Links current hearts (4 = 1 Heart)
pub mod health {
    pub const ADDRESS: *mut u8 = 0x1506b503 as *mut u8;
    pub const MAX: u8 = 80;

    #[inline]
    pub fn write(value: u8) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> u8 {
        unsafe { core::ptr::read(ADDRESS) }
    }
}

/// Links heart containers (4 = 1 Container)
pub mod containers {
    pub const ADDRESS: *mut u8 = 0x1506b501 as *mut u8;
    pub const MAX: u8 = 80;

    #[inline]
    pub fn write(value: u8) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> u8 {
        unsafe { core::ptr::read(ADDRESS) }
    }
}

/// Links current magic
pub mod magic {
    pub const ADDRESS: *mut u8 = 0x1506b514 as *mut u8;
    pub const MAX: u8 = 32;

    #[inline]
    pub fn write(value: u8) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> u8 {
        unsafe { core::ptr::read(ADDRESS) }
    }
}

/// Links maximum magic
pub mod max_magic {
    pub const ADDRESS: *mut u8 = 0x1506b513 as *mut u8;
    pub const NONE: u8 = 0;
    pub const NORMAL: u8 = 16;
    pub const DOUBLE: u8 = 32;

    #[inline]
    pub fn write(value: u8) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> u8 {
        unsafe { core::ptr::read(ADDRESS) }
    }
}

/// Links current rupees
pub mod rupees {
    pub const ADDRESS: *mut u16 = 0x1506b504 as *mut u16;
    pub const MAX: u16 = 5000;

    #[inline]
    pub fn write(value: u16) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> u16 {
        unsafe { core::ptr::read(ADDRESS) }
    }
}

/// Links current arrows
pub mod arrows {
    pub const ADDRESS: *mut u8 = 0x1506b569 as *mut u8;
    pub const MAX: u8 = 99;

    #[inline]
    pub fn write(value: u8) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> u8 {
        unsafe { core::ptr::read(ADDRESS) }
    }
}

/// Links maximum arrows
pub mod max_arrows {
    pub const ADDRESS: *mut u8 = 0x1506b56f as *mut u8;
    pub const MAX: u8 = 99;

    #[inline]
    pub fn write(value: u8) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> u8 {
        unsafe { core::ptr::read(ADDRESS) }
    }
}

/// Links current bombs
pub mod bombs {
    pub const ADDRESS: *mut u8 = 0x1506b56a as *mut u8;
    pub const MAX: u8 = 99;

    #[inline]
    pub fn write(value: u8) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> u8 {
        unsafe { core::ptr::read(ADDRESS) }
    }
}

/// Links maximum bombs
pub mod max_bombs {
    pub const ADDRESS: *mut u8 = 0x1506b570 as *mut u8;
    pub const MAX: u8 = 99;

    #[inline]
    pub fn write(value: u8) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> u8 {
        unsafe { core::ptr::read(ADDRESS) }
    }
}

/// Link's super fast during crouching
pub mod super_crouch {
    pub const ADDRESS: *mut u32 = 0x10537550 as *mut u32;

    #[inline]
    pub fn enable(enable: bool) {
        if enable {
            unsafe {
                core::ptr::write(ADDRESS, 0x41f0_0000);
            }
        } else {
            unsafe {
                core::ptr::write(ADDRESS, 0x4040_0000);
            }
        }
    }
}

/// Link's super fast during swimming (different from "normal" superswims)
pub mod super_swim {
    #[inline]
    pub fn enable(enable: bool) {
        if enable {
            unsafe {
                core::ptr::write(super::air::ADDRESS, 0xdff);
            }
        } else {
            unsafe {
                core::ptr::write(super::air::ADDRESS, super::air::MAX);
            }
        }
    }
}

pub mod sea_charts {
    pub const ADDRESS: *mut [u8; 49] = 0x1506b604 as *mut [u8; 49];

    pub enum State {
        /// TODO
        Unknown0 = 0,
        /// TODO
        Unknown1 = 1,
        /// TODO
        Unknown2 = 2,
        /// Map for coordinate is obtained
        Mapped = 3,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Location {
        ForsakenFortress = 1,
        FourEyeReef = 2,
        WesternFairyIsland = 3,
        ThreeEyeReef = 4,
        NeedleRockIsle = 5,
        DiamondSteppeIsland = 6,
        HorseshoeIsland = 7,
        StarIsland = 8,
        MotherChildIsles = 9,
        RockSpireIsle = 10,
        GreatfishIsle = 11,
        IsletOfSteel = 12,
        FiveEyeReef = 13,
        OutsetIsland = 14,
        NorthernFairyIsland = 15,
        SpectacleIsland = 16,
        TingleIsland = 17,
        CyclopsReef = 18,
        StoneWatcherIsland = 19,
        SharkIsland = 20,
        HeadstoneIsland = 21,
        GaleIsle = 22,
        WindfallIsland = 23,
        NorthernTriangleIsland = 24,
        SixEyeReef = 25,
        SouthernTriangleIsle = 26,
        SouthernFairyIsland = 27,
        TwoEyeReef = 28,
        CrescentMoonIsland = 29,
        PawprintIsle = 30,
        EasternFairyIsland = 31,
        TowerOfTheGods = 32,
        PrivateOasis = 33,
        IceRingIsle = 34,
        AngularIsles = 35,
        SevenStarIsles = 36,
        DragonRoostIsland = 37,
        FireMountain = 38,
        EasternTriangleIsland = 39,
        BombIsland = 40,
        ForestHaven = 41,
        BoatingCourse = 42,
        OverlookIsland = 43,
        FlightControlPlatform = 44,
        StarBeltArchipelago = 45,
        ThornedFairyIsland = 46,
        BirdsPeakRock = 47,
        CliffPlateauIsles = 48,
        FiveStarIsles = 49,
    }

    #[allow(unused_imports)]
    pub use Location::*;
    #[allow(unused_imports)]
    pub use State::*;

    #[inline]
    pub fn write(value: [u8; 49]) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> [u8; 49] {
        unsafe { core::ptr::read(ADDRESS) }
    }

    /// Sets a single coordinate
    #[inline]
    pub fn set_single(value: State, location: Location) {
        unsafe {
            core::ptr::write(
                (ADDRESS as usize + location as usize) as *mut u8,
                value as u8,
            );
        }
    }

    /// Gets a single coordinate
    #[inline]
    pub fn get_single(location: Location) -> u8 {
        unsafe { core::ptr::read((ADDRESS as usize + location as usize) as *mut u8) }
    }
}

// Equipped items

pub mod equipped_items {

    use crate::items::Item;
    pub use Button::*;

    pub enum Button {
        X = 0x10976e6b,
        Y = 0x10976e6c,
        R = 0x10976e6d,
    }

    #[inline]
    pub fn write(button: usize, item: u8) {
        unsafe {
            core::ptr::write(button as *mut u8, item);
        }
    }

    #[inline]
    pub fn read(button: usize) -> u8 {
        unsafe { core::ptr::read(button as *mut u8) }
    }

    #[inline]
    pub fn set(button: Button, item: Option<Item>) {
        if let Some(item) = item {
            write(button as usize, item as u8);
        } else {
            write(button as usize, 0);
        }
    }

    #[inline]
    pub fn get(button: Button) -> Option<Item> {
        match read(button as usize) {
            x if x == Item::Bombs as u8 => Some(Item::Bombs),
            x if x == Item::Boomerang as u8 => Some(Item::Boomerang),
            x if x == Item::DekuLeaf as u8 => Some(Item::DekuLeaf),
            x if x == Item::DeluxeBox as u8 => Some(Item::DeluxeBox),
            x if x == Item::GrapplingHook as u8 => Some(Item::GrapplingHook),
            x if x == Item::HeroBow as u8 => Some(Item::HeroBow),
            x if x == Item::Hookshot as u8 => Some(Item::Hookshot),
            x if x == Item::IronBoots as u8 => Some(Item::IronBoots),
            x if x == Item::MagicArmor as u8 => Some(Item::MagicArmor),
            x if x == Item::PictoBox as u8 => Some(Item::PictoBox),
            x if x == Item::SkullHammer as u8 => Some(Item::SkullHammer),
            x if x == Item::Telescope as u8 => Some(Item::Telescope),
            x if x == Item::TingleBottle as u8 => Some(Item::TingleBottle),
            x if x == Item::WindWaker as u8 => Some(Item::WindWaker),
            _ => None,
        }
    }
}

// Location

pub mod position {
    pub mod x {
        pub const ADDRESS: *mut f32 = 0x1096ef48 as *mut f32;

        #[inline]
        pub fn write(value: f32) {
            unsafe {
                core::ptr::write(ADDRESS, value);
            }
        }

        #[inline]
        pub fn read() -> f32 {
            unsafe { core::ptr::read(ADDRESS) }
        }
    }

    pub mod y {
        pub const ADDRESS: *mut f32 = 0x1096ef50 as *mut f32;

        #[inline]
        pub fn write(value: f32) {
            unsafe {
                core::ptr::write(ADDRESS, value);
            }
        }

        #[inline]
        pub fn read() -> f32 {
            unsafe { core::ptr::read(ADDRESS) }
        }
    }

    pub mod z {
        pub const ADDRESS: *mut f32 = 0x1096ef4c as *mut f32;

        #[inline]
        pub fn write(value: f32) {
            unsafe {
                core::ptr::write(ADDRESS, value);
            }
        }

        #[inline]
        pub fn read() -> f32 {
            unsafe { core::ptr::read(ADDRESS) }
        }
    }

    pub mod facing_angle {
        pub const ADDRESS: *mut u16 = 0x1096ef12 as *mut u16;

        #[inline]
        pub fn write(value: u16) {
            unsafe {
                core::ptr::write(ADDRESS, value);
            }
        }

        #[inline]
        pub fn read() -> u16 {
            unsafe { core::ptr::read(ADDRESS) }
        }
    }

    pub mod speed_angle {
        pub const ADDRESS: *mut u16 = 0x1096ef0a as *mut u16;

        #[inline]
        pub fn write(value: u16) {
            unsafe {
                core::ptr::write(ADDRESS, value);
            }
        }

        #[inline]
        pub fn read() -> u16 {
            unsafe { core::ptr::read(ADDRESS) }
        }
    }

    pub mod speed {
        pub const ADDRESS: *mut usize = 0x10989C74 as *mut usize;
        pub const OFFSET: usize = 0x6938;

        #[inline]
        pub fn write(value: f32) {
            unsafe {
                let ptr = core::ptr::read(ADDRESS);
                let ptr = (ptr + OFFSET) as *mut f32;

                if wut::ptr::is_valid(ptr) {
                    core::ptr::write(ptr, value);
                }
            }
        }

        #[inline]
        pub fn read() -> f32 {
            unsafe {
                let ptr = core::ptr::read(ADDRESS);
                let ptr = (ptr + OFFSET) as *mut f32;

                if wut::ptr::is_valid(ptr) {
                    core::ptr::read(ptr)
                } else {
                    0.0
                }
            }
        }
    }
}

pub mod hover {
    pub const ADDRESS: *mut usize = 0x10976ab4 as *mut usize;
    pub const OFFSET: usize = 0x340;

    #[inline]
    pub fn activate() {
        unsafe {
            let ptr = core::ptr::read(ADDRESS);
            let ptr = (ptr + OFFSET) as *mut u32;

            if wut::ptr::is_valid(ptr) {
                core::ptr::write(ptr, 0x4210_0000);
            }
        }
    }
}

pub mod collision {
    pub const ADDRESS: *mut usize = 0x10976de4 as *mut usize;
    pub const OFFSET: usize = 2060 + 40;

    #[allow(unused_imports)]
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
