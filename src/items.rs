#![allow(dead_code)]

pub struct Item {
    pub address: *mut u8,
    pub value: u8,
}

pub const BAIT_BAG: Item = Item {
    address: 0x1506b547 as *mut u8,
    value: 0x2c,
};

pub const BOMBS: Item = Item {
    address: 0x1506b549 as *mut u8,
    value: 0x31,
};

pub const BOOMERANG: Item = Item {
    address: 0x1506b541 as *mut u8,
    value: 0x2d,
};

pub const DEKU_LEAF: Item = Item {
    address: 0x1506b542 as *mut u8,
    value: 0x34,
};

pub const DELIVERY_BAG: Item = Item {
    address: 0x1506b54e as *mut u8,
    value: 0x30,
};

pub const PICTO_BOX: Item = Item {
    address: 0x1506b544 as *mut u8,
    value: 0x23,
};

pub const DELUXE_BOX: Item = Item {
    address: 0x1506b544 as *mut u8,
    value: 0x26,
};

pub const GRAPPLING_HOOK: Item = Item {
    address: 0x1506b53f as *mut u8,
    value: 0x25,
};

pub const HERO_BOW: Item = Item {
    address: 0x1506b548 as *mut u8,
    value: 0x27,
};

pub const ELEMENTAL_BOW: Item = Item {
    address: 0x1506b548 as *mut u8,
    value: 0x35,
};

pub const MAGICAL_BOW: Item = Item {
    address: 0x1506b548 as *mut u8,
    value: 0x36,
};

pub const HERO_CHARM: Item = Item {
    address: 0x1506b5b8 as *mut u8,
    value: 0x01,
};

pub const HERO_SHIELD: Item = Item {
    address: 0x1506b50f as *mut u8,
    value: 0x3b,
};

pub const MIRROR_SHIELD: Item = Item {
    address: 0x1506b50f as *mut u8,
    value: 0x3c,
};

pub const HERO_SWORD: Item = Item {
    address: 0x1506b50e as *mut u8,
    value: 0x38,
};

pub const MASTER_SWORD_1: Item = Item {
    address: 0x1506b50e as *mut u8,
    value: 0x39,
};

pub const MASTER_SWORD_2: Item = Item {
    address: 0x1506b50e as *mut u8,
    value: 0x3a,
};

pub const MASTER_SWORD_3: Item = Item {
    address: 0x1506b50e as *mut u8,
    value: 0x3e,
};

pub const HOOKSHOT: Item = Item {
    address: 0x1506b54f as *mut u8,
    value: 0x2f,
};

pub const IRON_BOOTS: Item = Item {
    address: 0x1506b545 as *mut u8,
    value: 0x29,
};

pub const MAGIC_ARMOR: Item = Item {
    address: 0x1506b546 as *mut u8,
    value: 0x2a,
};

pub const POWER_BRACELETS_1: Item = Item {
    address: 0x1506b510 as *mut u8,
    value: 0x28,
};

pub const POWER_BRACELETS_2: Item = Item {
    address: 0x1506b5b6 as *mut u8,
    value: 0xff,
};

pub const SKULL_HAMMER: Item = Item {
    address: 0x1506b550 as *mut u8,
    value: 0x33,
};

pub const SPOILS_BAG: Item = Item {
    address: 0x1506b540 as *mut u8,
    value: 0x24,
};

pub const TELESCOPE: Item = Item {
    address: 0x1506b53c as *mut u8,
    value: 0x20,
};

pub const TINGLE_BOTTLE: Item = Item {
    address: 0x1506b543 as *mut u8,
    value: 0x21,
};

pub const WIND_WAKER: Item = Item {
    address: 0x1506b53e as *mut u8,
    value: 0x22,
};

pub const BOTTLE_1: Item = Item {
    address: 0x1506b54a as *mut u8,
    value: 0x50, // empty bottle
};

pub const BOTTLE_2: Item = Item {
    address: 0x1506b54b as *mut u8,
    value: 0x50, // empty bottle
};

pub const BOTTLE_3: Item = Item {
    address: 0x1506b54c as *mut u8,
    value: 0x50, // empty bottle
};

pub const BOTTLE_4: Item = Item {
    address: 0x1506b54d as *mut u8,
    value: 0x50, // empty bottle
};
