#![no_std]
#![no_main]

use wut::font::icons;
use wut::prelude::*;
use wut::*;

use alloc::rc::Rc;
use core::cell::RefCell;
use notifications;
use overlay;
use wupf::{
    hook_on_input, hook_on_update, hook_plugin, Handler, OnInput, OnUpdate, Plugin, StaticHandler,
};
use wups::WUPS_PLUGIN_NAME;

mod items;
mod locations;
mod misc;
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

struct WWHDTrainer {
    active: bool,
    controller: Option<gamepad::Port>,
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
                player::health::set(80);
            }
            if self.magic {
                player::magic::set(32);
            }
            if self.rupees {
                player::rupees::set(5000);
            }
            if self.arrows {
                player::arrows::set(99);
            }
            if self.bombs {
                player::bombs::set(99);
            }
            if self.air {
                player::air::set(900);
            }
            if self.super_swim {
                player::super_swim::enable();
            }
            if self.super_crouch {
                player::super_crouch::enable();
            }
        }

        pub fn hover(&mut self, state: gamepad::State) {
            if !self.hover {
                return;
            }

            player::hover::activate();

            if let Some(stick) = &state.left_stick {
                player::position::speed::set(30.0 * stick.abs());
                if let Some(angle) = stick.angle() {
                    let facing_angle = player::position::facing_angle::get();
                    player::position::speed_angle::set(facing_angle + angle);
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
                        player::position::speed::get(),
                        player::position::facing_angle::get(),
                        player::position::speed_angle::get()
                    ))
                    .unwrap();
            }
        }
    }

    #[derive(Default)]
    pub struct PlayerPos {
        x: f32,
        y: f32,
        z: f32,
    }

    impl PlayerPos {
        pub fn store(&mut self) {
            self.x = player::position::x::get();
            self.y = player::position::y::get();
            self.z = player::position::z::get();
        }

        pub fn apply(&mut self) {
            player::position::z::set(self.z);
            player::position::x::set(self.x);
            player::position::y::set(self.y);
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
}

impl StaticHandler for WWHDTrainer {
    fn handler() -> &'static Handler<Self> {
        static HANDLER: Handler<WWHDTrainer> = Handler::new();
        &HANDLER
    }
}

