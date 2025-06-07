#![allow(dead_code)]

macro_rules! flags {
    ($name: ident, $addr:literal, $len:literal) => {
        pub mod $name {
            pub const ADDRESS: *mut [u8; $len] = $addr as *mut [u8; $len];

            #[inline]
            pub fn write(value: [u8; $len]) {
                unsafe {
                    core::ptr::write(ADDRESS, value);
                }
            }

            #[inline]
            pub fn read() -> [u8; $len] {
                unsafe { core::ptr::read(ADDRESS) }
            }
        }
    };
}

flags!(scene, 0x1506bc78, 0x40);

flags!(global, 0x1506bb24, 0x40);

flags!(great_sea, 0x1506B880, 0x24);

flags!(forsaken_fortress, 0x1506B8C8, 0x24);

flags!(dragon_roost_cavern, 0x1506B8EC, 0x28);

flags!(forbidden_woods, 0x1506B914, 0x20);

flags!(tower_of_the_gods, 0x1506B934, 0x24);

flags!(earth_temple, 0x1506B958, 0x28);

flags!(wind_temple, 0x1506B980, 0x20);

flags!(ganons_tower, 0x1506B9A0, 0x24);

flags!(hyrule, 0x1506B9C4, 0x24);

flags!(interior, 0x1506BA0C, 0x24);
