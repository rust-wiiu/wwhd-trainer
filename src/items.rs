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

pub mod bottle {
    pub const NONE: u8 = 0xff;
    pub const EMPTY: u8 = 0x50;
    pub const RED_ELIXIR: u8 = 0x51;
    pub const GREEN_ELIXIR: u8 = 0x52;
    pub const BLUE_ELIXIR: u8 = 0x53;
    pub const SOUP_HALF: u8 = 0x54;
    pub const SOUP: u8 = 0x55;
    pub const WATER: u8 = 0x56;
    pub const FAIRY: u8 = 0x57;
    pub const POLLEN: u8 = 0x58;
    pub const MAGIC_WATER: u8 = 0x59;
}

pub const NORMAL_SAIL: Item = Item {
    address: 0x1506b53d as *mut u8,
    value: 0x78,
};

pub const SWIFT_SAIL: Item = Item {
    address: 0x1506b53d as *mut u8,
    value: 0x77,
};

pub const WINDS_REQUIEM: Item = Item {
    address: 0x1506b5bd as *mut u8,
    value: 0b0000_0001,
};

pub const BALLAD_OF_GALES: Item = Item {
    address: 0x1506b5bd as *mut u8,
    value: 0b0000_0010,
};

pub const COMMAND_MELODY: Item = Item {
    address: 0x1506b5bd as *mut u8,
    value: 0b0000_0100,
};

pub const EARTH_GODS_LYRICS: Item = Item {
    address: 0x1506b5bd as *mut u8,
    value: 0b0000_1000,
};

pub const WIND_GODS_ARIA: Item = Item {
    address: 0x1506b5bd as *mut u8,
    value: 0b0001_0000,
};

pub const SONG_OF_PASSING: Item = Item {
    address: 0x1506b5bd as *mut u8,
    value: 0b0010_0000,
};

pub const TRIFORCE: Item = Item {
    address: 0x1506b5be as *mut u8,
    value: 0b1111_1111,
};

pub const NAYRUS_PEARL: Item = Item {
    address: 0x1506b5bf as *mut u8,
    value: 0b0000_0001,
};

pub const DINS_PEARL: Item = Item {
    address: 0x1506b5bf as *mut u8,
    value: 0b0000_0010,
};

pub const FARORES_PEARL: Item = Item {
    address: 0x1506b5bf as *mut u8,
    value: 0b0000_0100,
};

pub const MAILBAG_1: Item = Item {
    address: 0x1506b586 as *mut u8,
    value: 0xff, // empty
};

pub const MAILBAG_2: Item = Item {
    address: 0x1506b587 as *mut u8,
    value: 0xff, // empty
};

pub const MAILBAG_3: Item = Item {
    address: 0x1506b588 as *mut u8,
    value: 0xff, // empty
};

pub const MAILBAG_4: Item = Item {
    address: 0x1506b589 as *mut u8,
    value: 0xff, // empty
};

pub const MAILBAG_5: Item = Item {
    address: 0x1506b58a as *mut u8,
    value: 0xff, // empty
};

pub const MAILBAG_6: Item = Item {
    address: 0x1506b58b as *mut u8,
    value: 0xff, // empty
};

pub const MAILBAG_7: Item = Item {
    address: 0x1506b58c as *mut u8,
    value: 0xff, // empty
};

pub const MAILBAG_8: Item = Item {
    address: 0x1506b58d as *mut u8,
    value: 0xff, // empty
};

pub mod mailbag {
    pub const NONE: u8 = 0xff;
    pub const TOWN_FLOWER: u8 = 0x8c;
    pub const SEA_FLOWER: u8 = 0x8d;
    pub const EXOTIC_FLOWER: u8 = 0x8e;
    pub const HEROS_FLAG: u8 = 0x8f;
    pub const BIG_CATCH_FLAG: u8 = 0x90;
    pub const BIG_SALE_FLAG: u8 = 0x91;
    pub const PINWHEEL: u8 = 0x92;
    pub const SICKLE_MOON_FLAG: u8 = 0x93;
    pub const SKULL_TOWER_IDOL: u8 = 0x94;
    pub const FOUNTAIN_IDOL: u8 = 0x95;
    pub const POSTMAN_STATUE: u8 = 0x96;
    pub const SHOP_GURU_STATUE: u8 = 0x97;
    pub const FATHERS_LETTER: u8 = 0x98;
    pub const NOTE_TO_MOM: u8 = 0x99;
    pub const MAGGIES_LETTER: u8 = 0x9a;
    pub const MOBLINS_LETTER: u8 = 0x9b;
    pub const CABANA_DEED: u8 = 0x9c;
    pub const COMPLIMENTARY_ID: u8 = 0x9d;
    pub const FILL_UP_COUPON: u8 = 0x9e;
}
