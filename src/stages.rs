#![allow(dead_code, unused_imports)]

macro_rules! impl_display {
    ($s:ident) => {
        impl ::core::fmt::Display for $s {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
    };
}

pub mod reload {
    pub const ADDRESS: *mut u8 = 0x109763fc as *mut u8;

    #[inline]
    pub fn write(value: u8) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> u8 {
        unsafe { core::ptr::read(ADDRESS) }
    }

    #[inline]
    pub fn activate() {
        write(1);
    }
}

pub mod spawn {
    pub const ADDRESS: *mut u8 = 0x109763f9 as *mut u8;

    #[inline]
    pub fn write(value: u8) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> u8 {
        unsafe { core::ptr::read(ADDRESS) }
    }
}

pub mod room {
    pub const ADDRESS: *mut u8 = 0x109763fa as *mut u8;

    #[inline]
    pub fn write(value: u8) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> u8 {
        unsafe { core::ptr::read(ADDRESS) }
    }
}

pub mod layer {
    pub const ADDRESS: *mut u8 = 0x109763fb as *mut u8;

    #[inline]
    pub fn write(value: u8) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> u8 {
        unsafe { core::ptr::read(ADDRESS) }
    }
}

// region: Stages

pub mod stage {
    pub const ADDRESS: *mut [u8; 8] = 0x109763f0 as *mut [u8; 8];

    macro_rules! generate_stages {
        ($(
            ($variant:ident, $name:expr, $id:expr),
        )*) => {
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub enum Stage {
                $(
                    $variant,
                )*
            }

            impl Stage {
                pub const fn id(&self) -> [u8; 8] {
                    match self {
                        $(
                            Self::$variant => $id,
                        )*
                    }
                }

                pub const fn name(&self) -> &'static str {
                    match self {
                        $(
                            Self::$variant => $name,
                        )*
                    }
                }
            }

            impl TryFrom<[u8; 8]> for Stage {
                type Error = ();

                fn try_from(value: [u8; 8]) -> Result<Self, Self::Error> {
                    match value {
                        $(
                            x if x == $id => Ok(Self::$variant),
                        )*
                        _ => Err(()),
                    }
                }
            }
        }
    }

    pub use Stage::*;

    generate_stages!(
        (OutsetForest, "Outset Island - Forest", *b"A_mori\0\0"),
        (InvisibleIsland, "Invisible Island", *b"A_nami\0\0"),
        (PirateShipDeck, "Pirate Ship - Deck", *b"A_umikz\0"),
        (LinksOasis, "Link's Oasis", *b"Abesso\0\0"),
        (SubmarineInterior, "Submarine Interior", *b"Abship\0\0"),
        (
            DragonRoostIslandSpring,
            "Dragon Roost Island - Spring",
            *b"Adanmae\0"
        ),
        (
            TowerOfGodsCutscene,
            "Tower of Gods - Cutscene",
            *b"ADMumi\0\0"
        ),
        (
            PirateShipInterior,
            "Pirate Ship - Interior",
            *b"Asoko\0\0\0"
        ),
        (
            DragonRoostIslandMailCenter,
            "Dragon Roost Island - Mail Center",
            *b"Atorizk\0"
        ),
        (BombIslandCave, "Bomb Island - Cave", *b"Cave01\0\0"),
        (StarIslandCave, "Star Island - Cave", *b"Cave02\0\0"),
        (
            CliffPlateauIslesCave,
            "Cliff Plateau Isles - Cave",
            *b"Cave03\0\0"
        ),
        (RockSpireIsleCave, "Rock Spire Isle - Cave", *b"Cave04\0\0"),
        (
            HorseshoeIslandCave,
            "Horseshoe Island - Cave",
            *b"Cave05\0\0"
        ),
        (
            OutsetIslandUnusedCave,
            "Outset Island - Unused Cave",
            *b"Cave06\0\0"
        ),
        (PawprintIsleCave1, "Pawprint Isle - Cave", *b"Cave07\0\0"),
        (SavageLabyrinth1, "Savage Labyrinth", *b"Cave09\0\0"),
        (SavageLabyrinth2, "Savage Labyrinth", *b"Cave10\0\0"),
        (SavageLabyrinth3, "Savage Labyrinth", *b"Cave11\0\0"),
        (
            DragonRoostIslandKomalisRoom,
            "Dragon Roost Island - Komali's Room",
            *b"Comori\0\0"
        ),
        (OutsetIslandTest, "Outset Island - Test", *b"DmSpot0\0"),
        (
            EarthTempleEntrance,
            "Earth Temple - Entrance",
            *b"Edaichi\0"
        ),
        (
            WindTempleEntrance,
            "Wind Temple - Entrance",
            *b"Ekaze\0\0\0"
        ),
        (EndGameCutscene, "End Game Cutscene", *b"ENDumi\0\0"),
        (
            NorthernFairyIslandFairy,
            "Northern Fairy Island - Fairy",
            *b"Fairy01\0"
        ),
        (
            EasternFairyIslandFairy,
            "Eastern Fairy Island - Fairy",
            *b"Fairy02\0"
        ),
        (
            WesternFairyIslandFairy,
            "Western Fairy Island - Fairy",
            *b"Fairy03\0"
        ),
        (
            OutsetIslandForestFairy,
            "Outset Island - Forest - Fairy",
            *b"Fairy04\0"
        ),
        (
            ThornedFairyIslandFairy,
            "Thorned Fairy Island - Fairy",
            *b"Fairy05\0"
        ),
        (
            SouthernFairyIslandFairy,
            "Southern Fairy Island - Fairy",
            *b"Fairy06\0"
        ),
        (
            GalleryGreatSea,
            "Gallery - Great Sea Figurines",
            *b"figureA\0"
        ),
        (
            GalleryWindfallIsland,
            "Gallery - Windfall Island Figurines",
            *b"figureB\0"
        ),
        (
            GalleryOutsetIsland,
            "Gallery - Outset Island Figurines",
            *b"figureC\0"
        ),
        (GalleryBoss, "Gallery - Boss Figurines", *b"figureD\0"),
        (GalleryEnemy, "Gallery - Enemy Figurines", *b"figureE\0"),
        (GalleryDrc, "Gallery - DRC Figurines", *b"figureF\0"),
        (
            GalleryForestHaven,
            "Gallery - Forest Haven Figurines",
            *b"figureG\0"
        ),
        (
            GanonsTowerEntrance,
            "Ganon's Tower - Entrance",
            *b"GanonA\0\0"
        ),
        (
            GanonsTowerDrcTrail,
            "Ganon's Tower - DRC Trail",
            *b"GanonB\0\0"
        ),
        (
            GanonsTowerWtTrail,
            "Ganon's Tower - WT Trail",
            *b"GanonD\0\0"
        ),
        (
            GanonsTowerFwTrail,
            "Ganon's Tower - FW Trail",
            *b"GanonD\0\0"
        ),
        (
            GanonsTowerEtTrail,
            "Ganon's Tower - ET Trail",
            *b"GanonE\0\0"
        ),
        (GanonsTowerMaze, "Ganon's Tower - Maze", *b"GanonJ\0\0"),
        (
            GanonsTowerGanonsRoom,
            "Ganon's Tower - Ganon's Room",
            *b"GanonK\0\0"
        ),
        (
            GanonsTowerStaircaseToGanon,
            "Ganon's Tower - Staircase to Ganon",
            *b"GanonL\0\0"
        ),
        (GanonsTowerCenter, "Ganon's Tower - Center", *b"GanonM\0\0"),
        (
            GanonsTowerStaircaseToCenter,
            "Ganon's Tower - Staircase to Center",
            *b"GanonN\0\0"
        ),
        (
            GanonsTowerFinalFight,
            "Ganon's Tower - Final Fight",
            *b"GTower\0\0"
        ),
        (
            HyruleCastleInterior,
            "Hyrule Castle - Interior",
            *b"Hyroom\0\0"
        ),
        (
            HyruleCastleExterior,
            "Hyrule Castle - Exterior",
            *b"Hyrule\0\0"
        ),
        (BombIslandCaveTest, "Bomb Island - Cave Test", *b"ITest61\0"),
        (IceRingIsleGrotto, "Ice Ring Isle - Grotto", *b"ITest62\0"),
        (SharkIslandCave, "Shark Island - Cave", *b"ITest63\0"),
        (
            WindfallIslandMinigameHouse,
            "Windfall Island - Minigame House",
            *b"Kaisen\0\0"
        ),
        (UnusedFireMountain, "Unused Fire Mountain", *b"kazan\0\0\0"),
        (WindTemple, "Wind Temple", *b"kaze\0\0\0\0"),
        (
            WindTempleBossRoom,
            "Wind Temple - Boss Room",
            *b"kazeB\0\0\0"
        ),
        (
            WindTempleMinibossRoom,
            "Wind Temple - Miniboss Room",
            *b"kazeMB\0\0"
        ),
        (
            HyruleCastleBasement,
            "Hyrule Castle - Basement",
            *b"kenroom\0"
        ),
        (
            ForbiddenWoodsBossRoom,
            "Forbidden Woods - Boss Room",
            *b"kinBOSS\0"
        ),
        (ForbiddenWoods, "Forbidden Woods", *b"kindan\0\0"),
        (
            ForbiddenWoodsMinibossRoom,
            "Forbidden Woods - Miniboss Room",
            *b"kinMB\0\0\0"
        ),
        (
            OutsetIslandLinksHouse,
            "Outset Island - Link's House",
            *b"LinkRM\0\0"
        ),
        (
            OutsetIslandBasement,
            "Outset Island - Basement",
            *b"LinkUG\0\0"
        ),
        (EarthTemple, "Earth Temple", *b"M_Dai\0\0\0"),
        (
            EarthTempleBossRoom,
            "Earth Temple - Boss Room",
            *b"M_DaiB\0\0"
        ),
        (
            DragonRoostCavernBossRoom,
            "Dragon Roost Cavern - Boss Room",
            *b"M_DragB\0"
        ),
        (DragonRoostCavern, "Dragon Roost Cavern", *b"M_NewD2\0"),
        (
            ForsakenFortressGanonsLair,
            "Forsaken Fortress - Ganon's Lair",
            *b"M2ganon\0"
        ),
        (
            ForsakenFortressInterior1,
            "Forsaken Fortress - Interior 1",
            *b"ma2room\0"
        ),
        (
            ForsakenFortressInterior2,
            "Forsaken Fortress - Interior 2",
            *b"ma3room\0"
        ),
        (
            ForsakenFortressInterior3,
            "Forsaken Fortress - Interior 3",
            *b"majroom\0"
        ),
        (
            ForsakenFortressExterior,
            "Forsaken Fortress - Exterior",
            *b"MajyuE\0\0"
        ),
        (InsideIceRingIsle, "Inside Ice Ring Isle", *b"MiniHyo\0"),
        (InsideFireMountain, "Inside Fire Mountain", *b"MiniKaz\0"),
        (
            ForsakenFortressHelmrocsRoom,
            "Forsaken Fortress - Helmroc's Room",
            *b"Mjtower\0"
        ),
        (UnusedTempleIsland, "Unused Temple Island", *b"Mukao\0\0\0"),
        (FileSelect, "File Select", *b"Name\0\0\0\0"),
        (
            WindfallIslandSchoolOfJoy,
            "Windfall Island - School of Joy",
            *b"Nitiyou\0"
        ),
        (
            WindfallIslandBombShop,
            "Windfall Island - Bomb Shop",
            *b"Obombh\0\0"
        ),
        (BeedlesShopShip, "Beedle's Shop Ship", *b"Obshop\0\0"),
        (BoatingCourse, "Boating Course", *b"Ocean\0\0\0"),
        (
            WindfallIslandLenzosStudio,
            "Windfall Island - Lenzo's Studio",
            *b"Ocmera\0\0"
        ),
        (
            ForestHavenBombShop,
            "Forest Haven - Bomb Shop",
            *b"Ocrogh\0\0"
        ),
        (
            OutsetIslandOrcasHouse,
            "Outset Island - Orca's House",
            *b"Ojhous\0\0"
        ),
        (
            OutsetIslandSturgeonsHouse,
            "Outset Island - Sturgeon's House",
            *b"Ojhous2\0"
        ),
        (
            OutsetIslandMesasHouse,
            "Outset Island - Mesa's House",
            *b"Omasao\0\0"
        ),
        (
            ForestHavenInterior,
            "Forest Haven - Interior",
            *b"Omori\0\0\0"
        ),
        (
            OutsetIslandAbesHouse,
            "Outset Island - Abe's House",
            *b"Onobuta\0"
        ),
        (
            WindfallIslandCafeBar,
            "Windfall Island - Cafe Bar",
            *b"Opub\0\0\0\0"
        ),
        (
            WindfallIslandAuctionHouse,
            "Windfall Island - Auction House",
            *b"Orichh\0\0"
        ),
        (
            ForestHavenBehindWaterfall,
            "Forest Haven - Behind Waterfall",
            *b"Otkura\0\0"
        ),
        (
            WindfallIslandPotionShop,
            "Windfall Island - Potion Shop",
            *b"Pdrgsh\0\0"
        ),
        (NintendoGallery, "Nintendo Gallery", *b"Pfigure\0"),
        (
            OutsetIslandJabunsCavern,
            "Outset Island - Jabun's Cavern",
            *b"Pjavdou\0"
        ),
        (
            WindfallIslandPrison,
            "Windfall Island - Prison",
            *b"Pnezumi\0"
        ),
        (GhostShip, "Ghost Ship", *b"PShip\0\0\0"),
        (
            SubmarineUnusedRoom1,
            "Submarine - Unused Room",
            *b"PShip2\0\0"
        ),
        (
            SubmarineUnusedRoom2,
            "Submarine - Unused Room 2",
            *b"PShip3\0\0"
        ),
        (GreatSea, "Great Sea", *b"sea\0\0\0\0\0"),
        (Credits, "Credits", *b"sea_E\0\0\0"),
        (TitleScreen, "Title Screen", *b"sea_T\0\0\0"),
        (
            IsletOfSteelInterior,
            "Islet of Steel - Interior",
            *b"ShipD\0\0\0"
        ),
        (TowerOfGods, "Tower of Gods", *b"Siren\0\0\0"),
        (
            TowerOfGodsBossRoom,
            "Tower of Gods - Boss Room",
            *b"SirenB\0\0"
        ),
        (
            TowerOfGodsMinibossRoom,
            "Tower of Gods - Miniboss Room",
            *b"SirenMB\0"
        ),
        (
            NeedleRockIsleCave,
            "Needle Rock Isle - Cave",
            *b"SubD42\0\0"
        ),
        (AngularIslesCave, "Angular Isles - Cave", *b"SubD43\0\0"),
        (
            StonewatcherIslandCave,
            "Stonewatcher Island - Cave",
            *b"SubD44\0\0"
        ),
        (
            BombIslandEarlyCave1,
            "Bomb Island - Early Cave",
            *b"SubD51\0\0"
        ),
        (
            BombIslandEarlyCave2,
            "Bomb Island - Early Cave",
            *b"SubD71\0\0"
        ),
        (TinglesPaintRoom, "Tingle's Paint Room", *b"tincle\0\0"),
        (
            StoneWatcherIslandTriforceCave,
            "Stonewatcher Island - Triforce Cave",
            *b"TF_01\0\0\0"
        ),
        (
            OverlookIslandTriforceCave,
            "Overlook Island - Triforce Cave",
            *b"TF_02\0\0\0"
        ),
        (
            BirdsPeakRockTriforceCave,
            "Bird's Peak Rock - Triforce Cave",
            *b"TF_03\0\0\0"
        ),
        (
            LinksOasisTriforceCave,
            "Link's Oasis - Triforce Cave",
            *b"TF_04\0\0\0"
        ),
        (
            DragonRoostIslandCave,
            "Dragon Roost Island - Cave",
            *b"TF_06\0\0\0"
        ),
        (PawprintIsleCave2, "Pawprint Isle - Cave", *b"TyuTyu\0\0"),
        (
            DiamondSteppeIslandCave,
            "Diamond Steppe Island - Cave",
            *b"WarpD\0\0\0"
        ),
        (
            GanonsTowerDrcTrailBoss,
            "Ganon's Tower - DRC Trail Boss",
            *b"Xboxx0\0\0"
        ),
        (
            GanonsTowerFwTrailBoss,
            "Ganon's Tower - FW Trail Boss",
            *b"Xboss1\0\0"
        ),
        (
            GanonsTowerEtTrailBoss,
            "Ganon's Tower - ET Trail Boss",
            *b"Xboss2\0\0"
        ),
        (
            GanonsTowerWtTrailBoss,
            "Ganon's Tower - WT Trail Boss",
            *b"Xboss3\0\0"
        ),
    );

    #[inline]
    pub fn write(value: [u8; 8]) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> [u8; 8] {
        unsafe { core::ptr::read(ADDRESS) }
    }

    #[inline]
    pub fn set(stage: Stage) {
        write(stage.id());
    }

    #[inline]
    pub fn get() -> Stage {
        Stage::try_from(read()).unwrap_or(Stage::TitleScreen)
    }
}

pub mod daytime {
    pub const ADDRESS: *mut u32 = 0x1506b524 as *mut u32;

    pub use Daytime::*;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Daytime {
        Dawn = 0x4300_0000,
        Day = 0x4320_0000,
        Night = 0x3f80_0000,
    }

    impl_display!(Daytime);

    #[inline]
    pub fn write(value: u32) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> u32 {
        unsafe { core::ptr::read(ADDRESS) }
    }

    #[inline]
    pub fn set(value: Daytime) {
        write(value as u32);
    }
}

pub mod weather {
    pub const ADDRESS: *mut u32 = 0x10978cf4 as *mut u32;

    pub use Weather::*;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Weather {
        Normal = 0x0000_ffff,
        Cloudy = 0x0001_ffff,
        Foggy = 0x0002_ffff,
    }

    impl_display!(Weather);

    #[inline]
    pub fn write(value: u32) {
        unsafe {
            core::ptr::write(ADDRESS, value);
        }
    }

    #[inline]
    pub fn read() -> u32 {
        unsafe { core::ptr::read(ADDRESS) }
    }

    #[inline]
    pub fn set(value: Weather) {
        write(value as u32);
    }
}
