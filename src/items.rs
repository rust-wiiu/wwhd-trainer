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

pub mod bait_bag {
    pub const ADDRESS: *mut u8 = 0x1506b547 as *mut u8;

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
        set(super::Item::BaitBag as u8);
    }

    #[inline]
    pub fn disable() {
        set(0);
    }
}

pub mod bombs {
    pub const ADDRESS: *mut u8 = 0x1506b549 as *mut u8;

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
        set(super::Item::Bombs as u8);
    }

    #[inline]
    pub fn disable() {
        set(0);
    }
}

pub mod boomerang {
    pub const ADDRESS: *mut u8 = 0x1506b541 as *mut u8;

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
        set(super::Item::Boomerang as u8);
    }

    #[inline]
    pub fn disable() {
        set(0);
    }
}

pub mod deku_leaf {
    pub const ADDRESS: *mut u8 = 0x1506b542 as *mut u8;

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
        set(super::Item::DekuLeaf as u8);
    }

    #[inline]
    pub fn disable() {
        set(0);
    }
}

pub mod delivery_bag {
    pub const ADDRESS: *mut u8 = 0x1506b54e as *mut u8;

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
        set(super::Item::DeliveryBag as u8);
    }

    #[inline]
    pub fn disable() {
        set(0);
    }
}

pub mod picto_box {
    pub const ADDRESS: *mut u8 = 0x1506b544 as *mut u8;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Version {
        None = 0xff,
        Normal = super::Item::PictoBox as isize,
        Delux = super::Item::DeluxeBox as isize,
    }

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
    pub fn enable(version: Version) {
        set(version as u8);
    }

    #[inline]
    pub fn disable() {
        set(Version::None as u8);
    }
}

pub mod grappling_hook {
    pub const ADDRESS: *mut u8 = 0x1506b53f as *mut u8;

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
        set(super::Item::GrapplingHook as u8);
    }

    #[inline]
    pub fn disable() {
        set(0);
    }
}

pub mod bow {
    pub const ADDRESS: *mut u8 = 0x1506b548 as *mut u8;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Version {
        None = 0,
        Hero = super::Item::HeroBow as isize,
        Elemental = super::Item::ElementalBow as isize,
        Magical = super::Item::MagicalBow as isize,
    }

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
    pub fn enable(version: Version) {
        set(version as u8);
    }

    #[inline]
    pub fn disable() {
        set(Version::None as u8);
    }
}

pub mod hero_charm {
    pub const ADDRESS: *mut u8 = 0x1506b5b8 as *mut u8;

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
        set(super::Item::HeroCharm as u8);
    }

    #[inline]
    pub fn disable() {
        set(0);
    }
}

pub mod shield {
    pub const ADDRESS: *mut u8 = 0x1506b50f as *mut u8;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Version {
        None = 0xff,
        Hero = super::Item::HeroShield as isize,
        Mirror = super::Item::MirrorShield as isize,
    }

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
    pub fn enable(version: Version) {
        set(version as u8);
    }

    #[inline]
    pub fn disable() {
        set(Version::None as u8);
    }
}

pub mod sword {
    pub const ADDRESS: *mut u8 = 0x1506b50e as *mut u8;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Version {
        None = 0xff,
        Hero = super::Item::HeroSword as isize,
        Master1 = super::Item::MasterSword1 as isize,
        Master2 = super::Item::MasterSword2 as isize,
        Master3 = super::Item::MasterSword3 as isize,
    }

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
    pub fn enable(version: Version) {
        set(version as u8);
    }

    #[inline]
    pub fn disable() {
        set(Version::None as u8);
    }
}

pub mod hookshot {
    pub const ADDRESS: *mut u8 = 0x1506b54f as *mut u8;

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
        set(super::Item::Hookshot as u8);
    }

    #[inline]
    pub fn disable() {
        set(0);
    }
}

pub mod iron_boots {
    pub const ADDRESS: *mut u8 = 0x1506b545 as *mut u8;

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
        set(super::Item::IronBoots as u8);
    }

    #[inline]
    pub fn disable() {
        set(0);
    }
}

pub mod magic_armor {
    pub const ADDRESS: *mut u8 = 0x1506b546 as *mut u8;

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
        set(super::Item::MagicArmor as u8);
    }

    #[inline]
    pub fn disable() {
        set(0);
    }
}

pub mod power_bracelets {
    pub const ADDRESS_1: *mut u8 = 0x1506b510 as *mut u8;
    pub const ADDRESS_2: *mut u8 = 0x1506b5b6 as *mut u8;

    #[inline]
    pub fn set(value1: u8, value2: u8) {
        unsafe {
            core::ptr::write(ADDRESS_1, value1);
            core::ptr::write(ADDRESS_2, value2);
        }
    }

