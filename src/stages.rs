use wut::prelude::*;

/// Set to 1 to reload stage.
pub const RELOAD: *mut u8 = 0x109763fc as *mut u8;

pub const STAGE_ID: *mut [u8; 8] = 0x109763f0 as *mut [u8; 8];

pub const SPAWN_ID: *mut u8 = 0x109763f9 as *mut u8;

pub const ROOM_ID: *mut u8 = 0x109763fa as *mut u8;

pub const STAGE_LAYER: *mut u8 = 0x109763fb as *mut u8;

// region: Stages

pub struct Stage {
    pub name: &'static str,
    pub value: [u8; 8],
}

pub const OUTSET_FOREST: Stage = Stage {
    name: "Outset Island - Forest",
    value: *b"A_mori\0\0",
};

pub const INVISIBLE_ISLAND: Stage = Stage {
    name: "Invisible Island",
    value: *b"A_nami\0\0",
};

pub const PIRATE_SHIP_DECK: Stage = Stage {
    name: "Pirate Ship - Deck",
    value: *b"A_umikz\0",
};

pub const LINKS_OASIS: Stage = Stage {
    name: "Link's Oasis",
    value: *b"Abesso\0\0",
};

pub const SUBMARINE_INTERIOR: Stage = Stage {
    name: "Submarine Interior",
    value: *b"Abship\0\0",
};

pub const DRAGON_ROOST_ISLAND_SPRING: Stage = Stage {
    name: "Dragon Roost Island - Spring",
    value: *b"Adanmae\0",
};

pub const TOWER_OF_GODS_CUTSCENE: Stage = Stage {
    name: "Tower of Gods - Cutscene",
    value: *b"ADMumi\0\0",
};

pub const PIRATE_SHIP_INTERIOR: Stage = Stage {
    name: "Pirate Ship - Interior",
    value: *b"Asoko\0\0\0",
};

pub const DRAGON_ROOST_ISLAND_MAIL_CENTER: Stage = Stage {
    name: "Dragon Roost Island - Mail Center",
    value: *b"Atorizk\0",
};

pub const BOMB_ISLAND_CAVE: Stage = Stage {
    name: "Bomb Island - Cave",
    value: *b"Cave01\0\0",
};

pub const STAR_ISLAND_CAVE: Stage = Stage {
    name: "Star Island - Cave",
    value: *b"Cave02\0\0",
};

pub const CLIFF_PLATEAU_ISLES_CAVE: Stage = Stage {
    name: "Cliff Plateau Isles - Cave",
    value: *b"Cave03\0\0",
};

pub const ROCK_SPIRE_ISLE_CAVE: Stage = Stage {
    name: "Rock Spire Isle - Cave",
    value: *b"Cave04\0\0",
};

pub const HORSESHOE_ISLAND_CAVE: Stage = Stage {
    name: "Horseshoe Island - Cave",
    value: *b"Cave05\0\0",
};

pub const OUTSET_ISLAND_UNUSED_CAVE: Stage = Stage {
    name: "Outset Island - Unused Cave",
    value: *b"Cave06\0\0",
};

pub const PAWPRINT_ISLE_CAVE_1: Stage = Stage {
    name: "Pawprint Isle - Cave",
    value: *b"Cave07\0\0",
};

pub const SAVAGE_LABYRINTH_1: Stage = Stage {
    name: "Savage Labyrinth",
    value: *b"Cave09\0\0",
};

pub const SAVAGE_LABYRINTH_2: Stage = Stage {
    name: "Savage Labyrinth",
    value: *b"Cave10\0\0",
};

pub const SAVAGE_LABYRINTH_3: Stage = Stage {
    name: "Savage Labyrinth",
    value: *b"Cave11\0\0",
};

pub const DRAGON_ROOST_ISLAND_KOMALIS_ROOM: Stage = Stage {
    name: "Dragon Roost Island - Komali's Room",
    value: *b"Comori\0\0",
};

pub const OUTSET_ISLAND_TEST: Stage = Stage {
    name: "Outset Island - Test",
    value: *b"DmSpot0\0",
};

pub const EARTH_TEMPLE_ENTRANCE: Stage = Stage {
    name: "Earth Temple - Entrance",
    value: *b"Edaichi\0",
};

