#![allow(dead_code)]

use super::{flags, items, player, stages};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Status {
    pub health: u8,
    pub magic: u8,
    pub rupees: u16,
    pub arrows: u8,
    pub bombs: u8,
    pub containers: u8,
    pub max_magic: u8,
    pub max_arrows: u8,
    pub max_bombs: u8,
    pub equip_x: Option<items::Item>,
    pub equip_y: Option<items::Item>,
    pub equip_r: Option<items::Item>,
}

impl Status {
    pub fn read() -> Self {
        Self {
            health: player::health::read(),
            magic: player::magic::read(),
            rupees: player::rupees::read(),
            arrows: player::arrows::read(),
            bombs: player::bombs::read(),
            containers: player::containers::read(),
            max_magic: player::max_magic::read(),
            max_arrows: player::max_arrows::read(),
            max_bombs: player::max_bombs::read(),
            equip_x: player::equipped_items::get(player::equipped_items::X),
            equip_y: player::equipped_items::get(player::equipped_items::Y),
            equip_r: player::equipped_items::get(player::equipped_items::R),
        }
    }

    pub fn write(&self) {
        player::health::write(self.health);
        player::magic::write(self.magic);
        player::rupees::write(self.rupees);
        player::arrows::write(self.arrows);
        player::bombs::write(self.bombs);
        player::containers::write(self.containers);
        player::max_magic::write(self.max_magic);
        player::max_arrows::write(self.max_arrows);
        player::max_bombs::write(self.max_bombs);
        player::equipped_items::set(player::equipped_items::X, self.equip_x);
        player::equipped_items::set(player::equipped_items::Y, self.equip_y);
        player::equipped_items::set(player::equipped_items::R, self.equip_r);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Items {
    pub hero_charm: bool,
    pub telescope: bool,
    pub tingle_bottle: bool,
    pub wind_waker: bool,
    pub picto_box: Option<items::picto_box::Version>,
    pub spoils_bag: bool,
    pub grappling_hook: bool,
    pub bow: Option<items::bow::Version>,
    pub power_bracelets: bool,
    pub iron_boots: bool,
    pub magic_armor: bool,
    pub bait_bag: bool,
    pub boomerang: bool,
    pub hookshot: bool,
    pub delivery_bag: bool,
    pub bombs: bool,
    pub skull_hammer: bool,
    pub deku_leaf: bool,
    pub sword: Option<items::sword::Version>,
    pub shield: Option<items::shield::Version>,
}

impl Items {
    pub fn read() -> Self {
        Self {
            hero_charm: items::hero_charm::enabled(),
            telescope: items::telescope::enabled(),
            tingle_bottle: items::tingle_bottle::enabled(),
            wind_waker: items::wind_waker::enabled(),
            picto_box: items::picto_box::get(),
            spoils_bag: items::spoils_bag::enabled(),
            grappling_hook: items::grappling_hook::enabled(),
            bow: items::bow::get(),
            power_bracelets: items::power_bracelets::enabled(),
            iron_boots: items::iron_boots::enabled(),
            magic_armor: items::magic_armor::enabled(),
            bait_bag: items::bait_bag::enabled(),
            boomerang: items::boomerang::enabled(),
            hookshot: items::hookshot::enabled(),
            delivery_bag: items::delivery_bag::enabled(),
            bombs: items::bombs::enabled(),
            skull_hammer: items::skull_hammer::enabled(),
            deku_leaf: items::deku_leaf::enabled(),
            sword: items::sword::get(),
            shield: items::shield::get(),
        }
    }

    pub fn write(self) {
        items::hero_charm::enable(self.hero_charm);
        items::telescope::enable(self.telescope);
        items::tingle_bottle::enable(self.tingle_bottle);
        items::wind_waker::enable(self.wind_waker);
        items::picto_box::set(self.picto_box);
        items::spoils_bag::enable(self.spoils_bag);
        items::grappling_hook::enable(self.grappling_hook);
        items::bow::set(self.bow);
        items::power_bracelets::enable(self.power_bracelets);
        items::iron_boots::enable(self.iron_boots);
        items::magic_armor::enable(self.magic_armor);
        items::bait_bag::enable(self.bait_bag);
        items::boomerang::enable(self.boomerang);
        items::hookshot::enable(self.hookshot);
        items::delivery_bag::enable(self.delivery_bag);
        items::bombs::enable(self.bombs);
        items::skull_hammer::enable(self.skull_hammer);
        items::deku_leaf::enable(self.deku_leaf);
        items::sword::set(self.sword);
        items::shield::set(self.shield);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stage {
    pub id: stages::stage::Stage,
    pub room: u8,
    pub spawn: u8,
    pub layer: u8,
}

impl Stage {
    pub fn read() -> Self {
        Self {
            id: stages::stage::get(),
            room: stages::room::read(),
            spawn: stages::spawn::read(),
            layer: stages::layer::read(),
        }
    }

    pub fn write(&self) {
        stages::stage::set(self.id);
        stages::room::write(self.room);
        stages::spawn::write(self.spawn);
        stages::layer::write(self.layer);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub angle: u16,
}

impl Position {
    pub fn read() -> Self {
        Self {
            x: player::position::x::read(),
            y: player::position::y::read(),
            z: player::position::z::read(),
            angle: player::position::facing_angle::read(),
        }
    }

    pub fn write(&self) {
        let (x, y, z, a) = (self.x, self.y, self.z, self.angle);

        // Here we dispatch a thread because collsion need to be updated via a new frame before active
        let _ = wut::thread::spawn(move || {
            player::collision::enable(player::collision::DoorCancel);
            wut::thread::sleep(wut::time::Duration::from_millis(50));

            player::position::z::write(z);
            player::position::x::write(x);
            player::position::y::write(y);
            player::position::facing_angle::write(a);

            wut::thread::sleep(wut::time::Duration::from_millis(50));
            player::collision::disable(player::collision::DoorCancel);
        })
        .unwrap();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Flags {
    pub global: [u8; 0x40],
    pub scene: [u8; 0x40],
    pub great_sea: [u8; 0x24],
    pub forsaken_fortress: [u8; 0x24],
    pub dragon_roost_cavern: [u8; 0x28],
    pub forbidden_woods: [u8; 0x20],
    pub tower_of_the_gods: [u8; 0x24],
    pub earth_temple: [u8; 0x28],
    pub wind_temple: [u8; 0x20],
    pub ganons_tower: [u8; 0x24],
    pub hyrule: [u8; 0x24],
    pub interior: [u8; 0x24],
}

impl Flags {
    pub fn read() -> Self {
        Self {
            global: flags::global::read(),
            scene: flags::scene::read(),
            great_sea: flags::great_sea::read(),
            forsaken_fortress: flags::forsaken_fortress::read(),
            dragon_roost_cavern: flags::dragon_roost_cavern::read(),
            forbidden_woods: flags::forbidden_woods::read(),
            tower_of_the_gods: flags::tower_of_the_gods::read(),
            earth_temple: flags::earth_temple::read(),
            wind_temple: flags::wind_temple::read(),
            ganons_tower: flags::ganons_tower::read(),
            hyrule: flags::hyrule::read(),
            interior: flags::interior::read(),
        }
    }

    pub fn write(&self) {
        flags::global::write(self.global);
        flags::scene::write(self.scene);
        flags::great_sea::write(self.great_sea);
        flags::forsaken_fortress::write(self.forsaken_fortress);
        flags::dragon_roost_cavern::write(self.dragon_roost_cavern);
        flags::forbidden_woods::write(self.forbidden_woods);
        flags::tower_of_the_gods::write(self.tower_of_the_gods);
        flags::earth_temple::write(self.earth_temple);
        flags::wind_temple::write(self.wind_temple);
        flags::ganons_tower::write(self.ganons_tower);
        flags::hyrule::write(self.hyrule);
        flags::interior::write(self.interior);
    }
}

/// Memfile
///
/// Savestate-like bevaior by storing and restoring important parts of player information. In contrast to full savestates, this does not (re)store the internal state of the console.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Memfile {
    pub status: Status,
    pub items: Items,
    pub stage: Stage,
    pub position: Position,
    pub flags: Flags,
}

impl Memfile {
    /// Save from current memory into memfile
    pub fn save() -> Self {
        Self {
            status: Status::read(),
            items: Items::read(),
            stage: Stage::read(),
            position: Position::read(),
            flags: Flags::read(),
        }
    }

    /// Load from memfile into current memory
    pub fn load(&self) {
        self.status.write();
        self.items.write();
        self.stage.write();
        self.flags.write();
        stages::reload::activate();

        let position = self.position;
        wut::thread::spawn(move || {
            wut::thread::sleep(wut::time::Duration::from_secs(3));
            position.write();
        })
        .unwrap();
    }

    // /// Write to persistent storage on the SD card
    // pub fn persist(&self) {
    //     unimplemented!()
    // }
}
