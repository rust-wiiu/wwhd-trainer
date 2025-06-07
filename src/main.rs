#![no_std]
#![no_main]

use wut::font::icons;
use wut::prelude::*;
use wut::*;

use alloc::rc::Rc;
use alloc::sync::Arc;
use core::cell::RefCell;
use notifications;
use overlay;
use wupf::{hook_on_input, hook_on_update, hook_plugin, OnInput, OnUpdate, Plugin, PluginHandler};
use wups::WUPS_PLUGIN_NAME;

mod flags;
mod items;
mod locations;
mod memfile;
mod player;
mod stages;

WUPS_PLUGIN_NAME!("WWHD Trainer");

fn is_windwaker() -> bool {
    matches!(
        wut::title::current_title(),
        0x00050000_10143600 | // EUR
        0x00050000_10143500 | // USA
        0x00050000_10143400 | // JPN
        0x00050000_10143599 // RANDOMIZER
    )
}

#[derive(PluginHandler)]
struct WWHDTrainer {
    active: bool,
    controller: Option<gamepad::Port>,
    frame_advance: Rc<RefCell<FrameAdvance>>,
    state: Rc<RefCell<State>>,
    overlay: overlay::Overlay,
}

unsafe impl Send for WWHDTrainer {}
unsafe impl Sync for WWHDTrainer {}

#[derive(Default)]
pub struct State {
    pub cheats: state::Cheats,
    pub speed_popup: state::Popup,
    pub player_pos: state::PlayerPos,
    pub macros: state::Macros,
    pub recorder: state::InputRecorder<{ 30 * 60 }>,
    pub memfiles: state::Memfiles,
}

impl State {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::default()))
    }
}

mod state {
    use super::*;

    #[derive(Default)]
    pub struct Cheats {
        pub health: bool,
        pub magic: bool,
        pub rupees: bool,
        pub arrows: bool,
        pub bombs: bool,
        pub air: bool,
        pub super_swim: bool,
        pub super_crouch: bool,
        pub hover: bool,
        pub sea_charts: Option<[u8; 49]>,
    }

    impl Cheats {
        pub fn run(&self) {
            if self.health {
                player::health::write(80);
            }
            if self.magic {
                player::magic::write(32);
            }
            if self.rupees {
                player::rupees::write(5000);
            }
            if self.arrows {
                player::arrows::write(99);
            }
            if self.bombs {
                player::bombs::write(99);
            }
            if self.air {
                player::air::write(900);
            }
            if self.super_swim {
                player::super_swim::enable(true);
            }
            if self.super_crouch {
                player::super_crouch::enable(true);
            }
        }

        pub fn hover(&mut self, state: gamepad::State) {
            if !self.hover {
                return;
            }

            player::hover::activate();

            if let Some(stick) = &state.left_stick {
                player::position::speed::write(30.0 * stick.abs());
                if let Some(angle) = stick.angle() {
                    let facing_angle = player::position::facing_angle::read();
                    player::position::speed_angle::write(facing_angle + angle);
                }
            }

            self.hover = false;
        }
    }

    #[derive(Default)]
    pub struct Popup(Option<notifications::Notification>);

    impl Popup {
        pub fn show(&mut self, show: bool) {
            if show && self.0.is_none() {
                self.0 = Some(
                    notifications::dynamic("Speed: ..., Angle: ...")
                        .show()
                        .unwrap(),
                );
            } else {
                self.0 = None;
            }
        }

        pub fn update(&mut self) {
            if let Some(ref mut popup) = self.0 {
                let _ = popup
                    .text(&format!(
                        "Speed: {:.2}, Facing Angle: {:5}, Speed Angle: {:5}",
                        player::position::speed::read(),
                        player::position::facing_angle::read(),
                        player::position::speed_angle::read()
                    ))
                    .unwrap();
            }
        }
    }

    pub struct PlayerPos(memfile::Position);