pub const WIND_TEMPLE_ENTRANCE: Stage = Stage {
    name: "Wind Temple - Entrance",
    value: *b"Ekaze\0\0\0",
};

pub const END_GAME_CUTSCENE: Stage = Stage {
    name: "End Game Cutscene",
    value: *b"ENDumi\0\0",
};

pub const NORTHERN_FAIRY_ISLAND_FAIRY: Stage = Stage {
    name: "Northern Fairy Island - Fairy",
    value: *b"Fairy01\0",
};

pub const EASTERN_FAIRY_ISLAND_FAIRY: Stage = Stage {
    name: "Eastern Fairy Island - Fairy",
    value: *b"Fairy02\0",
};

pub const WESTERN_FAIRY_ISLAND_FAIRY: Stage = Stage {
    name: "Western Fairy Island - Fairy",
    value: *b"Fairy03\0",
};

pub const OUTSET_ISLAND_FOREST_FAIRY: Stage = Stage {
    name: "Outset Island - Forest - Fairy",
    value: *b"Fairy04\0",
};

pub const THORNED_FAIRY_ISLAND_FAIRY: Stage = Stage {
    name: "Thorned Fairy Island - Fairy",
    value: *b"Fairy05\0",
};

pub const SOUTHERN_FAIRY_ISLAND_FAIRY: Stage = Stage {
    name: "Southern Fairy Island - Fairy",
    value: *b"Fairy06\0",
};

pub const GALLERY_GREAT_SEA: Stage = Stage {
    name: "Gallery - Great Sea Figurines",
    value: *b"figureA\0",
};

pub const GALLERY_WINDFALL_ISLAND: Stage = Stage {
    name: "Gallery - Windfall Island Figurines",
    value: *b"figureB\0",
};

pub const GALLERY_OUTSET_ISLAND: Stage = Stage {
    name: "Gallery - Outset Island Figurines",
    value: *b"figureC\0",
};

pub const GALLERY_BOSS: Stage = Stage {
    name: "Gallery - Boss Figurines",
    value: *b"figureD\0",
};

pub const GALLERY_ENEMY: Stage = Stage {
    name: "Gallery - Enemy Figurines",
    value: *b"figureE\0",
};

pub const GALLERY_DRC: Stage = Stage {
    name: "Gallery - DRC Figurines",
    value: *b"figureF\0",
};

pub const GALLERY_FOREST_HAVEN: Stage = Stage {
    name: "Gallery - Forest Haven Figurines",
    value: *b"figureG\0",
};

pub const GANONS_TOWER_ENTRANCE: Stage = Stage {
    name: "Ganon's Tower - Entrance",
    value: *b"GanonA\0\0",
};

pub const GANONS_TOWER_DRC_TRAIL: Stage = Stage {
    name: "Ganon's Tower - DRC Trail",
    value: *b"GanonB\0\0",
};

pub const GANONS_TOWER_WT_TRAIL: Stage = Stage {
    name: "Ganon's Tower - WT Trail",
    value: *b"GanonD\0\0",
};

pub const GANONS_TOWER_FW_TRAIL: Stage = Stage {
    name: "Ganon's Tower - FW Trail",
    value: *b"GanonD\0\0",
};

pub const GANONS_TOWER_ET_TRAIL: Stage = Stage {
    name: "Ganon's Tower - ET Trail",
    value: *b"GanonE\0\0",
};

pub const GANONS_TOWER_MAZE: Stage = Stage {
    name: "Ganon's Tower - Maze",
    value: *b"GanonJ\0\0",
};

pub const GANONS_TOWER_GANONS_ROOM: Stage = Stage {
    name: "Ganon's Tower - Ganon's Room",
    value: *b"GanonK\0\0",
};

pub const GANONS_TOWER_STAIRCASE_TO_GANON: Stage = Stage {
    name: "Ganon's Tower - Staircase to Ganon",
    value: *b"GanonL\0\0",
};

pub const GANONS_TOWER_CENTER: Stage = Stage {
    name: "Ganon's Tower - Center",
    value: *b"GanonM\0\0",
};

pub const GANONS_TOWER_STAIRCASE_TO_CENTER: Stage = Stage {
    name: "Ganon's Tower - Staircase to Center",
    value: *b"GanonN\0\0",
};

