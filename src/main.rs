#![no_std]
#![no_main]

use core::sync::atomic::{AtomicUsize, Ordering};

use wut::{font::icons, gamepad::GamepadState};
use wut::prelude::*;
use wut::*;

use notifications;
use overlay;
use wups::*;

use alloc::sync::Arc;
use wut::sync::{Mutex, LazyLock};

mod items;
mod misc;
mod player;
mod stages;

WUPS_PLUGIN_NAME!("WWHD Trainer");

static HANDLE: LazyLock<Mutex<Option<thread::JoinHandle>>> = LazyLock::new(|| Mutex::new(None));

static INPUT: LazyLock<Mutex<gamepad::GamepadState>> = LazyLock::new(|| Mutex::new(GamepadState::new()));

static FRAME_LIMITER: sync::LazyLock<sync::AutoEvent> =
    sync::LazyLock::new(|| sync::AutoEvent::new());

/// Check if current title is Wind Waker
fn title_is_windwaker() -> bool {
    const VALID_TITLE_IDS: [u64; 4] = [
        0x00050000_10143600, // EUR
        0x00050000_10143500, // USA
        0x00050000_10143400, // JPN
        0x00050000_10143599, // RANDOMIZER
    ];

    let title = wut::title::current_title();

    VALID_TITLE_IDS.iter().any(|&id| id == title)
}

#[function_hook(module = VPAD, function = VPADRead)]
fn my_VPADRead(
    chan: wut::bindings::VPADChan::Type,
    buffers: *mut wut::bindings::VPADStatus,
    count: u32,
    error: *mut wut::bindings::VPADReadError::Type,
) -> i32 {
    // unsafe {
    //     if PAUSE_ENABLED {
    //         (*buffers).trigger |= wut::bindings::VPADButtons::VPAD_BUTTON_HOME;
    //     }
    // }

    let status = unsafe { hooked(chan, buffers, count, error) };

    if status != 0 {
        use wut::gamepad::{Button, Joystick};

        let mut input = unsafe { gamepad::GamepadState::from(*buffers) };

        *INPUT.lock().unwrap() = input;

        if input.hold.contains(Button::L | Button::R) {
            input.hold = Button::none();
            input.trigger = Button::none();
            input.release = Button::none();
        }

        //

        let counter = FRAME_COUNTER.load(Ordering::Relaxed);

        if unsafe { MSS_ENABLED } {
            // input.left_stick = if counter % 2 == 0 {
            //     Some(Joystick::new(0.0, 1.0))
            // } else {
            //     Some(Joystick::new(0.0, -1.0))
            // };
            // unsafe {
            //     *buffers &= MSS[counter % 2];
            // }
            input |= MSS[counter % 2];
        }
        if unsafe { ZOMBIE_ENABLED } {
            if counter % 2 == 0 {
                input.hold |= Button::A | Button::B;
            }
        }
        if unsafe { PAUSE_ENABLED } {
            input.hold = Button::A | Button::ZL;
            input.trigger = Button::Home.into();
            input.left_stick = Some(Joystick::new(1.0, 0.0));
            unsafe {
                PAUSE_ENABLED = false;
                foreground::home_menu();
            }
        }

        //

        unsafe {
            *buffers &= input;
        }
    }

    status
}

static mut MSS_ENABLED: bool = false;
static mut ZOMBIE_ENABLED: bool = false;
static mut PAUSE_ENABLED: bool = false;

static MSS: sync::LazyLock<Vec<wut::gamepad::GamepadState>> = sync::LazyLock::new(|| {
    use wut::gamepad::*;
    vec![
        GamepadState {
            hold: Button::none(),
            trigger: Button::none(),
            release: Button::none(),
            left_stick: Some(Joystick::new(0.0, 1.0)),
            right_stick: None,
        },
        GamepadState {
            hold: Button::none(),
            trigger: Button::none(),
            release: Button::none(),
            left_stick: Some(Joystick::new(0.0, -1.0)),
            right_stick: None,
        },
    ]
});