    #[inline]
    pub fn get() -> (u8, u8) {
        unsafe { (core::ptr::read(ADDRESS_1), core::ptr::read(ADDRESS_2)) }
    }

    #[inline]
    pub fn enable() {
        set(super::Item::PowerBracelets as u8, 0xff);
    }

    #[inline]
    pub fn disable() {
        set(0xff, 0);
    }
}

pub mod skull_hammer {
    pub const ADDRESS: *mut u8 = 0x1506b550 as *mut u8;

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
        set(super::Item::SkullHammer as u8);
    }

    #[inline]
    pub fn disable() {
        set(0);
    }
}

pub mod spoils_bag {
    pub const ADDRESS: *mut u8 = 0x1506b540 as *mut u8;

    pub use Spoil::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        set(super::Item::SpoilsBag as u8);
    }

    #[inline]
    pub fn disable() {
        set(0);
    }

    #[inline]
    pub fn spoil(spoil: Spoil, count: u8) {
        unsafe {
            core::ptr::write(spoil as usize as *mut u8, count);
        }
    }
}

pub mod telescope {
    pub const ADDRESS: *mut u8 = 0x1506b53c as *mut u8;

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
        set(super::Item::Telescope as u8);
    }

    #[inline]
    pub fn disable() {
        set(0);
    }
}

pub mod tingle_bottle {
    pub const ADDRESS: *mut u8 = 0x1506b543 as *mut u8;

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
        set(super::Item::TingleBottle as u8);
    }

    #[inline]
    pub fn disable() {
        set(0);
    }
}

pub mod wind_waker {
    pub const ADDRESS: *mut u8 = 0x1506b53e as *mut u8;

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
        set(super::Item::WindWaker as u8);
    }

    #[inline]
    pub fn disable() {
        set(0);
    }
}

pub mod sail {
    pub const ADDRESS: *mut u8 = 0x1506b53d as *mut u8;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Version {
        None = 0xff,
        Normal = 0x78,
        Swift = 0x77,
    }

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
    pub fn enable(version: Version) {
        set(version as u8);
    }

    #[inline]
    pub fn disable() {
        set(Version::None as u8);
    }
}

pub mod bottles {
    pub const ADDRESS: *mut [u8; 4] = 0x1506b54a as *mut [u8; 4];

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Slot {
        Bottle1 = 0,
        Bottle2 = 1,
        Bottle3 = 2,
        Bottle4 = 3,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Content {
        None = 0xff,
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

    #[inline]
    pub fn set(value: [u8; 4]) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn get() -> [u8; 4] {
        unsafe { core::ptr::read(ADDRESS) }
    }

    #[inline]
    pub fn enable(slot: Slot, content: Content) {
        let mut bottles = get();
        bottles[slot as usize] = content as u8;
        set(bottles);
    }

    #[inline]
    pub fn disable(slot: Slot) {
        let mut bottles = get();
        bottles[slot as usize] = Content::None as u8;
        set(bottles);
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
    pub fn enable(song: Song) {
        set(get() | song as u8);
    }

    #[inline]
    pub fn disable(song: Song) {
        set(get() & !(song as u8));
    }
}

pub mod triforce {
    pub const ADDRESS: *mut u8 = 0x1506b5be as *mut u8;
    pub const MAX: u8 = 8;

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
    pub fn enable(pieces: u8) {
        set(match pieces {
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
    pub fn disable() {
        set(0);
    }
}

pub mod pearls {
    pub const ADDRESS: *mut u8 = 0x1506b5bf as *mut u8;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Pearl {
        Nayru = 0b0000_0001,
        Din = 0b0000_0010,
        Farore = 0b0000_0100,
    }

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
    pub fn enable(pearl: Pearl) {
        set(get() | pearl as u8);
    }

    #[inline]
    pub fn disable(pearl: Pearl) {
        set(get() & !(pearl as u8));
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
        None = 0xff,
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

    #[inline]
    pub fn set(value: [u8; 8]) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn get() -> [u8; 8] {
        unsafe { core::ptr::read(ADDRESS) }
    }

    #[inline]
    pub fn enable(slot: Slot, content: Content) {
        let mut bag = get();
        bag[slot as usize] = content as u8;
        set(bag);
    }

    #[inline]
    pub fn disable(slot: Slot) {
        let mut bag = get();
        bag[slot as usize] = Content::None as u8;
        set(bag);
    }
}

pub mod dungeon_keys {
    pub const ADDRESS: *mut u8 = 0x1506bc98 as *mut u8;
    pub const MAX: u8 = 10;

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
    pub fn enable(count: u8) {
        set(if count <= 10 { count } else { 10 });
    }

    #[inline]
    pub fn disable() {
        set(0);
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
    pub fn enable(item: Item) {
        set(get() | item as u8);
    }

    #[inline]
    pub fn disable(item: Item) {
        set(get() & !(item as u8));
    }
}