pub const GANONS_TOWER_FINAL_FIGHT: Stage = Stage {
    name: "Ganon's Tower - Final Fight",
    value: *b"GTower\0\0",
};

pub const HYRULE_CASTLE_INTERIOR: Stage = Stage {
    name: "Hyrule Castle - Interior",
    value: *b"Hyroom\0\0",
};

pub const HYRULE_CASTLE_EXTERIOR: Stage = Stage {
    name: "Hyrule Castle - Exterior",
    value: *b"Hyrule\0\0",
};

pub const BOMB_ISLAND_CAVE_TEST: Stage = Stage {
    name: "Bomb Island - Cave Test",
    value: *b"ITest61\0",
};

pub const ICE_RING_ISLE_GROTTO: Stage = Stage {
    name: "Ice Ring Isle - Grotto",
    value: *b"ITest62\0",
};

pub const SHARK_ISLAND_CAVE: Stage = Stage {
    name: "Shark Island - Cave",
    value: *b"ITest63\0",
};

pub const WINDFALL_ISLAND_MINIGAME_HOUSE: Stage = Stage {
    name: "Windfall Island - Minigame House",
    value: *b"Kaisen\0\0",
};

pub const UNUSED_FIRE_MOUNTAIN: Stage = Stage {
    name: "Unused Fire Mountain",
    value: *b"kazan\0\0\0",
};

pub const WIND_TEMPLE: Stage = Stage {
    name: "Wind Temple",
    value: *b"kaze\0\0\0\0",
};

pub const WIND_TEMPLE_BOSS_ROOM: Stage = Stage {
    name: "Wind Temple - Boss Room",
    value: *b"kazeB\0\0\0",
};

pub const WIND_TEMPLE_MINIBOSS_ROOM: Stage = Stage {
    name: "Wind Temple - Miniboss Room",
    value: *b"kazeMB\0\0",
};

pub const HYRULE_CASTLE_BASEMENT: Stage = Stage {
    name: "Hyrule Castle - Basement",
    value: *b"kenroom\0",
};

pub const FORBIDDEN_WOODS_BOSS_ROOM: Stage = Stage {
    name: "Forbidden Woods - Boss Room",
    value: *b"kinBOSS\0",
};

pub const FORBIDDEN_WOODSE: Stage = Stage {
    name: "Forbidden Woods",
    value: *b"kindan\0\0",
};

pub const FORBIDDEN_WOODS_MINIBOSS_ROOM: Stage = Stage {
    name: "Forbidden Woods - Miniboss Room",
    value: *b"kinMB\0\0\0",
};

pub const OUTSET_ISLAND_LINKS_HOUSE: Stage = Stage {
    name: "Outset Island - Link's House",
    value: *b"LinkRM\0\0",
};

pub const OUTSET_ISLAND_BASEMENT: Stage = Stage {
    name: "Outset Island - Basement",
    value: *b"LinkUG\0\0",
};

pub const EARTH_TEMPLE: Stage = Stage {
    name: "Earth Temple",
    value: *b"M_Dai\0\0\0",
};

pub const EARTH_TEMPLE_BOSS_ROOM: Stage = Stage {
    name: "Earth Temple - Boss Room",
    value: *b"M_DaiB\0\0",
};

pub const DRAGON_ROOST_CAVEN_BOSS_ROOM: Stage = Stage {
    name: "Dragon Roost Cavern - Boss Room",
    value: *b"M_DragB\0",
};

pub const DRAGON_ROOST_CAVEN: Stage = Stage {
    name: "Dragon Roost Cavern",
    value: *b"M_NewD2\0",
};

pub const FORSAKEN_FORTRESS_GANONS_LAIR: Stage = Stage {
    name: "Forsaken Fortress - Ganon's Lair",
    value: *b"M2ganon\0",
};

pub const FORSAKEN_FORTRESS_INTERIOR_1: Stage = Stage {
    name: "Forsaken Fortress - Interior 1",
    value: *b"ma2room\0",
};

pub const FORSAKEN_FORTRESS_INTERIOR_2: Stage = Stage {
    name: "Forsaken Fortress - Interior 2",
    value: *b"ma3room\0",
};