    impl Default for PlayerPos {
        fn default() -> Self {
            Self(memfile::Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                angle: 0,
            })
        }
    }

    impl PlayerPos {
        pub fn store(&mut self) {
            self.0 = memfile::Position::read();
        }

        pub fn apply(&mut self) {
            self.0.write();
        }
    }

    pub struct Macro<const N: usize> {
        pub enabled: bool,
        index: usize,
        inputs: [gamepad::State; N],
    }

    impl<const N: usize> Macro<N> {
        fn new(inputs: [gamepad::State; N]) -> Self {
            Self {
                enabled: false,
                index: 0,
                inputs,
            }
        }

        // Returns true during execution and false when finished
        pub fn run_once(&mut self, state: &mut gamepad::State) -> bool {
            *state |= self.inputs[self.index];

            if self.index < self.inputs.len() - 1 {
                self.index += 1;
                false
            } else {
                self.index = 0;
                true
            }
        }

        pub fn run_forever(&mut self, state: &mut gamepad::State) {
            *state |= self.inputs[self.index];

            if self.index < self.inputs.len() - 1 {
                self.index += 1;
            } else {
                self.index = 0;
            }
        }
    }

    pub struct Macros {
        pub mms: Macro<2>,
        pub zombie_hover: Macro<2>,
    }

    impl Default for Macros {
        fn default() -> Self {
            use gamepad::{Button, Joystick, State};
            Self {
                mms: Macro::new([
                    State {
                        hold: Button::none(),
                        trigger: Button::none(),
                        release: Button::none(),
                        left_stick: Some(Joystick::new(0.0, 1.0)),
                        right_stick: None,
                    },
                    State {
                        hold: Button::none(),
                        trigger: Button::none(),
                        release: Button::none(),
                        left_stick: Some(Joystick::new(0.0, -1.0)),
                        right_stick: None,
                    },
                ]),
                zombie_hover: Macro::new([
                    State {
                        hold: Button::A | Button::B | Button::ZL,
                        trigger: Button::none(),
                        release: Button::none(),
                        left_stick: None,
                        right_stick: None,
                    },
                    State {
                        hold: Button::ZL.into(),
                        trigger: Button::none(),
                        release: Button::none(),
                        left_stick: None,
                        right_stick: None,
                    },
                ]),
            }
        }
    }

    impl Macros {
        pub fn run(&mut self, input: &mut gamepad::State) {
            if self.mms.enabled {
                self.mms.run_forever(input);
            }

            if self.zombie_hover.enabled {
                self.zombie_hover.run_forever(input);
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Mode {
        Idle,
        Recording,
        Playback,
    }

    pub struct InputRecorder<const N: usize> {
        port: Option<gamepad::Port>,
        buffer: [gamepad::State; N],
        size: usize,
        index: usize,
        mode: Mode,
    }

    impl<const N: usize> Default for InputRecorder<N> {
        fn default() -> Self {
            Self {
                port: None,
                buffer: [gamepad::State::default(); N],
                size: 0,
                index: 0,
                mode: Mode::Idle,
            }
        }
    }

    impl<const N: usize> InputRecorder<N> {
        pub fn advance(&mut self) {
            match self.mode {
                Mode::Idle | Mode::Recording => {}
                Mode::Playback => {
                    self.index += 1;
                }
            }
        }

        pub fn start_recording(&mut self, recording: bool) {
            if recording {
                self.port = None;
                self.mode = Mode::Recording;
                self.size = 0;
                self.index = 0;
            } else {
                self.mode = Mode::Idle;
            }
        }

        pub fn start_playback(&mut self) {
            self.mode = Mode::Playback;
            self.index = 0;
        }

        pub fn record(&mut self, port: gamepad::Port, input: gamepad::State) {
            match self.mode {
                Mode::Recording => {
                    if self.port.is_none() {
                        self.port = Some(port);
                    }

                    if let Some(p) = self.port {
                        if p != port {
                            return;
                        }
                    }

                    if self.size < N {
                        self.buffer[self.size] = input;
                        self.size += 1;
                    } else {
                        self.mode = Mode::Idle;
                    }
                }
                Mode::Playback | Mode::Idle => {}
            }
        }

        pub fn playback(&mut self, port: gamepad::Port, input: &mut gamepad::State) {
            match self.mode {
                Mode::Idle | Mode::Recording => {}
                Mode::Playback => {
                    if let Some(p) = self.port {
                        if p != port {
                            return;
                        }
                    }

                    if self.index < self.size {
                        *input |= self.buffer[self.index];
                    } else {
                        self.mode = Mode::Idle;
                        self.index = 0;
                    }
                }
            }
        }

        pub fn run(&mut self, port: gamepad::Port, input: &mut gamepad::State) {
            if let Some(p) = self.port {
                if p != port {
                    return;
                }
            }

            match self.mode {
                Mode::Recording => {
                    if self.port.is_none() {
                        self.port = Some(port);
                    }

                    if self.size < N {
                        self.buffer[self.size] = *input;
                        self.size += 1;
                    } else {
                        self.mode = Mode::Idle;
                    }
                }
                Mode::Playback => {
                    if self.index < self.size {
                        *input |= self.buffer[self.index];
                    } else {
                        self.mode = Mode::Idle;
                        self.index = 0;
                    }
                }
                Mode::Idle => {}
            }
        }

        pub fn mode(&self) -> Mode {
            self.mode
        }
    }

    #[derive(Default)]
    pub struct Memfiles {
        pub file1: Option<memfile::Memfile>,
        pub file2: Option<memfile::Memfile>,
        pub file3: Option<memfile::Memfile>,
    }
}

struct FrameAdvance {
    enabled: bool,
    main: Option<wut::thread::Thread>,
    control: Option<wut::thread::Thread>,
    barrier: Arc<wut::sync::AutoEvent>,
}

impl FrameAdvance {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            enabled: false,
            main: None,
            control: None,
            barrier: Arc::new(wut::sync::AutoEvent::new()),
        }))
    }

    fn enable(&mut self, enabled: bool) {
        println!("enabled: {}", enabled);

        if enabled && self.control.is_none() {
            let barrier = Arc::clone(&self.barrier);
            let main = self.main.unwrap();
            self.control = Some(
                *wut::thread::spawn(move || {
                    let gp = gamepad::Gamepad::new(gamepad::Port::DRC);

                    loop {
                        barrier.wait();

                        main.park();
                        loop {
                            if let Ok(input) = gp.poll() {
                                if input.trigger.contains(gamepad::Button::LStick) {
                                    main.unpark();
                                    break;
                                }
                            }
                        }
                        main.unpark();
                    }
                })
                .unwrap()
                .thread(),
            );
        } else {
            if let Some(t) = self.control.take() {
                t.cancel();
            }
            self.main.unwrap().unpark();
        }

        self.enabled = enabled;
    }

    fn wait(&self) {
        if self.enabled && self.control.is_some() {
            self.barrier.signal();
        }
    }
}