hook_plugin!(WWHDTrainer);
impl Plugin for WWHDTrainer {
    fn on_init() -> Self {
        use overlay::*;

        let state = State::new();

        let bottle_options = vec![
            ("None", items::bottles::Content::None),
            ("Empty", items::bottles::Content::Empty),
            ("Red Elixir", items::bottles::Content::RedElixir),
            ("Green Elixir", items::bottles::Content::GreenElixir),
            ("Blue Elixir", items::bottles::Content::BlueElixir),
            ("Soup (Half)", items::bottles::Content::SoupHalf),
            ("Soup", items::bottles::Content::Soup),
            ("Water", items::bottles::Content::Water),
            ("Fairy", items::bottles::Content::Fairy),
            ("Pollen", items::bottles::Content::Pollen),
            ("Magic Water", items::bottles::Content::MagicWater),
        ];

        let mailbag_options = vec![
            ("None", items::mailbag::Content::None),
            ("Town Flower", items::mailbag::Content::TownFlower),
            ("Sea Flower", items::mailbag::Content::SeaFlower),
            ("Exotic Flower", items::mailbag::Content::ExoticFlower),
            ("Hero's Flag", items::mailbag::Content::HerosFlag),
            ("Big Catch Flag", items::mailbag::Content::BigCatchFlag),
            ("Big Sale Flag", items::mailbag::Content::BigSaleFlag),
            ("Pinwheel", items::mailbag::Content::Pinwheel),
            ("Sickle Moon Flag", items::mailbag::Content::SickleMoonFlag),
            ("Skull Tower Idol", items::mailbag::Content::SkullTowerIdol),
            ("Fountain Idol", items::mailbag::Content::FountainIdol),
            ("Postman Statue", items::mailbag::Content::PostmanStatue),
            ("Shop Guru Statue", items::mailbag::Content::ShopGuruStatue),
            ("Father's Letter", items::mailbag::Content::FathersLetter),
            ("Note to Mom", items::mailbag::Content::NoteToMom),
            ("Maggie's Letter", items::mailbag::Content::MaggiesLetter),
            ("Moblin's Letter", items::mailbag::Content::MoblinsLetter),
            ("Cabana Deed", items::mailbag::Content::CabanaDeed),
            ("Complimentary ID", items::mailbag::Content::ComplimentaryId),
            ("Fill-Up Coupon", items::mailbag::Content::FillUpCoupon),
        ];

        let spoof_items = vec![
            ("Bombs", items::Item::Bombs),
            ("Boomerang", items::Item::Boomerang),
            ("Deku Leaf", items::Item::DekuLeaf),
            ("Deluxe Box", items::Item::DeluxeBox),
            ("Grappling Hook", items::Item::GrapplingHook),
            ("Hero's Bow", items::Item::HeroBow),
            ("Hookshot", items::Item::Hookshot),
            ("Iron Boots", items::Item::IronBoots),
            ("Magic Armor", items::Item::MagicArmor),
            ("Picto Box", items::Item::PictoBox),
            ("Skull Hammer", items::Item::SkullHammer),
            ("Telescope", items::Item::Telescope),
            ("Tingle Bottle", items::Item::TingleBottle),
            ("Wind Waker", items::Item::WindWaker),
        ];

        Self {
            active: false,
            controller: None,
            state: Rc::clone(&state),
            overlay: Overlay::new(Menu::new(
                "Root",
                vec![
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
                                        state.cheats.sea_charts = Some(player::sea_charts::get());
                                        player::sea_charts::set([3; 49]);
                                    } else {
                                        if let Some(map) = state.cheats.sea_charts.take() {
                                            player::sea_charts::set(map);
                                        }
                                    }
                                }
                            }),
                            Toggle::new("Super Swim", false, {
                                let state = Rc::clone(&state);
                                move |v| {
                                    if !v {
                                        // {
                                        //     core::ptr::write(player::AIR as *mut u32, 800);
                                        // }
                                        player::super_swim::disable();
                                    }
                                    state.borrow_mut().cheats.super_swim = v;
                                }
                            }),
                            Toggle::new("Super Crouch", false, {
                                let state = Rc::clone(&state);
                                move |v| {
                                    // this code needs manual disabling
                                    if !v {
                                        // {
                                        //     core::ptr::write(player::SUPER_CROUCH, 0x4040_0000);
                                        // }
                                        player::super_crouch::disable();
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
                            Number::new("Health", 1, 1, 0, 80, |v| player::health::set(*v)),
                            Number::new("Containers", 1, 1, 1, 20, |v| {
                                // core::ptr::write(player::CONTAINERS, *v * 4);
                                player::containers::set(*v * 4);
                            }),
                            Number::new("Magic", 0, 1, 0, 32, |v| {
                                // core::ptr::write(player::MAGIC, *v);
                                player::magic::set(*v);
                            }),
                            Select::new(
                                "Max Magic",
                                vec![("No Magic", 0), ("Normal Magic", 16), ("Double Magic", 32)],
                                |_, v| {
                                    // core::ptr::write(player::MAX_MAGIC, v.value);
                                    player::max_magic::set(v.value);
                                },
                            ),
                            Number::new("Rupees", 50, 10, 0, 5000, |v| {
                                // core::ptr::write(player::RUPEES, *v);
                                player::rupees::set(*v);
                            }),
                            Number::new("Arrows", 30, 10, 0, 99, |v| {
                                // core::ptr::write(player::ARROWS, *v);
                                player::arrows::set(*v);
                            }),
                            Number::new("Max Arrows", 30, 10, 0, 99, |v| {
                                // core::ptr::write(player::MAX_ARROWS, *v);
                                player::max_arrows::set(*v);
                            }),
                            Number::new("Bombs", 30, 10, 0, 99, |v| {
                                // core::ptr::write(player::BOMBS, *v);
                                player::bombs::set(*v);
                            }),
                            Number::new("Max Bombs", 30, 10, 0, 99, |v| {
                                // core::ptr::write(player::MAX_BOMBS, *v);
                                player::max_bombs::set(*v);
                            }),
                        ],
                    ),
                    Menu::new(
                        "Items",
                        vec![
                            Toggle::new("Bait Bag", false, |v| {
                                // let v = if v { items::BAIT_BAG.value } else { 0 };
                                // core::ptr::write(items::BAIT_BAG.address, v);

                                if v {
                                    items::bait_bag::enable();
                                } else {
                                    items::bait_bag::disable();
                                }
                            }),
                            Toggle::new("Bombs", false, |v| {
                                // let v = if v { items::BOMBS.value } else { 0 };
                                // core::ptr::write(items::BOMBS.address, v);

                                if v {
                                    items::bombs::enable();
                                } else {
                                    items::bombs::disable();
                                }
                            }),
                            Toggle::new("Boomerang", false, |v| {
                                // let v = if v { items::BOOMERANG.value } else { 0 };
                                // core::ptr::write(items::BOOMERANG.address, v);
                                if v {
                                    items::boomerang::enable();
                                } else {
                                    items::boomerang::disable();
                                }
                            }),
                            Toggle::new("Deku Leaf", false, |v| {
                                // let v = if v { items::DEKU_LEAF.value } else { 0 };
                                // core::ptr::write(items::DEKU_LEAF.address, v);
                                if v {
                                    items::deku_leaf::enable();
                                } else {
                                    items::deku_leaf::disable();
                                }
                            }),
                            Toggle::new("Delivery Bag", false, |v| {
                                // let v = if v { items::DELIVERY_BAG.value } else { 0 };
                                // core::ptr::write(items::DELIVERY_BAG.address, v);
                                if v {
                                    items::delivery_bag::enable();
                                } else {
                                    items::delivery_bag::disable();
                                }
                            }),
                            Select::new(
                                "Picto Box",
                                vec![
                                    ("None", items::picto_box::Version::None),
                                    ("Normal", items::picto_box::Version::Normal),
                                    ("Deluxe", items::picto_box::Version::Delux),
                                ],
                                |_, v| {
                                    // core::ptr::write(items::PICTO_BOX.address, v.value);
                                    items::picto_box::enable(v.value);
                                },
                            ),
                            Toggle::new("Grappling Hook", false, |v| {
                                // let v = if v { items::GRAPPLING_HOOK.value } else { 0 };
                                // core::ptr::write(items::GRAPPLING_HOOK.address, v);
                                if v {
                                    items::grappling_hook::enable();
                                } else {
                                    items::grappling_hook::disable();
                                }
                            }),
                            Select::new(
                                "Hero's Bow",
                                vec![
                                    ("None", items::bow::Version::None),
                                    ("Normal", items::bow::Version::Hero),
                                    ("Fire & Ice", items::bow::Version::Elemental),
                                    ("Light", items::bow::Version::Magical),
                                ],
                                |_, v| {
                                    items::bow::enable(v.value);
                                },
                            ),
                            Toggle::new("Hero's Charm", false, |v| {
                                // let v = if v { items::HERO_CHARM.value } else { 0 };
                                // core::ptr::write(items::HERO_CHARM.address, v);
                                if v {
                                    items::hero_charm::enable();
                                } else {
                                    items::hero_charm::disable();
                                }
                            }),
                            Select::new(
                                "Hero's Shield",
                                vec![
                                    ("None", items::shield::Version::None),
                                    ("Normal", items::shield::Version::Hero),
                                    ("Mirror", items::shield::Version::Mirror),
                                ],
                                |_, v| {
                                    // core::ptr::write(items::HERO_SHIELD.address, v.value);
                                    items::shield::enable(v.value);
                                },
                            ),
                            Select::new(
                                "Hero's Sword",
                                vec![
                                    ("None", items::sword::Version::None),
                                    ("Normal", items::sword::Version::Hero),
                                    ("Master 1", items::sword::Version::Master1),
                                    ("Master 2", items::sword::Version::Master2),
                                    ("Master 3", items::sword::Version::Master3),
                                ],
                                |_, v| {
                                    // core::ptr::write(items::HERO_SWORD.address, v.value);
                                    items::sword::enable(v.value);
                                },
                            ),
                            Toggle::new("Hookshot", false, |v| {
                                // let v = if v { items::HOOKSHOT.value } else { 0 };
                                // core::ptr::write(items::HOOKSHOT.address, v);
                                if v {
                                    items::hookshot::enable();
                                } else {
                                    items::hookshot::disable();
                                }
                            }),
                            Toggle::new("Iron Boots", false, |v| {
                                // let v = if v { items::IRON_BOOTS.value } else { 0 };
                                // core::ptr::write(items::IRON_BOOTS.address, v);
                                if v {
                                    items::iron_boots::enable();
                                } else {
                                    items::iron_boots::disable();
                                }
                            }),
                            Toggle::new("Magic Armor", false, |v| {
                                // let v = if v { items::MAGIC_ARMOR.value } else { 0 };
                                // core::ptr::write(items::MAGIC_ARMOR.address, v);
                                if v {
                                    items::magic_armor::enable();
                                } else {
                                    items::magic_armor::disable();
                                }
                            }),
                            Toggle::new("Power Bracelets", false, |v| {
                                // let v1 = if v {
                                //     items::POWER_BRACELETS_1.value
                                // } else {
                                //     0xff
                                // };
                                // let v2 = if v { items::POWER_BRACELETS_2.value } else { 0 };

                                // core::ptr::write(items::POWER_BRACELETS_1.address, v1);
                                // core::ptr::write(items::POWER_BRACELETS_2.address, v2);

                                if v {
                                    items::power_bracelets::enable();
                                } else {
                                    items::power_bracelets::disable();
                                }
                            }),
                            Toggle::new("Skull Hammer", false, |v| {
                                // let v = if v { items::SKULL_HAMMER.value } else { 0 };
                                // core::ptr::write(items::SKULL_HAMMER.address, v);
                                if v {
                                    items::skull_hammer::enable();
                                } else {
                                    items::skull_hammer::disable();
                                }
                            }),
                            Toggle::new("Spoils Bag", false, |v| {
                                // let v = if v { items::SPOILS_BAG.value } else { 0 };
                                // core::ptr::write(items::SPOILS_BAG.address, v);
                                if v {
                                    items::spoils_bag::enable();
                                } else {
                                    items::spoils_bag::disable();
                                }
                            }),
                            Toggle::new("Telescope", false, |v| {
                                // let v = if v { items::TELESCOPE.value } else { 0 };
                                // core::ptr::write(items::TELESCOPE.address, v);
                                if v {
                                    items::telescope::enable();
                                } else {
                                    items::telescope::disable();
                                }
                            }),
                            Toggle::new("Tingle Bottle", false, |v| {
                                // let v = if v { items::TINGLE_BOTTLE.value } else { 0 };
                                // core::ptr::write(items::TINGLE_BOTTLE.address, v);
                                if v {
                                    items::tingle_bottle::enable();
                                } else {
                                    items::tingle_bottle::disable();
                                }
                            }),
                            Toggle::new("Wind Waker", false, |v| {
                                // let v = if v { items::WIND_WAKER.value } else { 0 };
                                // core::ptr::write(items::WIND_WAKER.address, v);
                                if v {
                                    items::wind_waker::enable();
                                } else {
                                    items::wind_waker::disable();
                                }
                            }),
                            Select::new(
                                "Sail",
                                vec![
                                    ("None", items::sail::Version::None),
                                    ("Normal", items::sail::Version::Normal),
                                    ("Swift", items::sail::Version::Swift),
                                ],
                                |_, v| {
                                    // core::ptr::write(items::NORMAL_SAIL.address, v.value);
                                    items::sail::enable(v.value);
                                },
                            ),
                            Menu::new(
                                "Bottles",
                                vec![
                                    Select::new("Bottle 1", bottle_options.clone(), |_, v| {
                                        // core::ptr::write(items::BOTTLE_1.address, v.value);
                                        items::bottles::enable(
                                            items::bottles::Slot::Bottle1,
                                            v.value,
                                        );
                                    }),
                                    Select::new("Bottle 2", bottle_options.clone(), |_, v| {
                                        // core::ptr::write(items::BOTTLE_2.address, v.value);
                                        items::bottles::enable(
                                            items::bottles::Slot::Bottle2,
                                            v.value,
                                        );
                                    }),
                                    Select::new("Bottle 3", bottle_options.clone(), |_, v| {
                                        // core::ptr::write(items::BOTTLE_3.address, v.value);
                                        items::bottles::enable(
                                            items::bottles::Slot::Bottle3,
                                            v.value,
                                        );
                                    }),
                                    Select::new("Bottle 4", bottle_options.clone(), |_, v| {
                                        // core::ptr::write(items::BOTTLE_4.address, v.value);
                                        items::bottles::enable(
                                            items::bottles::Slot::Bottle4,
                                            v.value,
                                        );
                                    }),
                                ],
                            ),
                            Menu::new(
                                "Songs",
                                vec![
                                    Toggle::new("Wind's Requiem", false, |v| {
                                        // let x = core::ptr::read(items::WINDS_REQUIEM.address);
                                        // let x = if v {
                                        //     x | items::WINDS_REQUIEM.value
                                        // } else {
                                        //     x & !items::WINDS_REQUIEM.value
                                        // };
                                        // core::ptr::write(items::WINDS_REQUIEM.address, x);
                                        if v {
                                            items::songs::enable(items::songs::Song::WindRequiem);
                                        } else {
                                            items::songs::disable(items::songs::Song::WindRequiem);
                                        }
                                    }),
                                    Toggle::new("Ballad of Gales", false, |v| {
                                        // let x = core::ptr::read(items::BALLAD_OF_GALES.address);
                                        // let x = if v {
                                        //     x | items::BALLAD_OF_GALES.value
                                        // } else {
                                        //     x & !items::BALLAD_OF_GALES.value
                                        // };
                                        // core::ptr::write(items::BALLAD_OF_GALES.address, x);

                                        if v {
                                            items::songs::enable(items::songs::Song::BalladOfGales);
                                        } else {
                                            items::songs::disable(
                                                items::songs::Song::BalladOfGales,
                                            );
                                        }
                                    }),
                                    Toggle::new("Command Melody", false, |v| {
                                        // let x = core::ptr::read(items::COMMAND_MELODY.address);
                                        // let x = if v {
                                        //     x | items::COMMAND_MELODY.value
                                        // } else {
                                        //     x & !items::COMMAND_MELODY.value
                                        // };
                                        // core::ptr::write(items::COMMAND_MELODY.address, x);
                                        if v {
                                            items::songs::enable(items::songs::Song::CommandMelody);
                                        } else {
                                            items::songs::disable(
                                                items::songs::Song::CommandMelody,
                                            );
                                        }
                                    }),
                                    Toggle::new("Earth God's Lyrics", false, |v| {
                                        // let x = core::ptr::read(items::EARTH_GODS_LYRICS.address);
                                        // let x = if v {
                                        //     x | items::EARTH_GODS_LYRICS.value
                                        // } else {
                                        //     x & !items::EARTH_GODS_LYRICS.value
                                        // };
                                        // core::ptr::write(items::EARTH_GODS_LYRICS.address, x);
                                        if v {
                                            items::songs::enable(
                                                items::songs::Song::EarthGodsLyrics,
                                            );
                                        } else {
                                            items::songs::disable(
                                                items::songs::Song::EarthGodsLyrics,
                                            );
                                        }
                                    }),
                                    Toggle::new("Wind God's Aria", false, |v| {
                                        // let x = core::ptr::read(items::WIND_GODS_ARIA.address);
                                        // let x = if v {
                                        //     x | items::WIND_GODS_ARIA.value
                                        // } else {
                                        //     x & !items::WIND_GODS_ARIA.value
                                        // };
                                        // core::ptr::write(items::WIND_GODS_ARIA.address, x);
                                        if v {
                                            items::songs::enable(items::songs::Song::WindGodsAria);
                                        } else {
                                            items::songs::disable(items::songs::Song::WindGodsAria);
                                        }
                                    }),
                                    Toggle::new("Song of Passing", false, |v| {
                                        // let x = core::ptr::read(items::SONG_OF_PASSING.address);
                                        // let x = if v {
                                        //     x | items::SONG_OF_PASSING.value
                                        // } else {
                                        //     x & !items::SONG_OF_PASSING.value
                                        // };
                                        // core::ptr::write(items::SONG_OF_PASSING.address, x);
                                        if v {
                                            items::songs::enable(items::songs::Song::SongOfPassing);
                                        } else {
                                            items::songs::disable(
                                                items::songs::Song::SongOfPassing,
                                            );
                                        }
                                    }),
                                ],
                            ),
                            Number::new("Triforce", 0u8, 1, 0, 8, |v| {
                                // let x = if *v == 8 { 0xff } else { (1 << *v) - 1 };
                                // core::ptr::write(items::TRIFORCE.address, x);
                                items::triforce::enable(*v);
                            }),
                            Menu::new(
                                "Pearls",
                                vec![
                                    Toggle::new("Nayru's Pearl", false, |v| {
                                        //     let x = core::ptr::read(items::NAYRUS_PEARL.address);
                                        //     let x = if v {
                                        //         x | items::NAYRUS_PEARL.value
                                        //     } else {
                                        //         x & !items::NAYRUS_PEARL.value
                                        //     };
                                        //     core::ptr::write(items::NAYRUS_PEARL.address, x);
                                        if v {
                                            items::pearls::enable(items::pearls::Pearl::Nayru);
                                        } else {
                                            items::pearls::disable(items::pearls::Pearl::Nayru);
                                        }
                                    }),
                                    Toggle::new("Din's Pearl", false, |v| {
                                        // let x = core::ptr::read(items::DINS_PEARL.address);
                                        // let x = if v {
                                        //     x | items::DINS_PEARL.value
                                        // } else {
                                        //     x & !items::DINS_PEARL.value
                                        // };
                                        // core::ptr::write(items::DINS_PEARL.address, x);
                                        if v {
                                            items::pearls::enable(items::pearls::Pearl::Din);
                                        } else {
                                            items::pearls::disable(items::pearls::Pearl::Din);
                                        }
                                    }),
                                    Toggle::new("Farore's Pearl", false, |v| {
                                        // let x = core::ptr::read(items::FARORES_PEARL.address);
                                        // let x = if v {
                                        //     x | items::FARORES_PEARL.value
                                        // } else {
                                        //     x & !items::FARORES_PEARL.value
                                        // };
                                        // core::ptr::write(items::FARORES_PEARL.address, x);
                                        if v {
                                            items::pearls::enable(items::pearls::Pearl::Farore);
                                        } else {
                                            items::pearls::disable(items::pearls::Pearl::Farore);
                                        }
                                    }),
                                ],
                            ),
                            Menu::new(
                                "Mailbag",
                                vec![
                                    Select::new("Item 1", mailbag_options.clone(), |_, v| {
                                        // core::ptr::write(items::MAILBAG_1.address, v.value);
                                        items::mailbag::enable(
                                            items::mailbag::Slot::Slot1,
                                            v.value,
                                        );
                                    }),
                                    Select::new("Item 2", mailbag_options.clone(), |_, v| {
                                        // core::ptr::write(items::MAILBAG_2.address, v.value);
                                        items::mailbag::enable(
                                            items::mailbag::Slot::Slot2,
                                            v.value,
                                        );
                                    }),
                                    Select::new("Item 3", mailbag_options.clone(), |_, v| {
                                        // core::ptr::write(items::MAILBAG_3.address, v.value);
                                        items::mailbag::enable(
                                            items::mailbag::Slot::Slot3,
                                            v.value,
                                        );
                                    }),
                                    Select::new("Item 4", mailbag_options.clone(), |_, v| {
                                        // core::ptr::write(items::MAILBAG_4.address, v.value);
                                        items::mailbag::enable(
                                            items::mailbag::Slot::Slot4,
                                            v.value,
                                        );
                                    }),
                                    Select::new("Item 5", mailbag_options.clone(), |_, v| {
                                        // core::ptr::write(items::MAILBAG_5.address, v.value);
                                        items::mailbag::enable(
                                            items::mailbag::Slot::Slot5,
                                            v.value,
                                        );
                                    }),
                                    Select::new("Item 6", mailbag_options.clone(), |_, v| {
                                        // core::ptr::write(items::MAILBAG_6.address, v.value);
                                        items::mailbag::enable(
                                            items::mailbag::Slot::Slot6,
                                            v.value,
                                        );
                                    }),
                                    Select::new("Item 7", mailbag_options.clone(), |_, v| {
                                        // core::ptr::write(items::MAILBAG_7.address, v.value);
                                        items::mailbag::enable(
                                            items::mailbag::Slot::Slot7,
                                            v.value,
                                        );
                                    }),
                                    Select::new("Item 8", mailbag_options.clone(), |_, v| {
                                        // core::ptr::write(items::MAILBAG_8.address, v.value);
                                        items::mailbag::enable(
                                            items::mailbag::Slot::Slot8,
                                            v.value,
                                        );
                                    }),
                                ],
                            ),
                            Menu::new(
                                "Dungeon",
                                vec![
                                    Number::new("Dungeon Keys", 0, 1, 0, 10, |v| {
                                        // core::ptr::write(items::DUNGEON_KEYS.address, *v);
                                        items::dungeon_keys::enable(*v);
                                    }),
                                    Toggle::new("Map", false, |v| {
                                        // let x = core::ptr::read(items::DUNGEON_MAP.address);
                                        // let x = if v {
                                        //     x | items::DUNGEON_MAP.value
                                        // } else {
                                        //     x & !items::DUNGEON_MAP.value
                                        // };
                                        // core::ptr::write(items::DUNGEON_MAP.address, x);
                                        if v {
                                            items::dungeon_items::enable(items::dungeon_items::Map);
                                        } else {
                                            items::dungeon_items::disable(
                                                items::dungeon_items::Map,
                                            );
                                        }
                                    }),
                                    Toggle::new("Compass", false, |v| {
                                        // let x = core::ptr::read(items::DUNGEON_COMPASS.address);
                                        // let x = if v {
                                        //     x | items::DUNGEON_COMPASS.value
                                        // } else {
                                        //     x & !items::DUNGEON_COMPASS.value
                                        // };
                                        // core::ptr::write(items::DUNGEON_COMPASS.address, x);
                                        if v {
                                            items::dungeon_items::enable(
                                                items::dungeon_items::Compass,
                                            );
                                        } else {
                                            items::dungeon_items::disable(
                                                items::dungeon_items::Compass,
                                            );
                                        }
                                    }),
                                    Toggle::new("Boss Key", false, |v| {
                                        // let x = core::ptr::read(items::DUNGEON_BOSS_KEY.address);
                                        // let x = if v {
                                        //     x | items::DUNGEON_BOSS_KEY.value
                                        // } else {
                                        //     x & !items::DUNGEON_BOSS_KEY.value
                                        // };
                                        // core::ptr::write(items::DUNGEON_BOSS_KEY.address, x);
                                        if v {
                                            items::dungeon_items::enable(
                                                items::dungeon_items::BossKey,
                                            );
                                        } else {
                                            items::dungeon_items::disable(
                                                items::dungeon_items::BossKey,
                                            );
                                        }
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
                                        // core::ptr::write(items::RED_JELLY.address, *v);
                                        items::spoils_bag::spoil(items::spoils_bag::RedJelly, *v);
                                    }),
                                    Number::new("Green Chu Jelly", value, inc, min, max, |v| {
                                        // core::ptr::write(items::GREEN_JELLY.address, *v);
                                        items::spoils_bag::spoil(items::spoils_bag::GreenJelly, *v);
                                    }),
                                    Number::new("Blue Chu Jelly", value, inc, min, max, |v| {
                                        // core::ptr::write(items::BLUE_JELLY.address, *v);
                                        items::spoils_bag::spoil(items::spoils_bag::BlueJelly, *v);
                                    }),
                                    Number::new("Joy Pendant", value, inc, min, max, |v| {
                                        // core::ptr::write(items::JOY_PENDANT.address, *v);
                                        items::spoils_bag::spoil(items::spoils_bag::JoyPendant, *v);
                                    }),
                                    Number::new("Boko Baba Seed", value, inc, min, max, |v| {
                                        // core::ptr::write(items::BOKO_SEEDS.address, *v);
                                        items::spoils_bag::spoil(items::spoils_bag::BokoSeed, *v);
                                    }),
                                    Number::new("Golden Feather", value, inc, min, max, |v| {
                                        // core::ptr::write(items::GOLDEN_FEATHERS.address, *v);
                                        items::spoils_bag::spoil(
                                            items::spoils_bag::GoldenFeather,
                                            *v,
                                        );
                                    }),
                                    Number::new("Skull Necklace", value, inc, min, max, |v| {
                                        // core::ptr::write(items::SKULL_NECKLACES.address, *v);
                                        items::spoils_bag::spoil(
                                            items::spoils_bag::SkullNecklace,
                                            *v,
                                        );
                                    }),
                                    Number::new("Knight's Crest", value, inc, min, max, |v| {
                                        // core::ptr::write(items::KNIGHT_CREST.address, *v);
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
                                    // core::ptr::write(player::BUTTON_X, v.value);
                                    player::equipped_items::set(player::equipped_items::X, v.value);
                                },
                            ),
                            Select::new(
                                &format!("{}", icons::BTN_Y),
                                spoof_items.clone(),
                                |_, v| {
                                    // core::ptr::write(player::BUTTON_Y, v.value);
                                    player::equipped_items::set(player::equipped_items::Y, v.value);
                                },
                            ),
                            Select::new(
                                &format!("{}", icons::BTN_R),
                                spoof_items.clone(),
                                |_, v| {
                                    // core::ptr::write(player::BUTTON_R, v.value);
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
                                    // core::ptr::read(player::position::X),
                                    // core::ptr::read(player::position::Y),
                                    // core::ptr::read(player::position::Z)
                                    player::position::x::get(),
                                    player::position::y::get(),
                                    player::position::z::get()
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
                                    misc::storage::enable();
                                } else {
                                    misc::storage::disable();
                                }
                            }),
                            Toggle::new("Chest Storage", false, |v| {
                                // misc::chest_storage(v);
                                if v {
                                    misc::collision::enable(misc::collision::ChestStorage);
                                } else {
                                    misc::collision::disable(misc::collision::ChestStorage);
                                }
                            }),
                            Toggle::new("Door Cancel", false, |v| {
                                // misc::door_cancel(v);
                                if v {
                                    misc::collision::enable(misc::collision::DoorCancel);
                                } else {
                                    misc::collision::disable(misc::collision::DoorCancel);
                                }
                            }),
                        ],
                    ),
                    Menu::new(
                        "Stage",
                        vec![
                            Text::new(|| {
                                // let stage = 0x109763f0 as *mut [u8; 8];
                                // let spawn = 0x109763f9 as *mut u8;
                                // let room = 0x109763fa as *mut u8;
                                // let layer = 0x109763fb as *mut u8;

                                format!(
                                    "Stage: {}, Spawn: {}, Room: {}, Layer: {}",
                                    // stages::value_to_name(core::ptr::read(stage)),
                                    // core::ptr::read(spawn),
                                    // core::ptr::read(room),
                                    // core::ptr::read(layer)
                                    stages::stage::get().name(),
                                    stages::spawn::get(),
                                    stages::room::get(),
                                    stages::layer::get()
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
                                    stages::daytime::enable(v.value);
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
                                    stages::weather::enable(v.value);
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

    fn on_deinit(&mut self) {}

    fn on_start(&mut self) {
        if is_windwaker() {
            self.active = true;

            let _ = notifications::info("WWHD Trainer started").show().unwrap();
        }
    }

    fn on_exit(&mut self) {
        self.active = false;
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

                    if state.macros.mms.enabled {
                        state.macros.mms.run_forever(&mut input);
                    }

                    if state.macros.zombie_hover.enabled {
                        state.macros.zombie_hover.run_forever(&mut input);
                    }
                } else {
                    self.controller = None;
                    self.overlay.hide();
                }

                // state.recorder.record(port, input);
            }
        } else {
            if input.hold.contains(combo) {
                self.controller = Some(port);
                self.overlay.show();
            }
        }

        self.state.borrow_mut().recorder.run(port, &mut input);

        input
    }
}

hook_on_update!(WWHDTrainer);
impl OnUpdate for WWHDTrainer {
    fn on_update(&mut self) {
        if !self.active {
            return;
        }

        let mut state = self.state.borrow_mut();

        state.recorder.advance();
        state.cheats.run();
        state.speed_popup.update();

        self.overlay.render();
    }
}