pub const FORSAKEN_FORTRESS_INTERIOR_3: Stage = Stage {
    name: "Forsaken Fortress - Interior 3",
    value: *b"majroom\0",
};

pub const FORSAKEN_FORTRESS_EXTERIOR: Stage = Stage {
    name: "Forsaken Fortress - Exterior",
    value: *b"MajyuE\0\0",
};

pub const INSIDE_ICE_RING_ISLE: Stage = Stage {
    name: "Inside Ice Ring Isle",
    value: *b"MiniHyo\0",
};

pub const INSIDE_FIRE_MOUNTAIN: Stage = Stage {
    name: "Inside Fire Mountain",
    value: *b"MiniKaz\0",
};

pub const FORSAKEN_FORTRESS_HELMROCS_ROOM: Stage = Stage {
    name: "Forsaken Fortress - Helmroc's Room",
    value: *b"Mjtower\0",
};

pub const UNUSED_TEMPLE_ISLAND: Stage = Stage {
    name: "Unused Temple Island",
    value: *b"Mukao\0\0\0",
};

pub const FILE_SELECT: Stage = Stage {
    name: "File Select",
    value: *b"Name\0\0\0\0",
};

pub const WINDFALL_ISLAND_SCHOOL_OF_JOY: Stage = Stage {
    name: "Windfall Island - School of Joy",
    value: *b"Nitiyou\0",
};

pub const WINDFALL_ISLAND_BOMB_SHOP: Stage = Stage {
    name: "Windfall Island - Bomb Shop",
    value: *b"Obombh\0\0",
};

pub const BEEDLES_SHOP_SHIP: Stage = Stage {
    name: "Beedle's Shop Ship",
    value: *b"Obshop\0\0",
};

pub const BOATING_COURSE: Stage = Stage {
    name: "Boating Course",
    value: *b"Ocean\0\0\0",
};

pub const WINDFALL_ISLAND_LENZOS_STUDIO: Stage = Stage {
    name: "Windfall Island - Lenzo's Studio",
    value: *b"Ocmera\0\0",
};

pub const FOREST_HAVEN_BOMB_SHOP: Stage = Stage {
    name: "Forest Haven - Bomb Shop",
    value: *b"Ocrogh\0\0",
};

pub const OUTSET_ISLAND_ORCAS_HOUSE: Stage = Stage {
    name: "Outset Island - Orca's House",
    value: *b"Ojhous\0\0",
};

pub const OUTSET_ISLAND_STURGEONS_HOUSE: Stage = Stage {
    name: "Outset Island - Sturgeon's House",
    value: *b"Ojhous2\0",
};

pub const OUTSET_ISLAND_MESAS_HOUSE: Stage = Stage {
    name: "Outset Island - Mesa's House",
    value: *b"Omasao\0\0",
};

pub const FOREST_HAVEN_INTERIOR: Stage = Stage {
    name: "Forest Haven - Interior",
    value: *b"Omori\0\0\0",
};

pub const OUTSET_ISLAND_ABES_HOUSE: Stage = Stage {
    name: "Outset Island - Abe's House",
    value: *b"Onobuta\0",
};

pub const WINDFALL_ISLAND_CAFE_BAR: Stage = Stage {
    name: "Windfall Island - Cafe Bar",
    value: *b"Opub\0\0\0\0",
};

pub const WINDFALL_ISLAND_AUCTION_HOUSE: Stage = Stage {
    name: "Windfall Island - Auction House",
    value: *b"Orichh\0\0",
};

pub const FOREST_HAVEN_BEHIND_WATERFALL: Stage = Stage {
    name: "Forest Haven - Behind Waterfall",
    value: *b"Otkura\0\0",
};

pub const WINDFALL_ISLAND_POTION_SHOP: Stage = Stage {
    name: "Windfall Island - Potion Shop",
    value: *b"Pdrgsh\0\0",
};

pub const NINTENDO_GALLERY: Stage = Stage {
    name: "Nintendo Gallery",
    value: *b"Pfigure\0",
};

pub const OUTSET_ISLAND_JABUNS_CAVERN: Stage = Stage {
    name: "Outset Island - Jabun's Cavern",
    value: *b"Pjavdou\0",
};

pub const WINDFALL_ISLAND_PRISON: Stage = Stage {
    name: "Windfall Island - Prison",
    value: *b"Pnezumi\0",
};

