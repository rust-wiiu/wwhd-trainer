#![no_std]
#![no_main]

use wut::font::icons;
use wut::prelude::*;
use wut::*;

use overlay;
use wups::*;

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

fn overlay_thread() {
    let _ = logger::init(logger::Udp);

    use overlay::*;

    let mut overlay = OverlayNotification::new(Menu::new(
        "Root",
        vec![
            Button::new("Search", || unsafe {
                println!("Start search");
                let start = 0x1000_0000;
                let to = start + 0x0100_0000;

                for (_i, x) in (start..=to).step_by(4).enumerate() {
                    let ptr = x as *const f32;
                    let _value = core::ptr::read_volatile(ptr);
                }

                println!("End search");
            }),
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
            Menu::new(
                "Health",
                vec![
                    Number::new("Health", 1, 1, 0, 80, |v| unsafe {
                        core::ptr::write_volatile(0x1506b503 as *mut u8, *v);
                        wut::bindings::DCFlushRange(0x1506b503 as *mut core::ffi::c_void, 1);
                    }),
                    Number::new("Containers", 1, 1, 1, 20, |v| unsafe {
                        core::ptr::write_volatile(0x1506b501 as *mut u8, *v * 4);
                        wut::bindings::DCFlushRange(0x1506b501 as *mut core::ffi::c_void, 1);
                    }),
                ],
            ),
            Menu::new(
                "Items",
                vec![Toggle::new("Grappling Hook", false, |v| unsafe {
                    let v = if v { 0x25 } else { 0 };
                    core::ptr::write_volatile(0x1506b53f as *mut u8, v * 4);
                    wut::bindings::DCFlushRange(0x1506b53f as *mut core::ffi::c_void, 1);
                })],
            ),
            Menu::new(
                "Spoofs",
                vec![
                    Select::new(
                        &format!("{}", icons::BTN_X),
                        vec![("Grappling Hook", 0x25), ("Hookshot", 0x2f)],
                        |_, v| unsafe {
                            let address = 0x10976e6b;

                            core::ptr::write_volatile(address as *mut u8, v.value);
                            wut::bindings::DCFlushRange(address as *mut core::ffi::c_void, 1);
                        },
                    ),
                    Select::new(
                        &format!("{}", icons::BTN_Y),
                        vec![("Grappling Hook", 0x25), ("Hookshot", 0x2f)],
                        |_, v| unsafe {
                            let address = 0x10976e6c;

                            core::ptr::write_volatile(address as *mut u8, v.value);
                            wut::bindings::DCFlushRange(address as *mut core::ffi::c_void, 1);
                        },
                    ),
                    Select::new(
                        &format!("{}", icons::BTN_R),
                        vec![("Grappling Hook", 0x25), ("Hookshot", 0x2f)],
                        |_, v| unsafe {
                            let address = 0x10976e6d;

                            core::ptr::write_volatile(address as *mut u8, v.value);
                            wut::bindings::DCFlushRange(address as *mut core::ffi::c_void, 1);
                        },
                    ),
                ],
            ),
            Menu::new(
                "Stage",
                vec![
                    // Button::new("Current", || unsafe {
                    //     let stage = 0x109763f0 as *mut [u8; 8];
                    //     println!("stage: {:02x?}", core::ptr::read(stage));

                    //     let spawn = 0x109763f9 as *mut u8;
                    //     println!("spawn: {:02x?}", core::ptr::read(spawn));

                    //     let room = 0x109763fa as *mut u8;
                    //     println!("room: {:02x?}", core::ptr::read(room));

                    //     let layer = 0x109763fb as *mut u8;
                    //     println!("layer: {:02x?}", core::ptr::read(layer));
                    // }),
                    Text::new(|| unsafe {
                        let stage = 0x109763f0 as *mut [u8; 8];
                        println!("stage: {:02x?}", core::ptr::read(stage));

                        let spawn = 0x109763f9 as *mut u8;
                        // println!("spawn: {:02x?}", core::ptr::read(spawn));

                        let room = 0x109763fa as *mut u8;
                        // println!("room: {:02x?}", core::ptr::read(room));

                        let layer = 0x109763fb as *mut u8;
                        // println!("layer: {:02x?}", core::ptr::read(layer));

                        format!(
                            "Spawn: {}, Room: {}, Layer: {}",
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