// static MACROS: sync::RwLock<Vec<Macro>> = sync::RwLock::new(vec![]);

/// Counter increasing once every frame (right after swap)
static FRAME_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[function_hook(module = GX2, function = GX2SwapScanBuffers)]
fn my_swapBuffers() {
    unsafe {
        hooked();
    }
    FRAME_COUNTER.fetch_add(1, Ordering::Relaxed);
    FRAME_LIMITER.signal();
}

#[on_application_start(Udp)]
fn start() {
    if !title_is_windwaker() {
        return;
    }

    // let macros = MACROS.write();
    // macros.push(value);

    let mut thread = HANDLE.lock().unwrap();
    if thread.is_none() {
        *thread = Some(
            thread::Builder::default()
                .name("WWHD Trainer")
                .attribute(thread::thread::ThreadAttribute::Cpu2)
                .spawn(overlay_thread)
                .unwrap(),
        );
    }

    let _ = notifications::info("WWHD Trainer started").show().unwrap();
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

impl Cheats {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(Self::default()))
    }

    pub fn run(&self) {
        unsafe {
            if self.health {
                core::ptr::write(player::HEALTH, 80);
            }
            if self.magic {
                core::ptr::write(player::MAGIC, 32);
            }
            if self.rupees {
                core::ptr::write(player::RUPEES, 5000);
            }
            if self.arrows {
                core::ptr::write(player::ARROWS, 99);
            }
            if self.bombs {
                core::ptr::write(player::BOMBS, 99);
            }
            if self.air {
                core::ptr::write(player::AIR, 900);
            }
            if self.super_swim {
                // kinda magic and not the same as "normal" superswims
                core::ptr::write(player::AIR as *mut u32, 0xdff);
            }
            if self.super_crouch {
                core::ptr::write(player::SUPER_CROUCH, 0x41f00000);
            }
        }
    }
}

struct SpeedPopup {
    popup: Arc<Mutex<Option<notifications::Notification>>>,
}

impl SpeedPopup {
    pub fn new() -> Self {
        Self {
            popup: Arc::new(Mutex::new(None)),
        }
    }

    pub fn popup(&self) -> Arc<Mutex<Option<notifications::Notification>>> {
        Arc::clone(&self.popup)
    }

    pub fn render(&mut self) {
        let p = self.popup();
        if let Some(ref popup) = *p.lock().unwrap() {
            let (speed, facing_angle, speed_angle) = unsafe {
                (
                    {
                        let x = player::position::SPEED_PTR;
                        let x = core::ptr::read(x);
                        let x = (x + player::position::SPEED_OFFSET) as *mut f32;

                        if wut::ptr::is_valid(x) {
                            core::ptr::read(x)
                        } else {
                            0.0
                        }
                    },
                    core::ptr::read(player::position::FACING_ANGLE),
                    core::ptr::read(player::position::SPEED_ANGLE),
                )
            };

            let _ = popup.text(&format!(
                "Speed: {:.2}, Facing Angle: {:5}, Speed Angle: {:5}",
                speed, facing_angle, speed_angle
            ));
        };
    }
}

#[derive(Debug, Default)]
struct PositionRestore {
    x: f32,
    y: f32,
    z: f32,
}

impl PositionRestore {
    pub fn new() -> Arc<Mutex<PositionRestore>> {
        Arc::new(Mutex::new(Self::default()))
    }
}

