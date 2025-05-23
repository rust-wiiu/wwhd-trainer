#![allow(dead_code)]

use core::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Location {
    pub name: &'static str,
    pub stage: crate::stages::stage::Stage,
    pub spawn: u8,
    pub room: u8,
    pub layer: u8,
}

impl Display for Location {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn teleport(location: Location) {
    use crate::stages::{layer, reload, room, spawn, stage};

    stage::set(location.stage);
    spawn::set(location.spawn);
    room::set(location.room);
    layer::set(location.layer);
    reload::activate();
}

pub mod great_sea {
    use super::Location;
    use crate::stages::stage::Stage;

    const SPAWN: u8 = 0;
    const LAYER: u8 = 0xff;

    pub const FORSAKEN_FORTRESS: Location = Location {
        name: "Forsaken Fortress",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 1,
        layer: LAYER,
    };

    pub const STAR_ISLAND: Location = Location {
        name: "Star Island",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 2,
        layer: LAYER,
    };

    pub const N_FAIRY_ISLAND: Location = Location {
        name: "N. Fairy Island",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 3,
        layer: LAYER,
    };

    pub const GALE_ISLAND: Location = Location {
        name: "Gale Island",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 4,
        layer: LAYER,
    };

    pub const CRESCENT_MOON_ISLAND: Location = Location {
        name: "Crescent Moon Island",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 5,
        layer: LAYER,
    };

    pub const SEVEN_STAR_ISLES: Location = Location {
        name: "Seven-Star Isles",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 6,
        layer: LAYER,
    };

    pub const OVERLOOK_ISLAND: Location = Location {
        name: "Overlook Island",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 7,
        layer: LAYER,
    };

    pub const FOUR_EYE_REEF: Location = Location {
        name: "Four-Eye Reef",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 8,
        layer: LAYER,
    };

    pub const MOTHER_CHILD_ISLE: Location = Location {
        name: "Mother & Child Isle",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 9,
        layer: LAYER,
    };

    pub const SPECTACLE_ISLAND: Location = Location {
        name: "Spectacle Island",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 10,
        layer: LAYER,
    };

    pub const WINDFALL_ISLAND: Location = Location {
        name: "Windfall Island",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 11,
        layer: LAYER,
    };

    pub const PAWPRINT_ISLE: Location = Location {
        name: "Pawprint Isle",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 12,
        layer: LAYER,
    };

    pub const DRAGON_ROOST_MT: Location = Location {
        name: "Dragon Roost Mt",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 13,
        layer: LAYER,
    };

    pub const FLIGHT_CONTROL_PLATFORM: Location = Location {
        name: "Flight Control Platform",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 14,
        layer: LAYER,
    };

    pub const W_FAIRY_ISLAND: Location = Location {
        name: "W. Fairy Island",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 15,
        layer: LAYER,
    };

    pub const ROCK_SPIRE_ISLE: Location = Location {
        name: "Rock Spire Isle",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 16,
        layer: LAYER,
    };

    pub const TINGLE_ISLAND: Location = Location {
        name: "Tingle Island",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 17,
        layer: LAYER,
    };

    pub const N_TRIANGLE_ISLE: Location = Location {
        name: "N. Triangle Isle",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 18,
        layer: LAYER,
    };

    pub const E_FAIRY_ISLE: Location = Location {
        name: "E. Fairy Isle",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 19,
        layer: LAYER,
    };

    pub const FIRE_MOUNTAIN: Location = Location {
        name: "Fire Mountain",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 20,
        layer: LAYER,
    };

    pub const STAR_BELT_ARCHIPELAGO: Location = Location {
        name: "Star Belt Archipelago",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 21,
        layer: LAYER,
    };

    pub const THREE_EYE_ISLE: Location = Location {
        name: "Three-Eye Isle",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 22,
        layer: LAYER,
    };

    pub const GREATFISH_ISLE: Location = Location {
        name: "Greatfish Isle",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 23,
        layer: LAYER,
    };

    pub const CYCLOPS_REEF: Location = Location {
        name: "Cyclops Reef",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 24,
        layer: LAYER,
    };

    pub const SIX_EYE_REEF: Location = Location {
        name: "Six-Eye Reef",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 25,
        layer: LAYER,
    };

    pub const TOWER_OF_GODS: Location = Location {
        name: "Tower of Gods",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 26,
        layer: LAYER,
    };

    pub const E_TRIANGLE_ISLE: Location = Location {
        name: "E. Triangle Isle",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 27,
        layer: LAYER,
    };

    pub const THORNED_FAIRY_ISLAND: Location = Location {
        name: "Thorned Fairy Island",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 28,
        layer: LAYER,
    };