pub const GHOST_SHIP: Stage = Stage {
    name: "Ghost Ship",
    value: *b"PShip\0\0\0",
};

pub const SUBMARINE_UNUSED_ROOM_1: Stage = Stage {
    name: "Submarine - Unused Room",
    value: *b"PShip2\0\0",
};

pub const SUBMARINE_UNUSED_ROOM_2: Stage = Stage {
    name: "Submarine - Unused Room 2",
    value: *b"PShip3\0\0",
};

pub const GREAT_SEA: Stage = Stage {
    name: "Great Sea",
    value: *b"sea\0\0\0\0\0",
};

pub const CREDITS: Stage = Stage {
    name: "Credits",
    value: *b"sea_E\0\0\0",
};

pub const TITLE_SCREEN: Stage = Stage {
    name: "Title Screen",
    value: *b"sea_T\0\0\0",
};

pub const ISLET_OF_STEEL_INTERIOR: Stage = Stage {
    name: "Islet of Steel - Interior",
    value: *b"ShipD\0\0\0",
};

pub const TOWER_OF_GODS: Stage = Stage {
    name: "Tower of Gods",
    value: *b"Siren\0\0\0",
};

pub const TOWER_OF_GODS_BOSS_ROOM: Stage = Stage {
    name: "Tower of Gods - Boss Room",
    value: *b"SirenB\0\0",
};

pub const TOWER_OF_GODS_MINIBOSS_ROOM: Stage = Stage {
    name: "Tower of Gods - Miniboss Room",
    value: *b"SirenMB\0",
};

pub const NEEDLE_ROCK_ISLE_CAVE: Stage = Stage {
    name: "Needle Rock Isle - Cave",
    value: *b"SubD42\0\0",
};

pub const ANGULAR_ISLES_CAVE: Stage = Stage {
    name: "Angular Isles - Cave",
    value: *b"SubD43\0\0",
};

pub const STONEWATCHER_ISLAND_CAVE: Stage = Stage {
    name: "Stonewatcher Island - Cave",
    value: *b"SubD44\0\0",
};

pub const BOMB_ISLAND_EARLY_CAVE_1: Stage = Stage {
    name: "Bomb Island - Early Cave",
    value: *b"SubD51\0\0",
};

pub const BOMB_ISLAND_EARLY_CAVE_2: Stage = Stage {
    name: "Bomb Island - Early Cave",
    value: *b"SubD71\0\0",
};

pub const TINGLES_PAINT_ROOM: Stage = Stage {
    name: "Tingle's Paint Room",
    value: *b"tincle\0\0",
};

pub const STONEWATCHER_ISLAND_TRIFORCE_CAVE: Stage = Stage {
    name: "Stonewatcher Island - Triforce Cave",
    value: *b"TF_01\0\0\0",
};

pub const OVERLOOK_ISLAND_TRIFORCE_CAVE: Stage = Stage {
    name: "Overlook Island - Triforce Cave",
    value: *b"TF_02\0\0\0",
};

pub const BIRDS_PEAK_ROCK_TRIFORCE_CAVE: Stage = Stage {
    name: "Bird's Peak Rock - Triforce Cave",
    value: *b"TF_03\0\0\0",
};

pub const LINKS_OASIS_TRIFORCE_CAVE: Stage = Stage {
    name: "Link's Oasis - Triforce Cave",
    value: *b"TF_04\0\0\0",
};

pub const DRAGON_ROOST_ISLAND_CAVE: Stage = Stage {
    name: "Dragon Roost Island - Cave",
    value: *b"TF_06\0\0\0",
};

pub const PAWPRINT_ISLE_CAVE_2: Stage = Stage {
    name: "Pawprint Isle - Cave",
    value: *b"TyuTyu\0\0",
};

pub const DIAMOND_STEPPE_ISLAND_CAVE: Stage = Stage {
    name: "Diamond Steppe Island - Cave",
    value: *b"WarpD\0\0\0",
};

pub const GANONS_TOWER_DRC_TRAIL_BOSS: Stage = Stage {
    name: "Ganon's Tower - DRC Trail Boss",
    value: *b"Xboxx0\0\0",
};

