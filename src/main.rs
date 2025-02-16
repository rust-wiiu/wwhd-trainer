#![no_std]
#![no_main]

use wut::font::icons;
use wut::prelude::*;
use wut::*;

use overlay;
use wups::*;

mod items;
mod player;
mod stages;

WUPS_PLUGIN_NAME!("WWHD Trainer");

static HANDLE: sync::RwLock<Option<thread::JoinHandle>> = sync::RwLock::new(None);
static mut INPUT: gamepad::GamepadState = gamepad::GamepadState::empty();

#[function_hook(module = VPAD, function = VPADRead)]
fn my_VPADRead(
    chan: wut::bindings::VPADChan::Type,
    buffers: *mut wut::bindings::VPADStatus,
    count: u32,
    error: *mut wut::bindings::VPADReadError::Type,
) -> i32 {
    let status = unsafe { hooked(chan, buffers, count, error) };

    use gamepad::Button as B;
    unsafe {
        INPUT = gamepad::GamepadState::from(*buffers);

        if INPUT.hold.contains(B::L | B::R) {
            (*buffers).hold = 0;
            (*buffers).trigger = 0;
        }
    }

    status
}

#[on_application_start(Udp)]
fn start() {
    let mut thread = HANDLE.write();
    if thread.is_none() {
        *thread = Some(
            thread::Builder::default()
                .name("Overlay")
                .attribute(thread::thread::ThreadAttribute::Cpu2)
                .priority(30)
                .spawn(overlay_thread)
                .unwrap(),
        );
    }
}

#[derive(Debug, Default)]
struct Cheats {
    health: bool,
    magic: bool,
    rupees: bool,
    arrows: bool,
    bombs: bool,
    air: bool,
    super_swim: bool,
    super_crouch: bool,
}