impl Drop for FrameAdvance {
    fn drop(&mut self) {
        self.barrier.signal();
        if let Some(t) = self.control {
            t.cancel();
        }
    }
}

hook_plugin!(WWHDTrainer);
impl Plugin for WWHDTrainer {
    fn on_init() -> Self {
        use overlay::*;

        let _ = wut::logger::udp();

        let state = State::new();
        let fa = FrameAdvance::new();

        let bottle_options = vec![
            ("None", None),
            ("Empty", Some(items::bottles::Empty)),
            ("Red Elixir", Some(items::bottles::RedElixir)),
            ("Green Elixir", Some(items::bottles::GreenElixir)),
            ("Blue Elixir", Some(items::bottles::BlueElixir)),
            ("Soup (Half)", Some(items::bottles::SoupHalf)),
            ("Soup", Some(items::bottles::Soup)),
            ("Water", Some(items::bottles::Water)),
            ("Fairy", Some(items::bottles::Fairy)),
            ("Pollen", Some(items::bottles::Pollen)),
            ("Magic Water", Some(items::bottles::MagicWater)),
        ];

        let mailbag_options = vec![
            ("None", None),
            ("Town Flower", Some(items::mailbag::TownFlower)),
            ("Sea Flower", Some(items::mailbag::SeaFlower)),
            ("Exotic Flower", Some(items::mailbag::ExoticFlower)),
            ("Hero's Flag", Some(items::mailbag::HerosFlag)),
            ("Big Catch Flag", Some(items::mailbag::BigCatchFlag)),
            ("Big Sale Flag", Some(items::mailbag::BigSaleFlag)),
            ("Pinwheel", Some(items::mailbag::Pinwheel)),
            ("Sickle Moon Flag", Some(items::mailbag::SickleMoonFlag)),
            ("Skull Tower Idol", Some(items::mailbag::SkullTowerIdol)),
            ("Fountain Idol", Some(items::mailbag::FountainIdol)),
            ("Postman Statue", Some(items::mailbag::PostmanStatue)),
            ("Shop Guru Statue", Some(items::mailbag::ShopGuruStatue)),
            ("Father's Letter", Some(items::mailbag::FathersLetter)),
            ("Note to Mom", Some(items::mailbag::NoteToMom)),
            ("Maggie's Letter", Some(items::mailbag::MaggiesLetter)),
            ("Moblin's Letter", Some(items::mailbag::MoblinsLetter)),
            ("Cabana Deed", Some(items::mailbag::CabanaDeed)),
            ("Complimentary ID", Some(items::mailbag::ComplimentaryId)),
            ("Fill-Up Coupon", Some(items::mailbag::FillUpCoupon)),
        ];

        let spoof_items = vec![
            ("None", None),
            ("Bombs", Some(items::Item::Bombs)),
            ("Boomerang", Some(items::Item::Boomerang)),
            ("Deku Leaf", Some(items::Item::DekuLeaf)),
            ("Deluxe Box", Some(items::Item::DeluxeBox)),
            ("Grappling Hook", Some(items::Item::GrapplingHook)),
            ("Hero's Bow", Some(items::Item::HeroBow)),
            ("Hookshot", Some(items::Item::Hookshot)),
            ("Iron Boots", Some(items::Item::IronBoots)),
            ("Magic Armor", Some(items::Item::MagicArmor)),
            ("Picto Box", Some(items::Item::PictoBox)),
            ("Skull Hammer", Some(items::Item::SkullHammer)),
            ("Telescope", Some(items::Item::Telescope)),
            ("Tingle Bottle", Some(items::Item::TingleBottle)),
            ("Wind Waker", Some(items::Item::WindWaker)),
        ];

        Self {
            active: false,
            controller: None,
            frame_advance: Rc::clone(&fa),
            state: Rc::clone(&state),
            overlay: Overlay::new(Menu::new(
                "Root",
                vec![
                    Menu::new(
                        "Memfile",
                        vec![
                            Menu::new(
                                "File 1",
                                vec![
                                    Button::new("Save", {
                                        let state = Rc::clone(&state);
                                        move || {
                                            state.borrow_mut().memfiles.file1 =
                                                Some(memfile::Memfile::save());
                                        }
                                    }),
                                    Button::new("Load", {
                                        let state = Rc::clone(&state);
                                        move || {
                                            if let Some(file) = state.borrow().memfiles.file1 {
                                                file.load();
                                            }
                                        }
                                    }),
                                ],
                            ),
                            Menu::new(
                                "File 2",
                                vec![
                                    Button::new("Save", {
                                        let state = Rc::clone(&state);
                                        move || {
                                            state.borrow_mut().memfiles.file2 =
                                                Some(memfile::Memfile::save());
                                        }
                                    }),
                                    Button::new("Load", {
                                        let state = Rc::clone(&state);
                                        move || {
                                            if let Some(file) = state.borrow().memfiles.file2 {
                                                file.load();
                                            }
                                        }
                                    }),
                                ],
                            ),
                            Menu::new(
                                "File 3",
                                vec![
                                    Button::new("Save", {
                                        let state = Rc::clone(&state);
                                        move || {
                                            state.borrow_mut().memfiles.file3 =
                                                Some(memfile::Memfile::save());
                                        }
                                    }),
                                    Button::new("Load", {
                                        let state = Rc::clone(&state);
                                        move || {
                                            if let Some(file) = state.borrow().memfiles.file3 {
                                                file.load();
                                            }
                                        }
                                    }),
                                ],
                            ),
                        ],
                    ),
                    Toggle::new("FrameAdvance", false, {
                        let fa = Rc::clone(&fa);
                        move |v| {
                            fa.borrow_mut().enable(v);
                        }
                    }),
                    Menu::new(
                        "Cheats",
                        vec![
                            Toggle::new("Infinite Health", false, {
                                let state = Rc::clone(&state);
                                move |v| {
                                    state.borrow_mut().cheats.health = v;
                                }
                            }),
                            Toggle::new("Infinite Magic", false, {
                                let state = Rc::clone(&state);
                                move |v| {
                                    state.borrow_mut().cheats.magic = v;
                                }
                            }),
                            Toggle::new("Infinite Rupees", false, {
                                let state = Rc::clone(&state);
                                move |v| {
                                    state.borrow_mut().cheats.rupees = v;
                                }
                            }),
                            Toggle::new("Infinite Arrows", false, {
                                let state = Rc::clone(&state);
                                move |v| {
                                    state.borrow_mut().cheats.arrows = v;
                                }
                            }),
                            Toggle::new("Infinite Bombs", false, {
                                let state = Rc::clone(&state);
                                move |v| {
                                    state.borrow_mut().cheats.bombs = v;
                                }
                            }),
                            Toggle::new("Infinite Air", false, {
                                let state = Rc::clone(&state);
                                move |v| {
                                    state.borrow_mut().cheats.air = v;
                                }
                            }),
                            Toggle::new("Complete Map", false, {
                                let state = Rc::clone(&state);
                                move |v| {
                                    let mut state = state.borrow_mut();

                                    if v {
                                        state.cheats.sea_charts = Some(player::sea_charts::read());
                                        player::sea_charts::write([3; 49]);
                                    } else {
                                        if let Some(map) = state.cheats.sea_charts.take() {
                                            player::sea_charts::write(map);
                                        }
                                    }
                                }
                            }),
                            Toggle::new("Super Swim", false, {
                                let state = Rc::clone(&state);
                                move |v| {
                                    if !v {
                                        player::super_swim::enable(false);
                                    }
                                    state.borrow_mut().cheats.super_swim = v;
                                }
                            }),
                            Toggle::new("Super Crouch", false, {
                                let state = Rc::clone(&state);
                                move |v| {
                                    if !v {
                                        player::super_crouch::enable(false);
                                    }
                                    state.borrow_mut().cheats.super_crouch = v;
                                }
                            }),
                            Button::new("Hover", {
                                let state = Rc::clone(&state);
                                move || {
                                    state.borrow_mut().cheats.hover = true;
                                }
                            }),
                        ],
                    ),
                    Menu::new(
                        "Link Tweaks",
                        vec![
                            Number::new("Health", 1, 1, 0, 80, |v| player::health::write(*v)),
                            Number::new("Containers", 1, 1, 1, 20, |v| {
                                // core::ptr::write(player::CONTAINERS, *v * 4);
                                player::containers::write(*v * 4);
                            }),
                            Number::new("Magic", 0, 1, 0, 32, |v| {
                                // core::ptr::write(player::MAGIC, *v);
                                player::magic::write(*v);
                            }),
                            Select::new(
                                "Max Magic",
                                vec![("No Magic", 0), ("Normal Magic", 16), ("Double Magic", 32)],
                                |_, v| {
                                    // core::ptr::write(player::MAX_MAGIC, v.value);
                                    player::max_magic::write(v.value);
                                },
                            ),
                            Number::new("Rupees", 50, 10, 0, 5000, |v| {
                                // core::ptr::write(player::RUPEES, *v);
                                player::rupees::write(*v);
                            }),
                            Number::new("Arrows", 30, 10, 0, 99, |v| {
                                // core::ptr::write(player::ARROWS, *v);
                                player::arrows::write(*v);
                            }),
                            Number::new("Max Arrows", 30, 10, 0, 99, |v| {
                                // core::ptr::write(player::MAX_ARROWS, *v);
                                player::max_arrows::write(*v);
                            }),
                            Number::new("Bombs", 30, 10, 0, 99, |v| {
                                // core::ptr::write(player::BOMBS, *v);
                                player::bombs::write(*v);
                            }),
                            Number::new("Max Bombs", 30, 10, 0, 99, |v| {
                                // core::ptr::write(player::MAX_BOMBS, *v);
                                player::max_bombs::write(*v);
                            }),
                        ],
                    ),
                    Menu::new(
                        "Items",
                        vec![
                            Toggle::new("Bait Bag", false, |v| {
                                items::bait_bag::enable(v);
                            }),
                            Toggle::new("Bombs", false, |v| {
                                items::bombs::enable(v);
                            }),
                            Toggle::new("Boomerang", false, |v| {
                                items::boomerang::enable(v);
                            }),
                            Toggle::new("Deku Leaf", false, |v| {
                                items::deku_leaf::enable(v);
                            }),
                            Toggle::new("Delivery Bag", false, |v| {
                                items::delivery_bag::enable(v);
                            }),
                            Select::new(
                                "Picto Box",
                                vec![
                                    ("None", None),
                                    ("Normal", Some(items::picto_box::Normal)),
                                    ("Deluxe", Some(items::picto_box::Deluxe)),
                                ],
                                |_, v| {
                                    items::picto_box::set(v.value);
                                },
                            ),
                            Toggle::new("Grappling Hook", false, |v| {
                                items::grappling_hook::enable(v);
                            }),
                            Select::new(
                                "Hero's Bow",
                                vec![
                                    ("None", None),
                                    ("Normal", Some(items::bow::Hero)),
                                    ("Fire & Ice", Some(items::bow::Elemental)),
                                    ("Light", Some(items::bow::Magical)),
                                ],
                                |_, v| {
                                    items::bow::set(v.value);
                                },
                            ),
                            Toggle::new("Hero's Charm", false, |v| {
                                items::hero_charm::enable(v);
                            }),
                            Select::new(
                                "Hero's Shield",
                                vec![
                                    ("None", None),
                                    ("Normal", Some(items::shield::Hero)),
                                    ("Mirror", Some(items::shield::Mirror)),
                                ],
                                |_, v| {
                                    items::shield::set(v.value);
                                },
                            ),
                            Select::new(
                                "Hero's Sword",
                                vec![
                                    ("None", None),
                                    ("Normal", Some(items::sword::Hero)),
                                    ("Master 1", Some(items::sword::Master1)),
                                    ("Master 2", Some(items::sword::Master2)),
                                    ("Master 3", Some(items::sword::Master3)),
                                ],
                                |_, v| {
                                    items::sword::set(v.value);
                                },
                            ),
                            Toggle::new("Hookshot", false, |v| {
                                items::hookshot::enable(v);
                            }),
                            Toggle::new("Iron Boots", false, |v| {
                                items::iron_boots::enable(v);
                            }),
                            Toggle::new("Magic Armor", false, |v| {
                                items::magic_armor::enable(v);
                            }),
                            Toggle::new("Power Bracelets", false, |v| {
                                items::power_bracelets::enable(v);
                            }),
                            Toggle::new("Skull Hammer", false, |v| {
                                items::skull_hammer::enable(v);
                            }),
                            Toggle::new("Spoils Bag", false, |v| {
                                items::spoils_bag::enable(v);
                            }),
                            Toggle::new("Telescope", false, |v| {
                                items::telescope::enable(v);
                            }),
                            Toggle::new("Tingle Bottle", false, |v| {
                                items::tingle_bottle::enable(v);
                            }),
                            Toggle::new("Wind Waker", false, |v| {
                                items::wind_waker::enable(v);
                            }),
                            Select::new(
                                "Sail",
                                vec![
                                    ("None", None),
                                    ("Normal", Some(items::sail::Normal)),
                                    ("Swift", Some(items::sail::Swift)),
                                ],
                                |_, v| {
                                    items::sail::set(v.value);
                                },
                            ),
                            Menu::new(
                                "Bottles",
                                vec![
                                    Select::new("Bottle 1", bottle_options.clone(), |_, v| {
                                        items::bottles::set(items::bottles::Bottle1, v.value);
                                    }),
                                    Select::new("Bottle 2", bottle_options.clone(), |_, v| {
                                        items::bottles::set(items::bottles::Bottle2, v.value);
                                    }),
                                    Select::new("Bottle 3", bottle_options.clone(), |_, v| {
                                        items::bottles::set(items::bottles::Bottle3, v.value);
                                    }),
                                    Select::new("Bottle 4", bottle_options.clone(), |_, v| {
                                        items::bottles::set(items::bottles::Bottle4, v.value);
                                    }),
                                ],
                            ),
                            Menu::new(
                                "Songs",
                                vec![
                                    Toggle::new("Wind's Requiem", false, |v| {
                                        items::songs::enable(v, items::songs::WindRequiem);
                                    }),
                                    Toggle::new("Ballad of Gales", false, |v| {
                                        items::songs::enable(v, items::songs::BalladOfGales);
                                    }),
                                    Toggle::new("Command Melody", false, |v| {
                                        items::songs::enable(v, items::songs::CommandMelody);
                                    }),
                                    Toggle::new("Earth God's Lyrics", false, |v| {
                                        items::songs::enable(v, items::songs::EarthGodsLyrics);
                                    }),
                                    Toggle::new("Wind God's Aria", false, |v| {
                                        items::songs::enable(v, items::songs::WindGodsAria);
                                    }),
                                    Toggle::new("Song of Passing", false, |v| {
                                        items::songs::enable(v, items::songs::SongOfPassing);
                                    }),
                                ],
                            ),
                            Number::new("Triforce", 0u8, 1, 0, 8, |v| {
                                items::triforce::set(*v);
                            }),
                            Menu::new(
                                "Pearls",
                                vec![
                                    Toggle::new("Nayru's Pearl", false, |v| {
                                        items::pearls::enable(v, items::pearls::Nayru);
                                    }),
                                    Toggle::new("Din's Pearl", false, |v| {
                                        items::pearls::enable(v, items::pearls::Din);
                                    }),
                                    Toggle::new("Farore's Pearl", false, |v| {
                                        items::pearls::enable(v, items::pearls::Farore);
                                    }),
                                ],
                            ),
                            Menu::new(
                                "Mailbag",
                                vec![
                                    Select::new("Item 1", mailbag_options.clone(), |_, v| {
                                        items::mailbag::set(items::mailbag::Slot1, v.value);
                                    }),
                                    Select::new("Item 2", mailbag_options.clone(), |_, v| {
                                        items::mailbag::set(items::mailbag::Slot2, v.value);
                                    }),
                                    Select::new("Item 3", mailbag_options.clone(), |_, v| {
                                        items::mailbag::set(items::mailbag::Slot3, v.value);
                                    }),
                                    Select::new("Item 4", mailbag_options.clone(), |_, v| {
                                        items::mailbag::set(items::mailbag::Slot4, v.value);
                                    }),
                                    Select::new("Item 5", mailbag_options.clone(), |_, v| {
                                        items::mailbag::set(items::mailbag::Slot5, v.value);
                                    }),
                                    Select::new("Item 6", mailbag_options.clone(), |_, v| {
                                        items::mailbag::set(items::mailbag::Slot6, v.value);
                                    }),
                                    Select::new("Item 7", mailbag_options.clone(), |_, v| {
                                        items::mailbag::set(items::mailbag::Slot7, v.value);
                                    }),
                                    Select::new("Item 8", mailbag_options.clone(), |_, v| {
                                        items::mailbag::set(items::mailbag::Slot8, v.value);
                                    }),
                                ],
                            ),
                            Menu::new(
                                "Dungeon",
                                vec![
                                    Number::new("Dungeon Keys", 0, 1, 0, 10, |v| {
                                        items::dungeon_keys::set(*v);
                                    }),
                                    Toggle::new("Map", false, |v| {
                                        items::dungeon_items::enable(v, items::dungeon_items::Map);
                                    }),
                                    Toggle::new("Compass", false, |v| {
                                        items::dungeon_items::enable(
                                            v,
                                            items::dungeon_items::Compass,
                                        );
                                    }),
                                    Toggle::new("Boss Key", false, |v| {
                                        items::dungeon_items::enable(
                                            v,
                                            items::dungeon_items::BossKey,
                                        );
                                    }),
                                ],
                            ),
                            Menu::new("Spoils", {
                                let value = 1;
                                let inc = 10;
                                let min = 0;
                                let max = 99;
                                vec![
                                    Number::new("Red Chu Jelly", value, inc, min, max, |v| {
                                        items::spoils_bag::spoil(items::spoils_bag::RedJelly, *v);
                                    }),
                                    Number::new("Green Chu Jelly", value, inc, min, max, |v| {
                                        items::spoils_bag::spoil(items::spoils_bag::GreenJelly, *v);
                                    }),
                                    Number::new("Blue Chu Jelly", value, inc, min, max, |v| {
                                        items::spoils_bag::spoil(items::spoils_bag::BlueJelly, *v);
                                    }),
                                    Number::new("Joy Pendant", value, inc, min, max, |v| {
                                        items::spoils_bag::spoil(items::spoils_bag::JoyPendant, *v);
                                    }),
                                    Number::new("Boko Baba Seed", value, inc, min, max, |v| {
                                        items::spoils_bag::spoil(items::spoils_bag::BokoSeed, *v);
                                    }),
                                    Number::new("Golden Feather", value, inc, min, max, |v| {
                                        items::spoils_bag::spoil(
                                            items::spoils_bag::GoldenFeather,
                                            *v,
                                        );
                                    }),
                                    Number::new("Skull Necklace", value, inc, min, max, |v| {
                                        items::spoils_bag::spoil(
                                            items::spoils_bag::SkullNecklace,
                                            *v,
                                        );
                                    }),
                                    Number::new("Knight's Crest", value, inc, min, max, |v| {
                                        items::spoils_bag::spoil(
                                            items::spoils_bag::KnightsCrest,
                                            *v,
                                        );
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
                                spoof_items.clone(),
                                |_, v| {
                                    player::equipped_items::set(player::equipped_items::X, v.value);
                                },
                            ),
                            Select::new(
                                &format!("{}", icons::BTN_Y),
                                spoof_items.clone(),
                                |_, v| {
                                    player::equipped_items::set(player::equipped_items::Y, v.value);
                                },
                            ),
                            Select::new(
                                &format!("{}", icons::BTN_R),
                                spoof_items.clone(),
                                |_, v| {
                                    player::equipped_items::set(player::equipped_items::R, v.value);
                                },
                            ),
                        ],
                    ),
                    Menu::new(
                        "Coordinates",
                        vec![
                            Text::new(|| {
                                format!(
                                    "X: {:.2}, Y: {:.2}, Z: {:.2}",
                                    player::position::x::read(),
                                    player::position::y::read(),
                                    player::position::z::read()
                                )
                            }),
                            Toggle::new("Show Speed", false, {
                                let state = Rc::clone(&state);
                                move |v| {
                                    state.borrow_mut().speed_popup.show(v);
                                }
                            }),
                            Button::new("Store position", {
                                let state = Rc::clone(&state);
                                move || {
                                    state.borrow_mut().player_pos.store();
                                }
                            }),
                            Button::new("Restore position", {
                                let state = Rc::clone(&state);
                                move || {
                                    state.borrow_mut().player_pos.apply();
                                }
                            }),
                        ],
                    ),
                    Menu::new(
                        "Storage",
                        vec![
                            Toggle::new("Storage", false, |v| {
                                // misc::storage(v);
                                if v {
                                    player::storage::enable();
                                } else {
                                    player::storage::disable();
                                }
                            }),
                            Toggle::new("Chest Storage", false, |v| {
                                // misc::chest_storage(v);
                                if v {
                                    player::collision::enable(player::collision::ChestStorage);
                                } else {
                                    player::collision::disable(player::collision::ChestStorage);
                                }
                            }),
                            Toggle::new("Door Cancel", false, |v| {
                                // misc::door_cancel(v);
                                if v {
                                    player::collision::enable(player::collision::DoorCancel);
                                } else {
                                    player::collision::disable(player::collision::DoorCancel);
                                }
                            }),
                        ],
                    ),
                    Menu::new(
                        "Stage",
                        vec![
                            Text::new(|| {
                                format!(
                                    "Stage: {}, Spawn: {}, Room: {}, Layer: {}",
                                    stages::stage::get().name(),
                                    stages::spawn::read(),
                                    stages::room::read(),
                                    stages::layer::read()
                                )
                            }),
                            Select::new(
                                "Great Sea",
                                vec![
                                    locations::great_sea::FORSAKEN_FORTRESS,
                                    locations::great_sea::STAR_ISLAND,
                                    locations::great_sea::N_FAIRY_ISLAND,
                                    locations::great_sea::GALE_ISLAND,
                                    locations::great_sea::CRESCENT_MOON_ISLAND,
                                    locations::great_sea::SEVEN_STAR_ISLES,
                                    locations::great_sea::OVERLOOK_ISLAND,
                                    locations::great_sea::FOUR_EYE_REEF,
                                    locations::great_sea::MOTHER_CHILD_ISLE,
                                    locations::great_sea::SPECTACLE_ISLAND,
                                    locations::great_sea::WINDFALL_ISLAND,
                                    locations::great_sea::PAWPRINT_ISLE,
                                    locations::great_sea::DRAGON_ROOST_MT,
                                    locations::great_sea::FLIGHT_CONTROL_PLATFORM,
                                    locations::great_sea::W_FAIRY_ISLAND,
                                    locations::great_sea::ROCK_SPIRE_ISLE,
                                    locations::great_sea::TINGLE_ISLAND,
                                    locations::great_sea::N_TRIANGLE_ISLE,
                                    locations::great_sea::E_FAIRY_ISLE,
                                    locations::great_sea::FIRE_MOUNTAIN,
                                    locations::great_sea::STAR_BELT_ARCHIPELAGO,
                                    locations::great_sea::THREE_EYE_ISLE,
                                    locations::great_sea::GREATFISH_ISLE,
                                    locations::great_sea::CYCLOPS_REEF,
                                    locations::great_sea::SIX_EYE_REEF,
                                    locations::great_sea::TOWER_OF_GODS,
                                    locations::great_sea::E_TRIANGLE_ISLE,
                                    locations::great_sea::THORNED_FAIRY_ISLAND,
                                    locations::great_sea::NEEDLE_ROCK_ISLE,
                                    locations::great_sea::ISLET_OF_STEEL,
                                    locations::great_sea::STONEWATCHER_ISLAND,
                                    locations::great_sea::S_TRIANGLE_ISLE,
                                    locations::great_sea::LINKS_OASIS,
                                    locations::great_sea::BOMB_ISLAND,
                                    locations::great_sea::BIRDS_PEAK_ROCK,
                                    locations::great_sea::DIAMOND_STEPPE_ISLAND,
                                    locations::great_sea::FIVE_EYE_REEF,
                                    locations::great_sea::SHARK_ISLAND,
                                    locations::great_sea::S_FAIRY_ISLAND,
                                    locations::great_sea::ICE_RING_ISLE,
                                    locations::great_sea::FOREST_HAVEN,
                                    locations::great_sea::CLIFF_PLATEAU_ISLES,
                                    locations::great_sea::HORSESHOE_ISLE,
                                    locations::great_sea::OUTSET_ISLAND,
                                    locations::great_sea::HEADSTONE_ISLAND,
                                    locations::great_sea::TWO_EYE_REEF,
                                    locations::great_sea::ANGULAR_ISLES,
                                    locations::great_sea::BOATING_COURSE,
                                    locations::great_sea::FIVE_STAR_ISLES,
                                ],
                                |_, v| {
                                    locations::teleport(v.value);
                                },
                            ),
                            Select::new(
                                "Dungeon",
                                vec![
                                    locations::dungeons::FORSAKEN_FORTRESS,
                                    locations::dungeons::DRAGON_ROOST_CAVERN,
                                    locations::dungeons::FORBIDDEN_WOODS,
                                    locations::dungeons::TOWER_OF_GODS,
                                    locations::dungeons::EARTH_TEMPLE,
                                    locations::dungeons::WIND_TEMPLE,
                                    locations::dungeons::GANONS_TOWER,
                                ],
                                |_, v| {
                                    locations::teleport(v.value);
                                },
                            ),
                            Button::new("Reload stage", || {
                                stages::reload::activate();
                            }),
                            Select::new(
                                "Daytime",
                                vec![
                                    stages::daytime::Dawn,
                                    stages::daytime::Day,
                                    stages::daytime::Night,
                                ],
                                |_, v| {
                                    // core::ptr::write(stages::daytime::ADDRESS, v.value);
                                    stages::daytime::set(v.value);
                                },
                            ),
                            Select::new(
                                "Weather",
                                vec![
                                    stages::weather::Normal,
                                    stages::weather::Cloudy,
                                    stages::weather::Foggy,
                                ],
                                |_, v| {
                                    // core::ptr::write(stages::weather::ADDRESS, v.value);
                                    stages::weather::set(v.value);
                                },
                            ),
                        ],
                    ),
                    Menu::new(
                        "Macros",
                        vec![
                            Toggle::new("MSS", false, {
                                let state = Rc::clone(&state);
                                move |v| {
                                    state.borrow_mut().macros.mms.enabled = v;
                                }
                            }),
                            Toggle::new("Zombie Hover", false, {
                                let state = Rc::clone(&state);
                                move |v| {
                                    state.borrow_mut().macros.zombie_hover.enabled = v;
                                }
                            }),
                            Toggle::new("Record", false, {
                                let state = Rc::clone(&state);
                                move |v| {
                                    state.borrow_mut().recorder.start_recording(v);
                                }
                            }),
                            Button::new("Play", {
                                let state = Rc::clone(&state);
                                move || state.borrow_mut().recorder.start_playback()
                            }),
                        ],
                    ),
                ],
            )),
        }
    }

    fn on_deinit(&mut self) {
        wut::logger::deinit();
    }

    fn on_start(&mut self) {
        if is_windwaker() {
            self.active = true;

            self.frame_advance.borrow_mut().main = Some(wut::thread::current());

            let _ = notifications::info("WWHD Trainer started").show().unwrap();
        }
    }

    fn on_exit(&mut self) {
        self.active = false;
        self.frame_advance.borrow_mut().main.take();
    }
}

hook_on_input!(WWHDTrainer);
impl OnInput for WWHDTrainer {
    fn on_input(&mut self, port: gamepad::Port, input: gamepad::State) -> gamepad::State {
        if !self.active {
            return input;
        }

        let combo = {
            use gamepad::Button::{L, R};

            L | R
        };

        let mut input = input;

        if let Some(controller) = self.controller {
            if port == controller {
                if input.hold.contains(combo) {
                    self.overlay.control(input);
                    // important that I borrow after to avoid double mut borrow
                    let mut state = self.state.borrow_mut();

                    state.cheats.hover(input);

                    input.hold.clear();
                    input.trigger.clear();
                    input.release.clear();

                    state.macros.run(&mut input);
                } else {
                    self.controller = None;
                    self.overlay.hide();
                }
            }
        } else {
            if input.hold.contains(combo) {
                self.controller = Some(port);
                self.overlay.show();
            }
        }

        self.state.borrow_mut().recorder.run(port, &mut input);

        // if let Some(thread) = self.main_thread {
        //     if input.trigger.contains(gamepad::Button::Plus) {
        //         thread.park();

        //         wut::thread::sleep(wut::time::Duration::from_secs(3));

        //         thread.unpark();

        //         input.trigger.retain(|f| f != gamepad::Button::Plus);
        //         input.hold.retain(|f| f != gamepad::Button::Plus);
        //     }

        //     if input.trigger.contains(gamepad::Button::Minus) {
        //         thread.unpark();
        //     }
        // }

        input
    }
}

hook_on_update!(WWHDTrainer);
impl OnUpdate for WWHDTrainer {
    fn on_update(&mut self) {
        if !self.active {
            return;
        }

        // println!("{}", wut::time::DateTime::now());

        let mut state = self.state.borrow_mut();

        state.recorder.advance();
        state.cheats.run();
        state.speed_popup.update();

        self.overlay.render();

        self.frame_advance.borrow().wait();
    }
}