pub const GANONS_TOWER_FW_TRAIL_BOSS: Stage = Stage {
    name: "Ganon's Tower - FW Trail Boss",
    value: *b"Xboss1\0\0",
};

pub const GANONS_TOWER_ET_TRAIL_BOSS: Stage = Stage {
    name: "Ganon's Tower - ET Trail Boss",
    value: *b"Xboss2\0\0",
};

pub const GANONS_TOWER_WT_TRAIL_BOSS: Stage = Stage {
    name: "Ganon's Tower - WT Trail Boss",
    value: *b"Xboss3\0\0",
};

pub const STAGES: [Stage; 123] = [
    OUTSET_FOREST,
    INVISIBLE_ISLAND,
    PIRATE_SHIP_DECK,
    LINKS_OASIS,
    SUBMARINE_INTERIOR,
    DRAGON_ROOST_ISLAND_SPRING,
    TOWER_OF_GODS_CUTSCENE,
    PIRATE_SHIP_INTERIOR,
    DRAGON_ROOST_ISLAND_MAIL_CENTER,
    BOMB_ISLAND_CAVE,
    STAR_ISLAND_CAVE,
    CLIFF_PLATEAU_ISLES_CAVE,
    ROCK_SPIRE_ISLE_CAVE,
    HORSESHOE_ISLAND_CAVE,
    OUTSET_ISLAND_UNUSED_CAVE,
    PAWPRINT_ISLE_CAVE_1,
    SAVAGE_LABYRINTH_1,
    SAVAGE_LABYRINTH_2,
    SAVAGE_LABYRINTH_3,
    DRAGON_ROOST_ISLAND_KOMALIS_ROOM,
    OUTSET_ISLAND_TEST,
    EARTH_TEMPLE_ENTRANCE,
    WIND_TEMPLE_ENTRANCE,
    END_GAME_CUTSCENE,
    NORTHERN_FAIRY_ISLAND_FAIRY,
    EASTERN_FAIRY_ISLAND_FAIRY,
    WESTERN_FAIRY_ISLAND_FAIRY,
    OUTSET_ISLAND_FOREST_FAIRY,
    THORNED_FAIRY_ISLAND_FAIRY,
    SOUTHERN_FAIRY_ISLAND_FAIRY,
    GALLERY_GREAT_SEA,
    GALLERY_WINDFALL_ISLAND,
    GALLERY_OUTSET_ISLAND,
    GALLERY_BOSS,
    GALLERY_ENEMY,
    GALLERY_DRC,
    GALLERY_FOREST_HAVEN,
    GANONS_TOWER_ENTRANCE,
    GANONS_TOWER_DRC_TRAIL,
    GANONS_TOWER_WT_TRAIL,
    GANONS_TOWER_FW_TRAIL,
    GANONS_TOWER_ET_TRAIL,
    GANONS_TOWER_MAZE,
    GANONS_TOWER_GANONS_ROOM,
    GANONS_TOWER_STAIRCASE_TO_GANON,
    GANONS_TOWER_CENTER,
    GANONS_TOWER_STAIRCASE_TO_CENTER,
    GANONS_TOWER_FINAL_FIGHT,
    HYRULE_CASTLE_INTERIOR,
    HYRULE_CASTLE_EXTERIOR,
    BOMB_ISLAND_CAVE_TEST,
    ICE_RING_ISLE_GROTTO,
    SHARK_ISLAND_CAVE,
    WINDFALL_ISLAND_MINIGAME_HOUSE,
    UNUSED_FIRE_MOUNTAIN,
    WIND_TEMPLE,
    WIND_TEMPLE_BOSS_ROOM,
    WIND_TEMPLE_MINIBOSS_ROOM,
    HYRULE_CASTLE_BASEMENT,
    FORBIDDEN_WOODS_BOSS_ROOM,
    FORBIDDEN_WOODSE,
    FORBIDDEN_WOODS_MINIBOSS_ROOM,
    OUTSET_ISLAND_LINKS_HOUSE,
    OUTSET_ISLAND_BASEMENT,
    EARTH_TEMPLE,
    EARTH_TEMPLE_BOSS_ROOM,
    DRAGON_ROOST_CAVEN_BOSS_ROOM,
    DRAGON_ROOST_CAVEN,
    FORSAKEN_FORTRESS_GANONS_LAIR,
    FORSAKEN_FORTRESS_INTERIOR_1,
    FORSAKEN_FORTRESS_INTERIOR_2,
    FORSAKEN_FORTRESS_INTERIOR_3,
    FORSAKEN_FORTRESS_EXTERIOR,
    INSIDE_ICE_RING_ISLE,
    INSIDE_FIRE_MOUNTAIN,
    FORSAKEN_FORTRESS_HELMROCS_ROOM,
    UNUSED_TEMPLE_ISLAND,
    FILE_SELECT,
    WINDFALL_ISLAND_SCHOOL_OF_JOY,
    WINDFALL_ISLAND_BOMB_SHOP,
    BEEDLES_SHOP_SHIP,
    BOATING_COURSE,
    WINDFALL_ISLAND_LENZOS_STUDIO,
    FOREST_HAVEN_BOMB_SHOP,
    OUTSET_ISLAND_ORCAS_HOUSE,
    OUTSET_ISLAND_STURGEONS_HOUSE,
    OUTSET_ISLAND_MESAS_HOUSE,
    FOREST_HAVEN_INTERIOR,
    OUTSET_ISLAND_ABES_HOUSE,
    WINDFALL_ISLAND_CAFE_BAR,
    WINDFALL_ISLAND_AUCTION_HOUSE,
    FOREST_HAVEN_BEHIND_WATERFALL,
    WINDFALL_ISLAND_POTION_SHOP,
    NINTENDO_GALLERY,
    OUTSET_ISLAND_JABUNS_CAVERN,
    WINDFALL_ISLAND_PRISON,
    GHOST_SHIP,
    SUBMARINE_UNUSED_ROOM_1,
    SUBMARINE_UNUSED_ROOM_2,
    GREAT_SEA,
    CREDITS,
    TITLE_SCREEN,
    ISLET_OF_STEEL_INTERIOR,
    TOWER_OF_GODS,
    TOWER_OF_GODS_BOSS_ROOM,
    TOWER_OF_GODS_MINIBOSS_ROOM,
    NEEDLE_ROCK_ISLE_CAVE,
    ANGULAR_ISLES_CAVE,
    STONEWATCHER_ISLAND_CAVE,
    BOMB_ISLAND_EARLY_CAVE_1,
    BOMB_ISLAND_EARLY_CAVE_2,
    TINGLES_PAINT_ROOM,
    STONEWATCHER_ISLAND_TRIFORCE_CAVE,
    OVERLOOK_ISLAND_TRIFORCE_CAVE,
    BIRDS_PEAK_ROCK_TRIFORCE_CAVE,
    LINKS_OASIS_TRIFORCE_CAVE,
    DRAGON_ROOST_ISLAND_CAVE,
    PAWPRINT_ISLE_CAVE_2,
    DIAMOND_STEPPE_ISLAND_CAVE,
    GANONS_TOWER_DRC_TRAIL_BOSS,
    GANONS_TOWER_FW_TRAIL_BOSS,
    GANONS_TOWER_ET_TRAIL_BOSS,
    GANONS_TOWER_WT_TRAIL_BOSS,
];

pub fn value_to_name(value: [u8; 8]) -> String {
    let mut name = String::new();

    let mut value = value;
    if let Some(index) = value.iter().position(|&x| x == 0) {
        value[(index + 1)..].fill(0);
    }

    for stage in STAGES {
        if value == stage.value {
            name.push_str(stage.name);
            break;
        }
    }

    if name.is_empty() {
        name.push_str(&format!(
            "Unknown ({})",
            core::ffi::CStr::from_bytes_until_nul(&value)
                .unwrap()
                .to_string_lossy()
        ));
    }

    name
}

// endregion

pub mod daytime {
    pub const ADDRESS: *mut u32 = 0x1506b524 as *mut u32;

    pub const DAWN: u32 = 0x4300_0000;
    pub const DAY: u32 = 0x4320_0000;
    pub const NIGHT: u32 = 0x3f80_0000;
}

pub mod weather {
    pub const ADDRESS: *mut u32 = 0x10978cf4 as *mut u32;

    pub const NORMAL: u32 = 0x0000_ffff;
    pub const CLOUDY: u32 = 0x0001_ffff;
    pub const FOGGY: u32 = 0x0002_ffff;
}