fn overlay_thread() {
    let _ = logger::init(logger::Udp);

    use alloc::sync::Arc;
    use overlay::*;
    use wut::sync::Mutex;

    let cheats = Arc::new(Mutex::new(Cheats::default()));

    let bottle_options = vec![
        ("None", items::bottle::NONE),
        ("Empty", items::bottle::EMPTY),
        ("Red Elixir", items::bottle::RED_ELIXIR),
        ("Green Elixir", items::bottle::GREEN_ELIXIR),
        ("Blue Elixir", items::bottle::BLUE_ELIXIR),
        ("Soup (Half)", items::bottle::SOUP_HALF),
        ("Soup", items::bottle::SOUP),
        ("Water", items::bottle::WATER),
        ("Fairy", items::bottle::FAIRY),
        ("Pollen", items::bottle::POLLEN),
        ("Magic Water", items::bottle::MAGIC_WATER),
    ];

    let mailbag_options = vec![
        ("None", items::mailbag::NONE),
        ("Town Flower", items::mailbag::TOWN_FLOWER),
        ("Sea Flower", items::mailbag::SEA_FLOWER),
        ("Exotic Flower", items::mailbag::EXOTIC_FLOWER),
        ("Hero's Flag", items::mailbag::HEROS_FLAG),
        ("Big Catch Flag", items::mailbag::BIG_CATCH_FLAG),
        ("Big Sale Flag", items::mailbag::BIG_SALE_FLAG),
        ("Pinwheel", items::mailbag::PINWHEEL),
        ("Sickle Moon Flag", items::mailbag::SICKLE_MOON_FLAG),
        ("Skull Tower Idol", items::mailbag::SKULL_TOWER_IDOL),
        ("Fountain Idol", items::mailbag::FOUNTAIN_IDOL),
        ("Postman Statue", items::mailbag::POSTMAN_STATUE),
        ("Shop Guru Statue", items::mailbag::SHOP_GURU_STATUE),
        ("Father's Letter", items::mailbag::FATHERS_LETTER),
        ("Note to Mom", items::mailbag::NOTE_TO_MOM),
        ("Maggie's Letter", items::mailbag::MAGGIES_LETTER),
        ("Moblin's Letter", items::mailbag::MOBLINS_LETTER),
        ("Cabana Deed", items::mailbag::CABANA_DEED),
        ("Complimentary ID", items::mailbag::COMPLIMENTARY_ID),
        ("Fill-Up Coupon", items::mailbag::FILL_UP_COUPON),
    ];

    let mut overlay = OverlayNotification::new(Menu::new(
        "Root",
        vec![
            Button::new("Search", || unsafe {
                println!("Start search");
                // let start = 0x1098_9c00;
                // let to = start + 0x0000_0001;

                // // 0x1098_9c74

                // for (_i, x) in (start..=to).step_by(1).enumerate() {
                //     let ptr = x as *const f32;
                //     let value = core::ptr::read(ptr);
                //     if value > 16.9 && value < 17.1 {
                //         println!("ptr: {:#08x} - value: {}", ptr as usize, value);
                //     }
                // }

                println!("End search");
            }),
            /*
            Button::new("Speed", || unsafe {
                println!("--- speed ---");

                // let ptr = 0x10989c74 as *mut [u8; 4];
                // let value = core::ptr::read_volatile(ptr);
                // println!("{:?}", value);

                let ptr = 0x1096ef10 as *mut [u8; 4];
                let value = core::ptr::read_volatile(ptr);
                println!(
                    "{:?}, {}",
                    value,
                    core::mem::transmute::<[u8; 4], f32>(value)
                );

                let ptr = 0x1096ef48 as *mut [u8; 4];
                let value = core::ptr::read_volatile(ptr);
                println!(
                    "{:?}, {}",
                    value,
                    core::mem::transmute::<[u8; 4], f32>(value)
                );

                let ptr = 0x1096ef4c as *mut [u8; 4];
                let value = core::ptr::read_volatile(ptr);
                println!(
                    "{:?}, {}",
                    value,
                    core::mem::transmute::<[u8; 4], f32>(value)
                );

                let ptr = 0x1096ef50 as *mut [u8; 4];
                let value = core::ptr::read_volatile(ptr);
                println!(
                    "{:?}, {}",
                    value,
                    core::mem::transmute::<[u8; 4], f32>(value)
                );

                // let ptr = 0x48723ec4 as *mut [u8; 4];
                // let value = core::ptr::read_volatile(ptr);
                // println!("{:?}", value);

                // let ptr = 0x48723ec8 as *mut [u8; 4];
                // let value = core::ptr::read_volatile(ptr);
                // println!("{:?}", value);

                // let ptr = 0x48723ecc as *mut [u8; 4];
                // let value = core::ptr::read_volatile(ptr);
                // println!("{:?}", value);

                println!("--- speed ---");
            }),
            */
            Button::new("Test", || unsafe {
                // let ptr = 0x1096ef48 as *mut f32; // X
                // let value = core::ptr::read(ptr);
                // println!("ptr: {:#08x} - value: {}", ptr as usize, value);

                // let ptr = 0x1096ef4c as *mut f32; // Y (height)
                // let value = core::ptr::read(ptr);
                // println!("ptr: {:#08x} - value: {}", ptr as usize, value);

                // let ptr = 0x1096ef50 as *mut f32; // Z
                // let value = core::ptr::read(ptr);
                // println!("ptr: {:#08x} - value: {}", ptr as usize, value);

                // let ptr = 0x1096ef10 as *mut f32; // Angle?
                // let value = core::ptr::read(ptr);
                // println!("ptr: {:#08x} - value: {}", ptr as usize, value);

                // let ptr = 0x10989c70 as *mut [u8; 16]; // this if for sure not speed
                // let value = core::ptr::read(ptr);
                // println!("ptr: {:#08x} - value: {:?}", ptr as usize, value);

                // println!("{}", core::ptr::read(items::SWIFT_SAIL.address));
                core::ptr::write(items::MAILBAG_1.address, 0x8c);
            }),
            Menu::new(
                "Cheats",
                vec![
                    Toggle::new("Infinite Health", false, {
                        let health = Arc::clone(&cheats);
                        move |v| {
                            health.lock().unwrap().health = v;
                        }
                    }),
                    Toggle::new("Infinite Magic", false, {
                        let magic = Arc::clone(&cheats);
                        move |v| {
                            magic.lock().unwrap().magic = v;
                        }
                    }),
                    Toggle::new("Infinite Rupees", false, {
                        let rupees = Arc::clone(&cheats);
                        move |v| {
                            rupees.lock().unwrap().rupees = v;
                        }
                    }),
                    Toggle::new("Infinite Arrows", false, {
                        let arrows = Arc::clone(&cheats);
                        move |v| {
                            arrows.lock().unwrap().arrows = v;
                        }
                    }),
                    Toggle::new("Infinite Bombs", false, {
                        let bombs = Arc::clone(&cheats);
                        move |v| {
                            bombs.lock().unwrap().bombs = v;
                        }
                    }),
                    Toggle::new("Infinite Air", false, {
                        let air = Arc::clone(&cheats);
                        move |v| {
                            air.lock().unwrap().air = v;
                        }
                    }),
                    Toggle::new("Super Swim", false, {
                        let super_swim = Arc::clone(&cheats);
                        move |v| {
                            super_swim.lock().unwrap().super_swim = v;
                        }
                    }),
                    Toggle::new("Super Crouch", false, {
                        let super_crouch = Arc::clone(&cheats);
                        move |v| {
                            // this code needs manual disabling
                            if !v {
                                unsafe {
                                    core::ptr::write(player::SUPER_CROUCH, 0x4040_0000);
                                }
                            }
                            super_crouch.lock().unwrap().super_crouch = v;
                        }
                    }),
                    Button::new("Complete Map", || unsafe {
                        // this is a one-way for now / just don't save ^^
                        core::ptr::write(player::OVERWORLD_MAP, [3; 49]);
                    }),
                ],
            ),
            Menu::new(
                "Link Tweaks",
                vec![
                    Number::new("Health", 1, 1, 0, 80, |v| unsafe {
                        core::ptr::write(player::HEALTH, *v);
                    }),
                    Number::new("Containers", 1, 1, 1, 20, |v| unsafe {
                        core::ptr::write(player::CONTAINERS, *v);
                    }),
                    Number::new("Magic", 0, 1, 0, 32, |v| unsafe {
                        core::ptr::write(player::MAGIC, *v);
                    }),
                    Select::new(
                        "Max Magic",
                        vec![("No Magic", 0), ("Normal Magic", 16), ("Double Magic", 32)],
                        |_, v| unsafe {
                            core::ptr::write(player::MAX_MAGIC, v.value);
                        },
                    ),
                    Number::new("Rupees", 50, 10, 0, 5000, |v| unsafe {
                        core::ptr::write(player::RUPEES, *v);
                    }),
                    Number::new("Arrows", 30, 10, 0, 99, |v| unsafe {
                        core::ptr::write(player::ARROWS, *v);
                    }),
                    Number::new("Max Arrows", 30, 10, 0, 99, |v| unsafe {
                        core::ptr::write(player::MAX_ARROWS, *v);
                    }),
                    Number::new("Bombs", 30, 10, 0, 99, |v| unsafe {
                        core::ptr::write(player::BOMBS, *v);
                    }),
                    Number::new("Max Bombs", 30, 10, 0, 99, |v| unsafe {
                        core::ptr::write(player::MAX_BOMBS, *v);
                    }),
                ],
            ),
            Menu::new(
                "Items",
                vec![
                    Toggle::new("Bait Bag", false, |v| unsafe {
                        let v = if v { items::BAIT_BAG.value } else { 0 };
                        core::ptr::write(items::BAIT_BAG.address, v);
                    }),
                    Toggle::new("Bombs", false, |v| unsafe {
                        let v = if v { items::BOMBS.value } else { 0 };
                        core::ptr::write(items::BOMBS.address, v);
                    }),
                    Toggle::new("Boomerang", false, |v| unsafe {
                        let v = if v { items::BOOMERANG.value } else { 0 };
                        core::ptr::write(items::BOOMERANG.address, v);
                    }),
                    Toggle::new("Deku Leaf", false, |v| unsafe {
                        let v = if v { items::DEKU_LEAF.value } else { 0 };
                        core::ptr::write(items::DEKU_LEAF.address, v);
                    }),
                    Toggle::new("Delivery Bag", false, |v| unsafe {
                        let v = if v { items::DELIVERY_BAG.value } else { 0 };
                        core::ptr::write(items::DELIVERY_BAG.address, v);
                    }),
                    Select::new(
                        "Picto Box",
                        vec![
                            ("None", 0xff),
                            ("Normal", items::PICTO_BOX.value),
                            ("Deluxe", items::DELUXE_BOX.value),
                        ],
                        |_, v| unsafe {
                            core::ptr::write(items::PICTO_BOX.address, v.value);
                        },
                    ),
                    Toggle::new("Grappling Hook", false, |v| unsafe {
                        let v = if v { items::GRAPPLING_HOOK.value } else { 0 };
                        core::ptr::write(items::GRAPPLING_HOOK.address, v);
                    }),
                    Select::new(
                        "Hero's Bow",
                        vec![
                            ("None", 0),
                            ("Normal", items::HERO_BOW.value),
                            ("Fire & Ice", items::ELEMENTAL_BOW.value),
                            ("Light", items::MAGICAL_BOW.value),
                        ],
                        |_, v| unsafe {
                            core::ptr::write(items::HERO_BOW.address, v.value);
                        },
                    ),
                    Toggle::new("Hero's Charm", false, |v| unsafe {
                        let v = if v { items::HERO_CHARM.value } else { 0 };
                        core::ptr::write(items::HERO_CHARM.address, v);
                    }),
                    Select::new(
                        "Hero's Shield",
                        vec![
                            ("None", 0xff),
                            ("Normal", items::HERO_SHIELD.value),
                            ("Mirror", items::MIRROR_SHIELD.value),
                        ],
                        |_, v| unsafe {
                            core::ptr::write(items::HERO_SHIELD.address, v.value);
                        },
                    ),
                    Select::new(
                        "Hero's Sword",
                        vec![
                            ("None", 0xff),
                            ("Normal", items::HERO_SWORD.value),
                            ("Master 1", items::MASTER_SWORD_1.value),
                            ("Master 2", items::MASTER_SWORD_2.value),
                            ("Master 3", items::MASTER_SWORD_3.value),
                        ],
                        |_, v| unsafe {
                            core::ptr::write(items::HERO_SWORD.address, v.value);
                        },
                    ),
                    Toggle::new("Hookshot", false, |v| unsafe {
                        let v = if v { items::HOOKSHOT.value } else { 0 };
                        core::ptr::write(items::HOOKSHOT.address, v);
                    }),
                    Toggle::new("Iron Boots", false, |v| unsafe {
                        let v = if v { items::IRON_BOOTS.value } else { 0 };
                        core::ptr::write(items::IRON_BOOTS.address, v);
                    }),
                    Toggle::new("Magic Armor", false, |v| unsafe {
                        let v = if v { items::MAGIC_ARMOR.value } else { 0 };
                        core::ptr::write(items::MAGIC_ARMOR.address, v);
                    }),
                    Toggle::new("Power Bracelets", false, |v| unsafe {
                        let v1 = if v {
                            items::POWER_BRACELETS_1.value
                        } else {
                            0xff
                        };
                        let v2 = if v { items::POWER_BRACELETS_2.value } else { 0 };

                        core::ptr::write(items::POWER_BRACELETS_1.address, v1);
                        core::ptr::write(items::POWER_BRACELETS_2.address, v2);
                    }),
                    Toggle::new("Skull Hammer", false, |v| unsafe {
                        let v = if v { items::SKULL_HAMMER.value } else { 0 };
                        core::ptr::write(items::SKULL_HAMMER.address, v);
                    }),
                    Toggle::new("Spoils Bag", false, |v| unsafe {
                        let v = if v { items::SPOILS_BAG.value } else { 0 };
                        core::ptr::write(items::SPOILS_BAG.address, v);
                    }),
                    Toggle::new("Telescope", false, |v| unsafe {
                        let v = if v { items::TELESCOPE.value } else { 0 };
                        core::ptr::write(items::TELESCOPE.address, v);
                    }),
                    Toggle::new("Tingle Bottle", false, |v| unsafe {
                        let v = if v { items::TINGLE_BOTTLE.value } else { 0 };
                        core::ptr::write(items::TINGLE_BOTTLE.address, v);
                    }),
                    Toggle::new("Wind Waker", false, |v| unsafe {
                        let v = if v { items::WIND_WAKER.value } else { 0 };
                        core::ptr::write(items::WIND_WAKER.address, v);
                    }),
                    Select::new(
                        "Sail",
                        vec![
                            ("None", 0xff),
                            ("Normal", items::NORMAL_SAIL.value),
                            ("Swift", items::SWIFT_SAIL.value),
                        ],
                        |_, v| unsafe {
                            core::ptr::write(items::NORMAL_SAIL.address, v.value);
                        },
                    ),
                    Menu::new(
                        "Bottles",
                        vec![
                            Select::new("Bottle 1", bottle_options.clone(), |_, v| unsafe {
                                core::ptr::write(items::BOTTLE_1.address, v.value);
                            }),
                            Select::new("Bottle 2", bottle_options.clone(), |_, v| unsafe {
                                core::ptr::write(items::BOTTLE_2.address, v.value);
                            }),
                            Select::new("Bottle 3", bottle_options.clone(), |_, v| unsafe {
                                core::ptr::write(items::BOTTLE_3.address, v.value);
                            }),
                            Select::new("Bottle 4", bottle_options.clone(), |_, v| unsafe {
                                core::ptr::write(items::BOTTLE_4.address, v.value);
                            }),
                        ],
                    ),
                    Menu::new(
                        "Songs",
                        vec![
                            Toggle::new("Wind's Requiem", false, |v| unsafe {
                                let x = core::ptr::read(items::WINDS_REQUIEM.address);
                                let x = if v {
                                    x | items::WINDS_REQUIEM.value
                                } else {
                                    x & !items::WINDS_REQUIEM.value
                                };
                                core::ptr::write(items::WINDS_REQUIEM.address, x);
                            }),
                            Toggle::new("Ballad of Gales", false, |v| unsafe {
                                let x = core::ptr::read(items::BALLAD_OF_GALES.address);
                                let x = if v {
                                    x | items::BALLAD_OF_GALES.value
                                } else {
                                    x & !items::BALLAD_OF_GALES.value
                                };
                                core::ptr::write(items::BALLAD_OF_GALES.address, x);
                            }),
                            Toggle::new("Command Melody", false, |v| unsafe {
                                let x = core::ptr::read(items::COMMAND_MELODY.address);
                                let x = if v {
                                    x | items::COMMAND_MELODY.value
                                } else {
                                    x & !items::COMMAND_MELODY.value
                                };
                                core::ptr::write(items::COMMAND_MELODY.address, x);
                            }),
                            Toggle::new("Earth God's Lyrics", false, |v| unsafe {
                                let x = core::ptr::read(items::EARTH_GODS_LYRICS.address);
                                let x = if v {
                                    x | items::EARTH_GODS_LYRICS.value
                                } else {
                                    x & !items::EARTH_GODS_LYRICS.value
                                };
                                core::ptr::write(items::EARTH_GODS_LYRICS.address, x);
                            }),
                            Toggle::new("Wind God's Aria", false, |v| unsafe {
                                let x = core::ptr::read(items::WIND_GODS_ARIA.address);
                                let x = if v {
                                    x | items::WIND_GODS_ARIA.value
                                } else {
                                    x & !items::WIND_GODS_ARIA.value
                                };
                                core::ptr::write(items::WIND_GODS_ARIA.address, x);
                            }),
                            Toggle::new("Song of Passing", false, |v| unsafe {
                                let x = core::ptr::read(items::SONG_OF_PASSING.address);
                                let x = if v {
                                    x | items::SONG_OF_PASSING.value
                                } else {
                                    x & !items::SONG_OF_PASSING.value
                                };
                                core::ptr::write(items::SONG_OF_PASSING.address, x);
                            }),
                        ],
                    ),
                    Number::new("Triforce", 0u8, 1, 0, 8, |v| unsafe {
                        let x = if *v == 8 { 0xff } else { (1 << *v) - 1 };
                        core::ptr::write(items::TRIFORCE.address, x);
                    }),
                    Menu::new(
                        "Pearls",
                        vec![
                            Toggle::new("Nayru's Pearl", false, |v| unsafe {
                                let x = core::ptr::read(items::NAYRUS_PEARL.address);
                                let x = if v {
                                    x | items::NAYRUS_PEARL.value
                                } else {
                                    x & !items::NAYRUS_PEARL.value
                                };
                                core::ptr::write(items::NAYRUS_PEARL.address, x);
                            }),
                            Toggle::new("Din's Pearl", false, |v| unsafe {
                                let x = core::ptr::read(items::DINS_PEARL.address);
                                let x = if v {
                                    x | items::DINS_PEARL.value
                                } else {
                                    x & !items::DINS_PEARL.value
                                };
                                core::ptr::write(items::DINS_PEARL.address, x);
                            }),
                            Toggle::new("Farore's Pearl", false, |v| unsafe {
                                let x = core::ptr::read(items::FARORES_PEARL.address);
                                let x = if v {
                                    x | items::FARORES_PEARL.value
                                } else {
                                    x & !items::FARORES_PEARL.value
                                };
                                core::ptr::write(items::FARORES_PEARL.address, x);
                            }),
                        ],
                    ),
                    Menu::new(
                        "Mailbag",
                        vec![
                            Select::new("Item 1", mailbag_options.clone(), |_, v| unsafe {
                                core::ptr::write(items::MAILBAG_1.address, v.value);
                            }),
                            Select::new("Item 2", mailbag_options.clone(), |_, v| unsafe {
                                core::ptr::write(items::MAILBAG_2.address, v.value);
                            }),
                            Select::new("Item 3", mailbag_options.clone(), |_, v| unsafe {
                                core::ptr::write(items::MAILBAG_3.address, v.value);
                            }),
                            Select::new("Item 4", mailbag_options.clone(), |_, v| unsafe {
                                core::ptr::write(items::MAILBAG_4.address, v.value);
                            }),
                            Select::new("Item 5", mailbag_options.clone(), |_, v| unsafe {
                                core::ptr::write(items::MAILBAG_5.address, v.value);
                            }),
                            Select::new("Item 6", mailbag_options.clone(), |_, v| unsafe {
                                core::ptr::write(items::MAILBAG_6.address, v.value);
                            }),
                            Select::new("Item 7", mailbag_options.clone(), |_, v| unsafe {
                                core::ptr::write(items::MAILBAG_7.address, v.value);
                            }),
                            Select::new("Item 8", mailbag_options.clone(), |_, v| unsafe {
                                core::ptr::write(items::MAILBAG_8.address, v.value);
                            }),
                        ],
                    ),
                ],
            ),
            Menu::new(
                "Spoofs",
                vec![
                    Select::new(
                        &format!("{}", icons::BTN_X),
                        vec![
                            ("Bombs", items::BOMBS.value),
                            ("Boomerang", items::BOOMERANG.value),
                            ("Deku Leaf", items::DEKU_LEAF.value),
                            ("Deluxe Box", items::DELUXE_BOX.value),
                            ("Grappling Hook", items::GRAPPLING_HOOK.value),
                            ("Hero's Bow", items::HERO_BOW.value),
                            ("Hookshot", items::HOOKSHOT.value),
                            ("Iron Boots", items::IRON_BOOTS.value),
                            ("Magic Armor", items::MAGIC_ARMOR.value),
                            ("Picto Box", items::PICTO_BOX.value),
                            ("Skull Hammer", items::SKULL_HAMMER.value),
                            ("Telescope", items::TELESCOPE.value),
                            ("Tingle Bottle", items::TINGLE_BOTTLE.value),
                            ("Wind Waker", items::WIND_WAKER.value),
                        ],
                        |_, v| unsafe {
                            core::ptr::write(player::BUTTON_X, v.value);
                        },
                    ),
                    Select::new(
                        &format!("{}", icons::BTN_Y),
                        vec![
                            ("Bombs", items::BOMBS.value),
                            ("Boomerang", items::BOOMERANG.value),
                            ("Deku Leaf", items::DEKU_LEAF.value),
                            ("Deluxe Box", items::DELUXE_BOX.value),
                            ("Grappling Hook", items::GRAPPLING_HOOK.value),
                            ("Hero's Bow", items::HERO_BOW.value),
                            ("Hookshot", items::HOOKSHOT.value),
                            ("Iron Boots", items::IRON_BOOTS.value),
                            ("Magic Armor", items::MAGIC_ARMOR.value),
                            ("Picto Box", items::PICTO_BOX.value),
                            ("Skull Hammer", items::SKULL_HAMMER.value),
                            ("Telescope", items::TELESCOPE.value),
                            ("Tingle Bottle", items::TINGLE_BOTTLE.value),
                            ("Wind Waker", items::WIND_WAKER.value),
                        ],
                        |_, v| unsafe {
                            core::ptr::write(player::BUTTON_Y, v.value);
                        },
                    ),
                    Select::new(
                        &format!("{}", icons::BTN_R),
                        vec![
                            ("Bombs", items::BOMBS.value),
                            ("Boomerang", items::BOOMERANG.value),
                            ("Deku Leaf", items::DEKU_LEAF.value),
                            ("Deluxe Box", items::DELUXE_BOX.value),
                            ("Grappling Hook", items::GRAPPLING_HOOK.value),
                            ("Hero's Bow", items::HERO_BOW.value),
                            ("Hookshot", items::HOOKSHOT.value),
                            ("Iron Boots", items::IRON_BOOTS.value),
                            ("Magic Armor", items::MAGIC_ARMOR.value),
                            ("Picto Box", items::PICTO_BOX.value),
                            ("Skull Hammer", items::SKULL_HAMMER.value),
                            ("Telescope", items::TELESCOPE.value),
                            ("Tingle Bottle", items::TINGLE_BOTTLE.value),
                            ("Wind Waker", items::WIND_WAKER.value),
                        ],
                        |_, v| unsafe {
                            core::ptr::write(player::BUTTON_R, v.value);
                        },
                    ),
                ],
            ),
            Menu::new(
                "Stage",
                vec![
                    Text::new(|| unsafe {
                        let stage = 0x109763f0 as *mut [u8; 8];
                        let spawn = 0x109763f9 as *mut u8;
                        let room = 0x109763fa as *mut u8;
                        let layer = 0x109763fb as *mut u8;

                        format!(
                            "Stage: {}, Spawn: {}, Room: {}, Layer: {}",
                            stages::value_to_name(core::ptr::read(stage)),
                            core::ptr::read(spawn),
                            core::ptr::read(room),
                            core::ptr::read(layer)
                        )
                    }),
                    Select::new(
                        "Great Sea",
                        vec![
                            "Forsaken Fortress",
                            "Star Island",
                            "N. Fairy Island",
                            "Gale Island",
                            "Crescent Moon Island",
                            "Seven-Star Isles",
                            "Overlook Island",
                            "Four-Eye Reef",
                            "Mother & Child Isle",
                            "Spectacle Island",
                            "Windfall Island",
                            "Pawprint Isle",
                            "Dragon Roost Mt",
                            "Flight Control Platform",
                            "W. Fairy Island",
                            "Rock Spire Isle",
                            "Tingle Island",
                            "N. Triangle Isle",
                            "E. Fairy Isle",
                            "Fire Mountain",
                            "Star Belt Archipelago",
                            "Three-Eye Isle",
                            "Greatfish Isle",
                            "Cyclops Reef",
                            "Six-Eye Reef",
                            "Tower of Gods",
                            "E. Triangle Isle",
                            "Thorned Fairy Island",
                            "Neele Rock Isle",
                            "Islet of Steel",
                            "Stonewatcher Island",
                            "S. Triangle Isle",
                            "Links Oasis",
                            "Bomb Island",
                            "Bird's Peak Rock",
                            "Diamond Steppe Island",
                            "Five-Eye Reef",
                            "Shark Island",
                            "S. Fairy Island",
                            "Ice Ring Isle",
                            "Forest Haven",
                            "Cliff Plateau Isles",
                            "Horseshoe Isle",
                            "Outset Island",
                            "Headstone Island",
                            "Two-Eye Reef",
                            "Angular Isles",
                            "Boating Course",
                            "Five-Star Isles",
                        ],
                        |i, _| unsafe {
                            // map
                            let stage = 0x109763f0 as *mut [u8; 8];
                            core::ptr::write(stage, *b"sea\0\0\0\0\0");

                            // spawn ID
                            let spawn = 0x109763f9 as *mut u8;
                            core::ptr::write(spawn, 0);

                            // room ID
                            let room = 0x109763fa as *mut u8;
                            core::ptr::write(room, i as u8 + 1);

                            // layer ID
                            let layer = 0x109763fb as *mut u8;
                            core::ptr::write(layer, 0xff);

                            // responsible for reload?
                            let ptr = 0x109763fc as *mut u8;
                            core::ptr::write(ptr, 0x01);
                        },
                    ),
                ],
            ),
        ],
    ));

    let mut input = unsafe { INPUT };

    while thread::current().running() {
        // println!("thread: {}", time::DateTime::now());

        if input != unsafe { INPUT } {
            input = unsafe { INPUT };

            overlay.run(input);
        }

        unsafe {
            let cheats = cheats.lock().unwrap();

            if cheats.health {
                core::ptr::write(player::HEALTH, 80);
            }
            if cheats.magic {
                core::ptr::write(player::MAGIC, 32);
            }
            if cheats.rupees {
                core::ptr::write(player::RUPEES, 5000);
            }
            if cheats.arrows {
                core::ptr::write(player::ARROWS, 99);
            }
            if cheats.bombs {
                core::ptr::write(player::BOMBS, 99);
            }
            if cheats.air {
                core::ptr::write(player::AIR, 900);
            }
            if cheats.super_swim {
                // kinda magic and not the same as "normal" superswims
                core::ptr::write(player::AIR as *mut u32, 0xdff);
            }
            if cheats.super_crouch {
                core::ptr::write(player::SUPER_CROUCH, 0x41f00000);
            }
        }

        unsafe {
            wut::bindings::GX2WaitForFlip();
        }
    }

    logger::deinit();
}

#[on_application_exit(Udp)]
fn stop() {
    //     // println!("stop");

    let mut h = HANDLE.write();
    if let Some(handle) = h.take() {
        handle.thread().cancel();
        println!("{:?}", handle.join());
    }
}