fn overlay_thread() {
    let _ = logger::udp().unwrap();

    use overlay::*;

    let cheats = Cheats::new();

    let mut speed_popup = SpeedPopup::new();

    let pos_restore = PositionRestore::new();

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
                // println!("Start search");
                // let start = 0x1000_0004;
                // let to = start + 0x0100_0000;

                // // 0x1098_9c74

                // for (_i, x) in (start..=to).step_by(16).enumerate() {
                //     let ptr = x as *const f32;
                //     let value = core::ptr::read(ptr);
                //     if value > 17.9 && value < 18.1 {
                //         println!("ptr: {:#08x} - value: {}", ptr as usize, value);
                //     }
                // }

                // println!("End search");

                let ptr = 0x17f4_63bc as *mut u8;
                let value = core::ptr::write(ptr, 1);
                println!("{:x?}", &value);
            }),
            Button::new("Test", || unsafe {
                let ptr = 0x0113_F508 as *mut u32;
                let value = core::ptr::read(ptr);
                println!("ptr: {:#08x} - value: {:?}", ptr as usize, value);
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
                    Button::new("Complete Map", || unsafe {
                        // this is a one-way for now / just don't save ^^
                        core::ptr::write(player::OVERWORLD_MAP, [3; 49]);
                    }),
                    Toggle::new("Super Swim", false, {
                        let super_swim = Arc::clone(&cheats);
                        move |v| {
                            if !v {
                                unsafe {
                                    core::ptr::write(player::AIR as *mut u32, 800);
                                }
                            }
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
                    Button::new("Hover", || unsafe {
                        let x = core::ptr::read(player::HOVER_PTR);
                        let x = (x + player::HOVER_OFFSET) as *mut u32;

                        if wut::ptr::is_valid(x) {
                            core::ptr::write(x, 0x4210_0000);

                            let ptr = core::ptr::read(player::position::SPEED_PTR);
                            let ptr = (ptr + player::position::SPEED_OFFSET) as *mut f32;

                            if wut::ptr::is_valid(ptr) {
                                let input = INPUT.lock().unwrap();

                                core::ptr::write(ptr, 30.0 * input.left_stick.unwrap().abs());

                                if let Some(angle) = input.left_stick.unwrap().angle() {
                                    core::ptr::write(
                                        player::position::SPEED_ANGLE,
                                        core::ptr::read(player::position::FACING_ANGLE) + angle,
                                    );
                                }
                            }
                        }
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
                    Menu::new(
                        "Dungeon",
                        vec![
                            Number::new("Dungeon Keys", 0, 1, 0, 10, |v| unsafe {
                                core::ptr::write(items::DUNGEON_KEYS.address, *v);
                            }),
                            Toggle::new("Map", false, |v| unsafe {
                                let x = core::ptr::read(items::DUNGEON_MAP.address);
                                let x = if v {
                                    x | items::DUNGEON_MAP.value
                                } else {
                                    x & !items::DUNGEON_MAP.value
                                };
                                core::ptr::write(items::DUNGEON_MAP.address, x);
                            }),
                            Toggle::new("Compass", false, |v| unsafe {
                                let x = core::ptr::read(items::DUNGEON_COMPASS.address);
                                let x = if v {
                                    x | items::DUNGEON_COMPASS.value
                                } else {
                                    x & !items::DUNGEON_COMPASS.value
                                };
                                core::ptr::write(items::DUNGEON_COMPASS.address, x);
                            }),
                            Toggle::new("Boss Key", false, |v| unsafe {
                                let x = core::ptr::read(items::DUNGEON_BOSS_KEY.address);
                                let x = if v {
                                    x | items::DUNGEON_BOSS_KEY.value
                                } else {
                                    x & !items::DUNGEON_BOSS_KEY.value
                                };
                                core::ptr::write(items::DUNGEON_BOSS_KEY.address, x);
                            }),
                        ],
                    ),
                    Menu::new("Spoils", {
                        let value = 1;
                        let inc = 10;
                        let min = 0;
                        let max = 99;
                        vec![
                            Number::new("Red Chu Jelly", value, inc, min, max, |v| unsafe {
                                core::ptr::write(items::RED_JELLY.address, *v);
                            }),
                            Number::new("Green Chu Jelly", value, inc, min, max, |v| unsafe {
                                core::ptr::write(items::GREEN_JELLY.address, *v);
                            }),
                            Number::new("Blue Chu Jelly", value, inc, min, max, |v| unsafe {
                                core::ptr::write(items::BLUE_JELLY.address, *v);
                            }),
                            Number::new("Joy Pendant", value, inc, min, max, |v| unsafe {
                                core::ptr::write(items::JOY_PENDANT.address, *v);
                            }),
                            Number::new("Boko Baba Seed", value, inc, min, max, |v| unsafe {
                                core::ptr::write(items::BOKO_SEEDS.address, *v);
                            }),
                            Number::new("Golden Feather", value, inc, min, max, |v| unsafe {
                                core::ptr::write(items::GOLDEN_FEATHERS.address, *v);
                            }),
                            Number::new("Skull Necklace", value, inc, min, max, |v| unsafe {
                                core::ptr::write(items::SKULL_NECKLACES.address, *v);
                            }),
                            Number::new("Knight's Crest", value, inc, min, max, |v| unsafe {
                                core::ptr::write(items::KNIGHT_CREST.address, *v);
                            }),
                        ]
                    }),
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
                "Coordinates",
                vec![
                    Text::new(|| unsafe {
                        format!(
                            "X: {:.2}, Y: {:.2}, Z: {:.2}",
                            core::ptr::read(player::position::X),
                            core::ptr::read(player::position::Y),
                            core::ptr::read(player::position::Z)
                        )
                    }),
                    Toggle::new("Show Speed", false, {
                        let popup = speed_popup.popup();
                        move |v| {
                            let mut p = popup.lock().unwrap();
                            if v && p.is_none() {
                                *p = Some(
                                    notifications::dynamic("Speed: ..., Angle: ...")
                                        .show()
                                        .unwrap(),
                                );
                            } else {
                                let _ = p.take();
                            }
                        }
                    }),
                    Button::new("Store position", {
                        let pos = Arc::clone(&pos_restore);
                        move || unsafe {
                            let mut p = pos.lock().unwrap();
                            p.x = core::ptr::read(player::position::X);
                            p.y = core::ptr::read(player::position::Y);
                            p.z = core::ptr::read(player::position::Z);
                        }
                    }),
                    Button::new("Restore position", {
                        let pos = Arc::clone(&pos_restore);
                        move || unsafe {
                            let p = pos.lock().unwrap();

                            // Enable and disable door cancel / collisions for teleport

                            // let door_cancel = (core::ptr::read(misc::LINK_PTR)
                            //     + misc::COLLISION_OFFSET)
                            //     as *mut u32;

                            // if wut::ptr::is_valid(door_cancel) {
                            //     let value = core::ptr::read(door_cancel) | 0x4004;
                            //     core::ptr::write(door_cancel, value);
                            // }

                            core::ptr::write(player::position::X, p.x);
                            core::ptr::write(player::position::Y, p.y);
                            core::ptr::write(player::position::Z, p.z);

                            // if wut::ptr::is_valid(door_cancel) {
                            //     let value = core::ptr::read(door_cancel) & !0x4004;
                            //     core::ptr::write(door_cancel, value);
                            // }
                        }
                    }),
                ],
            ),
            Menu::new(
                "Storage",
                vec![
                    Toggle::new("Storage", false, |v| {
                        misc::storage(v);
                    }),
                    Toggle::new("Chest Storage", false, |v| {
                        misc::chest_storage(v);
                    }),
                    Toggle::new("Door Cancel", false, |v| {
                        misc::door_cancel(v);
                    }),
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
                            core::ptr::write(stages::STAGE_ID, *b"sea\0\0\0\0\0");
                            core::ptr::write(stages::SPAWN_ID, 0);
                            core::ptr::write(stages::ROOM_ID, i as u8 + 1);
                            core::ptr::write(stages::STAGE_LAYER, 0xff);
                            core::ptr::write(stages::RELOAD, 0x01);
                        },
                    ),
                    Select::new(
                        "Dungeon",
                        vec![
                            (
                                "Forsaken Fortress 1",
                                (stages::FORSAKEN_FORTRESS_EXTERIOR.value, 2, 0, 255),
                            ), // not actually
                            (
                                "Dragon Roost Cavern",
                                (stages::DRAGON_ROOST_CAVEN.value, 0, 0, 255),
                            ),
                            (
                                "Forbidden Woods",
                                (stages::FORBIDDEN_WOODS.value, 0, 0, 255),
                            ),
                            (
                                "Tower of the Gods",
                                (stages::TOWER_OF_GODS.value, 0, 0, 255),
                            ),
                            ("Earth Temple", (stages::EARTH_TEMPLE.value, 0, 0, 255)),
                            ("Wind Temple", (stages::WIND_TEMPLE.value, 0, 0, 255)),
                            (
                                "Ganon's Tower",
                                (stages::GANONS_TOWER_ENTRANCE.value, 0, 0, 255),
                            ),
                        ],
                        |_, v| unsafe {
                            core::ptr::write(stages::STAGE_ID, v.value.0);
                            core::ptr::write(stages::SPAWN_ID, v.value.1);
                            core::ptr::write(stages::ROOM_ID, v.value.2);
                            core::ptr::write(stages::STAGE_LAYER, v.value.3);
                            core::ptr::write(stages::RELOAD, 0x01);
                        },
                    ),
                    Button::new("Reload stage", || unsafe {
                        core::ptr::write(stages::RELOAD, 1);
                    }),
                    Select::new(
                        "Daytime",
                        vec![
                            ("Dawn", stages::daytime::DAWN),
                            ("Day", stages::daytime::DAY),
                            ("Night", stages::daytime::NIGHT),
                        ],
                        |_, v| unsafe {
                            core::ptr::write(stages::daytime::ADDRESS, v.value);
                        },
                    ),
                    Select::new(
                        "Weather",
                        vec![
                            ("Normal", stages::weather::NORMAL),
                            ("Cloudy", stages::weather::CLOUDY),
                            ("Foggy", stages::weather::FOGGY),
                        ],
                        |_, v| unsafe {
                            core::ptr::write(stages::weather::ADDRESS, v.value);
                        },
                    ),
                ],
            ),
            Menu::new(
                "Macros",
                vec![
                    Toggle::new("MSS", false, |v| unsafe {
                        MSS_ENABLED = v;
                    }),
                    Toggle::new("Zombie Hover", false, |v| unsafe {
                        ZOMBIE_ENABLED = v;
                    }),
                    Button::new("Pause Storage", || unsafe {
                        PAUSE_ENABLED = true;
                    }),
                ],
            ),
        ],
    ));

    while thread::current().running() {
        overlay.run(*INPUT.lock().unwrap());

        speed_popup.render();

        cheats.lock().unwrap().run();

        FRAME_LIMITER.wait(); //_timeout(time::Duration::from_millis(100));
    }

    logger::deinit();
}

#[on_application_exit(Udp)]
fn stop() {
    let mut h = HANDLE.lock().unwrap();
    if let Some(handle) = h.take() {
        FRAME_LIMITER.signal();
        handle.thread().cancel();
        println!("{:?}", handle.join());
    }
}

// Put off for now. Maybe something like this later.
struct Macro {
    inputs: Vec<wut::gamepad::GamepadState>,
    index: usize,
    enabled: bool,
}

impl Macro {
    pub fn new(inputs: impl Into<Vec<wut::gamepad::GamepadState>>) -> Self {
        Self {
            inputs: inputs.into(),
            index: 0,
            enabled: false,
        }
    }
}

impl Iterator for Macro {
    type Item = wut::gamepad::GamepadState;

    fn next(&mut self) -> Option<Self::Item> {
        let mut value = None;
        if self.index < self.inputs.len() {
            value = Some(self.inputs[self.index]);
            self.index += 1;
        }
        value
    }
}
