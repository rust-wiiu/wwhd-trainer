#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    HeroCharm = 0x01,
    Telescope = 0x20,
    TingleBottle = 0x21,
    WindWaker = 0x22,
    PictoBox = 0x23,
    SpoilsBag = 0x24,
    GrapplingHook = 0x25,
    DeluxeBox = 0x26,
    HeroBow = 0x27,
    PowerBracelets = 0x28,
    IronBoots = 0x29,
    MagicArmor = 0x2a,
    BaitBag = 0x2c,
    Boomerang = 0x2d,
    Hookshot = 0x2f,
    DeliveryBag = 0x30,
    Bombs = 0x31,
    SkullHammer = 0x33,
    DekuLeaf = 0x34,
    ElementalBow = 0x35,
    MagicalBow = 0x36,
    HeroSword = 0x38,
    MasterSword1 = 0x39,
    MasterSword2 = 0x3a,
    HeroShield = 0x3b,
    MirrorShield = 0x3c,
    MasterSword3 = 0x3e,
}

macro_rules! item_module {
    ($module_name:ident, $address:expr, $item:path) => {
        pub mod $module_name {
            pub const ADDRESS: *mut u8 = $address as *mut u8;

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

            #[inline]
            pub fn enable(enable: bool) {
                if enable {
                    write($item as u8);
                } else {
                    write(0);
                }
            }
            #[inline]
            pub fn enabled() -> bool {
                read() == $item as u8
            }
        }
    };
}

macro_rules! item_module_with_versions {
    ($module_name:ident, $address:expr, [$(($variant:ident, $value:expr)),*]) => {
        pub mod $module_name {
            pub const ADDRESS: *mut u8 = $address as *mut u8;

            #[derive(Debug, Clone, Copy, PartialEq)]
            pub enum Version {
                $(
                    $variant = $value as isize,
                )*
            }

            #[allow(unused_imports)]
            pub use Version::*;

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

            #[inline]
            pub fn set(version: Option<Version>) {
                if let Some(version) = version {
                    write(version as u8);
                } else {
                    write(0xff);
                }
            }

            #[inline]
            pub fn get() -> Option<Version> {
                match read() {
                    $(
                        x if x == Version::$variant as u8 => Some(Version::$variant),
                    )*
                    _ => None
                }
            }

            #[inline]
            pub fn enabled() -> bool {
                get().is_some()
            }
        }
    };
}

item_module!(bait_bag, 0x1506b547, super::Item::BaitBag);

item_module!(bombs, 0x1506b549, super::Item::Bombs);

item_module!(boomerang, 0x1506b541, super::Item::Boomerang);

item_module!(deku_leaf, 0x1506b542, super::Item::DekuLeaf);

item_module!(delivery_bag, 0x1506b54e, super::Item::DeliveryBag);

item_module_with_versions!(
    picto_box,
    0x1506b544,
    [
        (Normal, super::Item::PictoBox),
        (Deluxe, super::Item::DeluxeBox)
    ]
);

item_module!(grappling_hook, 0x1506b53f, super::Item::GrapplingHook);

item_module_with_versions!(
    bow,
    0x1506b548,
    [
        (Hero, super::Item::HeroBow),
        (Elemental, super::Item::ElementalBow),
        (Magical, super::Item::MagicalBow)
    ]
);

item_module!(hero_charm, 0x1506b5b8, super::Item::HeroCharm);

item_module_with_versions!(
    shield,
    0x1506b50f,
    [
        (Hero, super::Item::HeroShield),
        (Mirror, super::Item::MirrorShield)
    ]
);

item_module_with_versions!(
    sword,
    0x1506b50e,
    [
        (Hero, super::Item::HeroSword),
        (Master1, super::Item::MasterSword1),
        (Master2, super::Item::MasterSword2),
        (Master3, super::Item::MasterSword3)
    ]
);

item_module!(hookshot, 0x1506b54f, super::Item::Hookshot);
item_module!(iron_boots, 0x1506b545, super::Item::IronBoots);
item_module!(magic_armor, 0x1506b546, super::Item::MagicArmor);