    pub const NEEDLE_ROCK_ISLE: Location = Location {
        name: "Neele Rock Isle",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 29,
        layer: LAYER,
    };

    pub const ISLET_OF_STEEL: Location = Location {
        name: "Islet of Steel",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 30,
        layer: LAYER,
    };

    pub const STONEWATCHER_ISLAND: Location = Location {
        name: "Stonewatcher Island",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 31,
        layer: LAYER,
    };

    pub const S_TRIANGLE_ISLE: Location = Location {
        name: "S. Triangle Isle",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 32,
        layer: LAYER,
    };

    pub const LINKS_OASIS: Location = Location {
        name: "Links Oasis",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 33,
        layer: LAYER,
    };

    pub const BOMB_ISLAND: Location = Location {
        name: "Bomb Island",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 34,
        layer: LAYER,
    };

    pub const BIRDS_PEAK_ROCK: Location = Location {
        name: "Bird's Peak Rock",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 35,
        layer: LAYER,
    };

    pub const DIAMOND_STEPPE_ISLAND: Location = Location {
        name: "Diamond Steppe Island",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 36,
        layer: LAYER,
    };

    pub const FIVE_EYE_REEF: Location = Location {
        name: "Five-Eye Reef",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 37,
        layer: LAYER,
    };

    pub const SHARK_ISLAND: Location = Location {
        name: "Shark Island",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 38,
        layer: LAYER,
    };

    pub const S_FAIRY_ISLAND: Location = Location {
        name: "S. Fairy Island",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 39,
        layer: LAYER,
    };

    pub const ICE_RING_ISLE: Location = Location {
        name: "Ice Ring Isle",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 40,
        layer: LAYER,
    };

    pub const FOREST_HAVEN: Location = Location {
        name: "Forest Haven",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 41,
        layer: LAYER,
    };

    pub const CLIFF_PLATEAU_ISLES: Location = Location {
        name: "Cliff Plateau Isles",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 42,
        layer: LAYER,
    };

    pub const HORSESHOE_ISLE: Location = Location {
        name: "Horseshoe Isle",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 43,
        layer: LAYER,
    };

    pub const OUTSET_ISLAND: Location = Location {
        name: "Outset Island",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 44,
        layer: LAYER,
    };

    pub const HEADSTONE_ISLAND: Location = Location {
        name: "Headstone Island",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 45,
        layer: LAYER,
    };

    pub const TWO_EYE_REEF: Location = Location {
        name: "Two-Eye Reef",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 46,
        layer: LAYER,
    };

    pub const ANGULAR_ISLES: Location = Location {
        name: "Angular Isles",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 47,
        layer: LAYER,
    };

    pub const BOATING_COURSE: Location = Location {
        name: "Boating Course",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 48,
        layer: LAYER,
    };

    pub const FIVE_STAR_ISLES: Location = Location {
        name: "Five-Star Isles",
        stage: Stage::GreatSea,
        spawn: SPAWN,
        room: 49,
        layer: LAYER,
    };
}

pub mod dungeons {
    use super::Location;
    use crate::stages::stage::Stage;

    pub const FORSAKEN_FORTRESS: Location = Location {
        name: "Forsaken Fortress",
        stage: Stage::ForsakenFortressExterior,
        spawn: 2,
        room: 0,
        layer: 0xff,
    };

    pub const DRAGON_ROOST_CAVERN: Location = Location {
        name: "Dragon Roost Cavern",
        stage: Stage::DragonRoostCavern,
        spawn: 0,
        room: 0,
        layer: 0xff,
    };

    pub const FORBIDDEN_WOODS: Location = Location {
        name: "Forbidden Woods",
        stage: Stage::ForbiddenWoods,
        spawn: 0,
        room: 0,
        layer: 0xff,
    };

    pub const TOWER_OF_GODS: Location = Location {
        name: "Tower of the Gods",
        stage: Stage::TowerOfGods,
        spawn: 0,
        room: 0,
        layer: 0xff,
    };

    pub const EARTH_TEMPLE: Location = Location {
        name: "Earth Temple",
        stage: Stage::EarthTemple,
        spawn: 0,
        room: 0,
        layer: 0xff,
    };

    pub const WIND_TEMPLE: Location = Location {
        name: "Wind Temple",
        stage: Stage::WindTemple,
        spawn: 0,
        room: 0,
        layer: 0xff,
    };

    pub const GANONS_TOWER: Location = Location {
        name: "Ganon's Tower",
        stage: Stage::GanonsTowerEntrance,
        spawn: 0,
        room: 0,
        layer: 0xff,
    };
}