pub mod power_bracelets {
    pub const ADDRESS: (*mut u8, *mut u8) = (0x1506b510 as *mut u8, 0x1506b5b6 as *mut u8);

    #[inline]
    pub fn write(value1: u8, value2: u8) {
        unsafe {
            core::ptr::write(ADDRESS.0, value1);
            core::ptr::write(ADDRESS.1, value2);
        }
    }

    #[inline]
    pub fn read() -> (u8, u8) {
        unsafe { (core::ptr::read(ADDRESS.0), core::ptr::read(ADDRESS.1)) }
    }

    #[inline]
    pub fn enable(enable: bool) {
        if enable {
            write(super::Item::PowerBracelets as u8, 0xff);
        } else {
            write(0xff, 0);
        }
    }

    #[inline]
    pub fn enabled() -> bool {
        let (value1, value2) = read();
        value1 == super::Item::PowerBracelets as u8 && value2 == 0xff
    }
}

item_module!(skull_hammer, 0x1506b550, super::Item::SkullHammer);

pub mod spoils_bag {
    pub const ADDRESS: *mut u8 = 0x1506b540 as *mut u8;

    pub use Spoil::*;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Spoil {
        SkullNecklace = 0x1506b59c,
        BokoSeed = 0x1506b59d,
        GoldenFeather = 0x1506b59e,
        KnightsCrest = 0x1506b59f,
        RedJelly = 0x1506b5a0,
        GreenJelly = 0x1506b5a1,
        BlueJelly = 0x1506b5a2,
        JoyPendant = 0x1506b5a3,
    }

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

    #[inline]
    pub fn enable(enable: bool) {
        if enable {
            write(super::Item::SpoilsBag as u8);
        } else {
            write(0);
        }
    }

    #[inline]
    pub fn enabled() -> bool {
        read() == super::Item::SpoilsBag as u8
    }

    #[inline]
    pub fn spoil(spoil: Spoil, count: u8) {
        unsafe {
            core::ptr::write(spoil as usize as *mut u8, count);
        }
    }
}

item_module!(telescope, 0x1506b53c, super::Item::Telescope);
item_module!(tingle_bottle, 0x1506b543, super::Item::TingleBottle);
item_module!(wind_waker, 0x1506b53e, super::Item::WindWaker);

item_module_with_versions!(sail, 0x1506b53d, [(Normal, 0x78), (Swift, 0x77)]);

pub mod bottles {
    pub const ADDRESS: *mut [u8; 4] = 0x1506b54a as *mut [u8; 4];

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Slot {
        Bottle1 = 0,
        Bottle2 = 1,
        Bottle3 = 2,
        Bottle4 = 3,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Content {
        Empty = 0x50,
        RedElixir = 0x51,
        GreenElixir = 0x52,
        BlueElixir = 0x53,
        SoupHalf = 0x54,
        Soup = 0x55,
        Water = 0x56,
        Fairy = 0x57,
        Pollen = 0x58,
        MagicWater = 0x59,
    }

    #[allow(unused_imports)]
    pub use Content::*;
    #[allow(unused_imports)]
    pub use Slot::*;

    #[inline]
    pub fn write(value: [u8; 4]) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> [u8; 4] {
        unsafe { core::ptr::read(ADDRESS) }
    }

    #[inline]
    pub fn set(slot: Slot, content: Option<Content>) {
        let mut bottles = read();
        if let Some(content) = content {
            bottles[slot as usize] = content as u8;
        } else {
            bottles[slot as usize] = 0xff;
        }
        write(bottles);
    }
}

pub mod songs {
    pub const ADDRESS: *mut u8 = 0x1506b5bd as *mut u8;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Song {
        WindRequiem = 0b0000_0001,
        BalladOfGales = 0b0000_0010,
        CommandMelody = 0b0000_0100,
        EarthGodsLyrics = 0b0000_1000,
        WindGodsAria = 0b0001_0000,
        SongOfPassing = 0b0010_0000,
    }

    #[allow(unused_imports)]
    pub use Song::*;

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

    #[inline]
    pub fn enable(enable: bool, song: Song) {
        if enable {
            write(read() | song as u8);
        } else {
            write(read() & !(song as u8));
        }
    }
}

pub mod triforce {
    pub const ADDRESS: *mut u8 = 0x1506b5be as *mut u8;
    pub const MAX: u8 = 8;

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

    #[inline]
    pub fn set(pieces: u8) {
        write(match pieces {
            0 => 0b0000_0000,
            1 => 0b0000_0001,
            2 => 0b0000_0011,
            3 => 0b0000_0111,
            4 => 0b0000_1111,
            5 => 0b0001_1111,
            6 => 0b0011_1111,
            7 => 0b0111_1111,
            _ => 0b1111_1111,
        });
    }

    #[inline]
    pub fn get() -> u8 {
        read().count_ones() as u8
    }
}

pub mod pearls {
    pub const ADDRESS: *mut u8 = 0x1506b5bf as *mut u8;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Pearl {
        Nayru = 0b0000_0001,
        Din = 0b0000_0010,
        Farore = 0b0000_0100,
    }

    #[allow(unused_imports)]
    pub use Pearl::*;

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

    #[inline]
    pub fn enable(enable: bool, pearl: Pearl) {
        if enable {
            write(read() | pearl as u8);
        } else {
            write(read() & !(pearl as u8));
        }
    }
}

pub mod mailbag {
    pub const ADDRESS: *mut [u8; 8] = 0x1506b586 as *mut [u8; 8];

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Slot {
        Slot1 = 0,
        Slot2 = 1,
        Slot3 = 2,
        Slot4 = 3,
        Slot5 = 4,
        Slot6 = 5,
        Slot7 = 6,
        Slot8 = 7,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Content {
        TownFlower = 0x8c,
        SeaFlower = 0x8d,
        ExoticFlower = 0x8e,
        HerosFlag = 0x8f,
        BigCatchFlag = 0x90,
        BigSaleFlag = 0x91,
        Pinwheel = 0x92,
        SickleMoonFlag = 0x93,
        SkullTowerIdol = 0x94,
        FountainIdol = 0x95,
        PostmanStatue = 0x96,
        ShopGuruStatue = 0x97,
        FathersLetter = 0x98,
        NoteToMom = 0x99,
        MaggiesLetter = 0x9a,
        MoblinsLetter = 0x9b,
        CabanaDeed = 0x9c,
        ComplimentaryId = 0x9d,
        FillUpCoupon = 0x9e,
    }

    #[allow(unused_imports)]
    pub use Content::*;
    #[allow(unused_imports)]
    pub use Slot::*;

    #[inline]
    pub fn write(value: [u8; 8]) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> [u8; 8] {
        unsafe { core::ptr::read(ADDRESS) }
    }

    #[inline]
    pub fn set(slot: Slot, content: Option<Content>) {
        let mut bag = read();
        if let Some(content) = content {
            bag[slot as usize] = content as u8;
        } else {
            bag[slot as usize] = 0xff;
        }
        write(bag);
    }
}

pub mod dungeon_keys {
    pub const ADDRESS: *mut u8 = 0x1506bc98 as *mut u8;
    pub const MAX: u8 = 10;

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

    #[inline]
    pub fn set(count: u8) {
        write(if count <= 10 { count } else { 10 });
    }
}

pub mod dungeon_items {
    pub const ADDRESS: *mut u8 = 0x1506bc99 as *mut u8;

    pub use Item::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Item {
        Map = 0b0000_0001,
        Compass = 0b0000_0010,
        BossKey = 0b0000_0100,
    }

    #[allow(unused_imports)]
    pub use Item::*;

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

    #[inline]
    pub fn enable(enable: bool, item: Item) {
        if enable {
            write(read() | item as u8);
        } else {
            write(read() & !(item as u8));
        }
    }
}
