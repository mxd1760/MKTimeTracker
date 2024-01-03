
pub static SAVE_LOCATION:&str = "./test/";

#[derive(PartialEq, Eq)]
pub enum CourseTitle{
   MarioKartStadium ,       WaterPark ,              SweetSweetCanyon ,       ThwompRuins ,             // Mushroom Cup
   MarioCircuit ,           ToadHarbor ,             TwistedMansion ,         ShyGuyFalls ,            // Flower Cup
   SunshineAirport ,        DolphinShoals ,          Electrodrome ,           MountWario ,              // Star Cup
   CloudtopCruise ,         BoneDryDunes ,           BowsersCastle ,          RainbowRoad ,             // Special Cup
   WiiMooMooMeadows ,       GBAMarioCircuit ,        DSCheepCheepBoach ,      N64ToadsTurnpike ,      // Shell Cup
   GCNDryDryDesert ,        SNESDonutPlains3 ,       N64RoyalRaceway ,        _3DSDKJungle ,            // Banana Cup
   DSWarioStadium ,         GCNSherbetLand ,         _3DSMusicPark ,          N64YoshiValley ,         // Leaf Cup
   DSTickTockClock ,        _3DSPiranhaPlantSlide ,  WiiGrumbleVolcano ,      N64RainbowRoad ,         // Lightning Cup
   GCNYoshiCircuit ,        ExcitebikeArena ,        DragonDriftway ,         MuteCity ,                // Egg Cup
   WiiWariosGoldMine ,      SNESRainbowRoad ,        IceIceOutpost ,          HyruleCircuit ,           // Triforce Cup
   GCNBabyPark ,            GBACheeseLand ,          WildWoods ,              AnimalCrossing ,          // Crossing Cup
   _3DSNeoBowserCity ,      GBARibbonRoad ,          SuperBellSubway ,        BigBlue ,                 // Bell Cup
   TourParisPromenade ,     _3DSToadCircuit ,        N64ChocoMountain ,       WiiCoconutMall ,         // Golden Dash Cup
   TourTokyoBlur ,          DSShroomRidge ,          GBASkyGarden ,           NinjaHideaway ,           // Lucky Cat Cup
   TourNewYorkMinute ,      SNESMarioCircuit3 ,      N64KalimariDesert ,      DSWaluigiPinball ,       // Turnip Cup
   TourSydneySprint ,       GBASnowLand ,            WiiMushroomGorge ,       SkyHighSundae ,          // Propeller Cup
   TourLondonLoop ,         GBABooLake ,             _3DSRockRockMountain ,   WiiMapleTreeway ,        // Rock Cup
   TourBerlinByways ,       DSPeachGardens ,         MerryMountain ,          _3DSRainbowRoad ,         // Moon Cup
   TourAmsterdamDrift ,     GBARiversidePark ,       WiiDKSummit ,            YoshisIsland ,           // Fruit Cup
   TourBangkokRush ,        DSMarioCircuit ,         GCNWaluigiStadium ,      TourSingaporeSpeedway ,  // Boomerang Cup
   TourAthensDash ,         GCNDaisyCruiser ,        WiiMoonviewHighway ,     SqueakyCleanSprint ,     // Feather Cup
   TourLosAngelesLaps ,     GBASunsetWilds ,         WiiKoopaCape ,           TourVancouverVelocity ,  // Cherry Cup
   TourRomeAvanti ,         GCNDKMountain ,          WiiDaisyCircuit ,        PiranhaPlantCove ,       // Acorn Cup
   TourMadridDrive ,        _3DSRosalinasIceWorld ,  SNESBowserCastle3 ,      WiiRainbowRoad           // Spiny Cup
}
impl CourseTitle{
  pub fn value(&self) -> usize{
    match *self{
      Self::MarioKartStadium      => 0,
      Self::WaterPark             => 1,
      Self::SweetSweetCanyon      => 2,
      Self::ThwompRuins           => 3,
      Self::MarioCircuit          => 4,
      Self::ToadHarbor            => 5,
      Self::TwistedMansion        => 6,
      Self::ShyGuyFalls           => 7,
      Self::SunshineAirport       => 8,
      Self::DolphinShoals         => 9,
      Self::Electrodrome          => 10,
      Self::MountWario            => 11,
      Self::CloudtopCruise        => 12,
      Self::BoneDryDunes          => 12,
      Self::BowsersCastle         => 13,
      Self::RainbowRoad           => 14,
      Self::WiiMooMooMeadows      => 15,
      Self::GBAMarioCircuit       => 16,
      Self::DSCheepCheepBoach     => 17,
      Self::N64ToadsTurnpike      => 18,
      Self::GCNDryDryDesert       => 19,
      Self::SNESDonutPlains3      => 20,
      Self::N64RoyalRaceway       => 21,
      Self::_3DSDKJungle          => 22,
      Self::DSWarioStadium        => 23,
      Self::GCNSherbetLand        => 24,
      Self::_3DSMusicPark         => 25,
      Self::N64YoshiValley        => 26,
      Self::DSTickTockClock       => 27,
      Self::_3DSPiranhaPlantSlide => 28,
      Self::WiiGrumbleVolcano     => 29,
      Self::N64RainbowRoad        => 30,
      Self::GCNYoshiCircuit       => 31,
      Self::ExcitebikeArena       => 32,
      Self::DragonDriftway        => 33,
      Self::MuteCity              => 34,
      Self::WiiWariosGoldMine     => 35,
      Self::SNESRainbowRoad       => 36,
      Self::IceIceOutpost         => 37,
      Self::HyruleCircuit         => 38,
      Self::GCNBabyPark           => 39,
      Self::GBACheeseLand         => 40,
      Self::WildWoods             => 41,
      Self::AnimalCrossing        => 42,
      Self::_3DSNeoBowserCity     => 43,
      Self::GBARibbonRoad         => 44,
      Self::SuperBellSubway       => 45,
      Self::BigBlue               => 46,
      Self::TourParisPromenade    => 47,
      Self::_3DSToadCircuit       => 48,
      Self::N64ChocoMountain      => 49,
      Self::WiiCoconutMall        => 50,
      Self::TourTokyoBlur         => 51,
      Self::DSShroomRidge         => 52,
      Self::GBASkyGarden          => 53,
      Self::NinjaHideaway         => 54,
      Self::TourNewYorkMinute     => 55,
      Self::SNESMarioCircuit3     => 56,
      Self::N64KalimariDesert     => 57,
      Self::DSWaluigiPinball      => 58,
      Self::TourSydneySprint      => 59,
      Self::GBASnowLand           => 60,
      Self::WiiMushroomGorge      => 61,
      Self::SkyHighSundae         => 62,
      Self::TourLondonLoop        => 63,
      Self::GBABooLake            => 64,
      Self::_3DSRockRockMountain  => 65,
      Self::WiiMapleTreeway       => 66,
      Self::TourBerlinByways      => 67,
      Self::DSPeachGardens        => 68,
      Self::MerryMountain         => 69,
      Self::_3DSRainbowRoad       => 70,
      Self::TourAmsterdamDrift    => 71,
      Self::GBARiversidePark      => 72,
      Self::WiiDKSummit           => 73,
      Self::YoshisIsland          => 74,
      Self::TourBangkokRush       => 75,
      Self::DSMarioCircuit        => 76,
      Self::GCNWaluigiStadium     => 78,
      Self::TourSingaporeSpeedway => 79,
      Self::TourAthensDash        => 80,
      Self::GCNDaisyCruiser       => 81,
      Self::WiiMoonviewHighway    => 82,
      Self::SqueakyCleanSprint    => 83,
      Self::TourLosAngelesLaps    => 84,
      Self::GBASunsetWilds        => 85,
      Self::WiiKoopaCape          => 86,
      Self::TourVancouverVelocity => 87,
      Self::TourRomeAvanti        => 88,
      Self::GCNDKMountain         => 89,
      Self::WiiDaisyCircuit       => 90,
      Self::PiranhaPlantCove      => 91,
      Self::TourMadridDrive       => 92,
      Self::_3DSRosalinasIceWorld => 93,
      Self::SNESBowserCastle3     => 94,
      Self::WiiRainbowRoad        => 95,
    }
  }
}

#[derive(PartialEq, Eq)]
pub enum SrdcCategory{
   _48Tracks(ItemsAndCc),
   _48BCPTracks(ItemsAndCc), 
   _96Tracks(ItemsAndCc),  
   NitroTracks(ItemsAndCc),   
   RetroTracks(ItemsAndCc),   
   BonusTracks(ItemsAndCc),   
   BCPWaves1to2(ItemsAndCc),  
   BCPWaves3to4(ItemsAndCc),   
   BCPWaves5to6(ItemsAndCc),   
}

impl SrdcCategory{
  fn value(&self) -> usize{
    match *self{
        Self::_48Tracks(x)    => 0*4+x.value(),
        Self::_48BCPTracks(x) => 1*4+x.value(),
        Self::_96Tracks(x)    => 2*4+x.value(),
        Self::NitroTracks(x)  => 3*4+x.value(),
        Self::RetroTracks(x)  => 4*4+x.value(),
        Self::BonusTracks(x)  => 5*4+x.value(),
        Self::BCPWaves1to2(x) => 6*4+x.value(),
        Self::BCPWaves3to4(x) => 7*4+x.value(),
        Self::BCPWaves5to6(x) => 8*4+x.value(),
    }
  }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ItemsAndCc{
  Items200cc,
  Items150cc,
  NoItems200cc,
  NoItems150cc
}
impl ItemsAndCc{
  fn value(&self) -> usize{
    match *self{
        ItemsAndCc::Items200cc => 0,
        ItemsAndCc::Items150cc => 1,
        ItemsAndCc::NoItems200cc => 2,
        ItemsAndCc::NoItems150cc => 3,
    }
  }
}

#[derive(PartialEq, Eq, Clone,Copy)]
pub enum Character{
  Mario ,          Luigi ,            Peach ,           Daisy ,          Rosalina,         TanookiMario ,    CatPeach ,        Birdo(u8) ,            
  Yoshi(u8) ,      Toad ,             KoopaTroopa ,     ShyGuy(u8) ,     Lakitu,           Toadette ,        KingBoo ,         PeteyPiranha ,    
  BabyMario ,      BabyLuigi ,        BabyPeach ,       BabyDaisy ,      BabyRosalina ,    MetalMario(u8) ,  PinkGoldPeach ,   Wiggler ,
  Wario ,          Waluigi ,          DonkeyKong ,      Bowser ,         DryBones ,        BowserJr ,        DryBowser ,       Kamek ,
  Lemmy ,          Larry ,            Wendy ,           Ludwig ,         Iggy ,            Roy ,             Morton ,          Peachette ,
  Inkling(u8) ,    Villager(u8) ,     Isabelle ,        Link(u8) ,       DiddyKong,        FunkyKong ,       Pauline ,         Mii(u8,u8) 
}
impl Character{
  pub fn value(&self) -> usize{
    match *self{
        Character::Mario          => 0,
        Character::Luigi          => 1,
        Character::Peach          => 2,
        Character::Daisy          => 3,
        Character::Rosalina       => 4,
        Character::TanookiMario   => 5,
        Character::CatPeach       => 6,
        Character::Birdo(x)       => 7+1_000*x as usize,
        Character::Yoshi(x)       => 8+1_000*x as usize,
        Character::Toad           => 9,
        Character::KoopaTroopa    => 10,
        Character::ShyGuy(x)      => 11+1_000*x as usize,
        Character::Lakitu         => 12,
        Character::Toadette       => 13,
        Character::KingBoo        => 14,
        Character::PeteyPiranha   => 15,
        Character::BabyMario      => 16,
        Character::BabyLuigi      => 17,
        Character::BabyPeach      => 18,
        Character::BabyDaisy      => 19,
        Character::BabyRosalina   => 20,
        Character::MetalMario(x)  => 21+1_000*x as usize,
        Character::PinkGoldPeach  => 22,
        Character::Wiggler        => 23,
        Character::Wario          => 24,
        Character::Waluigi        => 25,
        Character::DonkeyKong     => 26,
        Character::Bowser         => 27,
        Character::DryBones       => 28,
        Character::BowserJr       => 29,
        Character::DryBowser      => 30,
        Character::Kamek          => 31,
        Character::Lemmy          => 32,
        Character::Larry          => 33,
        Character::Wendy          => 34,
        Character::Ludwig         => 35,
        Character::Iggy           => 36,
        Character::Roy            => 37,
        Character::Morton         => 38,
        Character::Peachette      => 39,
        Character::Inkling(x)     => 40+1_000*x as usize,
        Character::Villager(x)    => 41+1_000*x as usize,
        Character::Isabelle       => 42,
        Character::Link(x)        => 43+1_000*x as usize,
        Character::DiddyKong      => 44,
        Character::FunkyKong      => 45,
        Character::Pauline        => 46,
        Character::Mii(x, y)      => 47+1_000*x as usize + 10_000*y as usize,
    }
  }
  pub fn from_str(s:&str) -> Option<Character>{
    let n = s.parse::<usize>().unwrap();
    for i in Character::all(){
      if i.value() == n{
        return Some(i);
      }
    }
    return None;
  }
  fn all()->Vec<Character>{
    return vec![
        Character::Mario,
        Character::Luigi,
        Character::Peach,
        Character::Daisy,
        Character::Rosalina,
        Character::TanookiMario,
        Character::CatPeach,
        Character::Toad,
        Character::KoopaTroopa,
        Character::Lakitu,
        Character::Toadette,
        Character::KingBoo,
        Character::PeteyPiranha,
        Character::BabyMario,
        Character::BabyLuigi,
        Character::BabyPeach,
        Character::BabyDaisy,
        Character::BabyRosalina,
        Character::MetalMario(0),
        Character::MetalMario(1),
        Character::PinkGoldPeach,
        Character::Wiggler,
        Character::Wario,
        Character::Waluigi,
        Character::DonkeyKong,
        Character::Bowser,
        Character::DryBones,
        Character::BowserJr,
        Character::DryBowser,
        Character::Kamek,
        Character::Lemmy,
        Character::Larry,
        Character::Wendy,
        Character::Ludwig,
        Character::Iggy,
        Character::Roy,
        Character::Morton,
        Character::Peachette,
        Character::Inkling(0),
        Character::Inkling(1),
        Character::Inkling(2),
        Character::Inkling(3),
        Character::Inkling(4),
        Character::Inkling(5),
        Character::Villager(0),
        Character::Villager(1),
        Character::Isabelle,
        Character::Link(0),
        Character::Link(1),
        Character::DiddyKong,
        Character::FunkyKong,
        Character::Pauline,
        Character::ShyGuy(0),
        Character::ShyGuy(1),
        Character::ShyGuy(2),
        Character::ShyGuy(3),
        Character::ShyGuy(4),
        Character::ShyGuy(5),
        Character::ShyGuy(6),
        Character::ShyGuy(7),
        Character::ShyGuy(8),
        Character::Birdo(0),
        Character::Birdo(1),
        Character::Birdo(2),
        Character::Birdo(3),
        Character::Birdo(4),
        Character::Birdo(5),
        Character::Birdo(6),
        Character::Birdo(7),
        Character::Birdo(8),
        Character::Yoshi(0),
        Character::Yoshi(1),
        Character::Yoshi(2),
        Character::Yoshi(3),
        Character::Yoshi(4),
        Character::Yoshi(5),
        Character::Yoshi(6),
        Character::Yoshi(7),
        Character::Yoshi(8),
        Character::Mii(0, 0),
        Character::Mii(0, 1),
        Character::Mii(0, 2),
        Character::Mii(0, 3),
        Character::Mii(0, 4),
        Character::Mii(0, 5),
        Character::Mii(0, 6),
        Character::Mii(0, 7),
        Character::Mii(0, 8),
        Character::Mii(1, 0),
        Character::Mii(1, 1),
        Character::Mii(1, 2),
        Character::Mii(1, 3),
        Character::Mii(1, 4),
        Character::Mii(1, 5),
        Character::Mii(1, 6),
        Character::Mii(1, 7),
        Character::Mii(1, 8),
        Character::Mii(2, 0),
        Character::Mii(2, 1),
        Character::Mii(2, 2),
        Character::Mii(2, 3),
        Character::Mii(2, 4),
        Character::Mii(2, 5),
        Character::Mii(2, 6),
        Character::Mii(2, 7),
        Character::Mii(2, 8),
        Character::Mii(3, 0),
        Character::Mii(3, 1),
        Character::Mii(3, 2),
        Character::Mii(3, 3),
        Character::Mii(3, 4),
        Character::Mii(3, 5),
        Character::Mii(3, 6),
        Character::Mii(3, 7),
        Character::Mii(3, 8),
        Character::Mii(4, 0),
        Character::Mii(4, 1),
        Character::Mii(4, 2),
        Character::Mii(4, 3),
        Character::Mii(4, 4),
        Character::Mii(4, 5),
        Character::Mii(4, 6),
        Character::Mii(4, 7),
        Character::Mii(4, 8),
        Character::Mii(5, 0),
        Character::Mii(5, 1),
        Character::Mii(5, 2),
        Character::Mii(5, 3),
        Character::Mii(5, 4),
        Character::Mii(5, 5),
        Character::Mii(5, 6),
        Character::Mii(5, 7),
        Character::Mii(5, 8),
        Character::Mii(6, 0),
        Character::Mii(6, 1),
        Character::Mii(6, 2),
        Character::Mii(6, 3),
        Character::Mii(6, 4),
        Character::Mii(6, 5),
        Character::Mii(6, 6),
        Character::Mii(6, 7),
        Character::Mii(6, 8),
        Character::Mii(7, 0),
        Character::Mii(7, 1),
        Character::Mii(7, 2),
        Character::Mii(7, 3),
        Character::Mii(7, 4),
        Character::Mii(7, 5),
        Character::Mii(7, 6),
        Character::Mii(7, 7),
        Character::Mii(7, 8),
        Character::Mii(8, 0),
        Character::Mii(8, 1),
        Character::Mii(8, 2),
        Character::Mii(8, 3),
        Character::Mii(8, 4),
        Character::Mii(8, 5),
        Character::Mii(8, 6),
        Character::Mii(8, 7),
        Character::Mii(8, 8),
  ]}
}

#[derive(PartialEq, Eq,Clone,Copy)]
pub enum Kart{
   StandardKart ,     PipeFrame ,        Mach8 ,            SteelDriver ,     // Karts
   CatCruiser ,       CircuitSpecial ,   TriSpeeder ,       Badwagon ,
   Prancer ,          Biddybuggy ,       Landship ,         Sneeker ,
   SportsCoupe ,      GoldStandard ,     GLA ,              W25SilverArrow ,
   _300SLRoadster ,   BlueFalcon ,       TanookiKart ,      BDasher ,
   Streetle ,         PWing ,            KoopaClown ,
  
   StandardBike ,     Comet ,            SportBike ,        TheDuke ,         // Bikes
   FlameRider ,       Varmint ,          MrScooty ,         JetBike ,
   YoshiBike ,        MasterCycle ,      MasterCycleZero ,  CityTripper ,
   StandardATV ,      WildWiggler ,      TeddyBuggy ,       BoneRattler ,
   SplatBuggy ,       Inkstriker 
}
impl Kart{
  pub fn value(&self)->usize{
    match *self{
        Kart::StandardKart    => 0,
        Kart::PipeFrame       => 1,
        Kart::Mach8           => 2,
        Kart::SteelDriver     => 3,
        Kart::CatCruiser      => 4,
        Kart::CircuitSpecial  => 5,
        Kart::TriSpeeder      => 6,
        Kart::Badwagon        => 7,
        Kart::Prancer         => 8,
        Kart::Biddybuggy      => 9,
        Kart::Landship        => 10,
        Kart::Sneeker         => 11,
        Kart::SportsCoupe     => 12,
        Kart::GoldStandard    => 13,
        Kart::GLA             => 14,
        Kart::W25SilverArrow  => 15,
        Kart::_300SLRoadster  => 16,
        Kart::BlueFalcon      => 17,
        Kart::TanookiKart     => 18,
        Kart::BDasher         => 19,
        Kart::Streetle        => 20,
        Kart::PWing           => 21,
        Kart::KoopaClown      => 22,
        Kart::StandardBike    => 23,
        Kart::Comet           => 24,
        Kart::SportBike       => 25,
        Kart::TheDuke         => 26,
        Kart::FlameRider      => 27,
        Kart::Varmint         => 28,
        Kart::MrScooty        => 29,
        Kart::JetBike         => 30,
        Kart::YoshiBike       => 31,
        Kart::MasterCycle     => 32,
        Kart::MasterCycleZero => 33,
        Kart::CityTripper     => 34,
        Kart::StandardATV     => 35,
        Kart::WildWiggler     => 36,
        Kart::TeddyBuggy      => 37,
        Kart::BoneRattler     => 38,
        Kart::SplatBuggy      => 39,
        Kart::Inkstriker      => 40,
    }
  }
  pub fn from_str(s:&str)->Option<Kart>{
    let n = s.parse::<usize>().unwrap();
    for i in Kart::all(){
      if i.value() == n{
        return Some(i);
      }
    }
    return None;
  }
  fn all() -> Vec<Kart>{
    return vec![   
        Kart::StandardKart,
        Kart::PipeFrame,
        Kart::Mach8,
        Kart::SteelDriver,
        Kart::CatCruiser,
        Kart::CircuitSpecial,
        Kart::TriSpeeder,
        Kart::Badwagon,
        Kart::Prancer,
        Kart::Biddybuggy,
        Kart::Landship,
        Kart::Sneeker,
        Kart::SportsCoupe,
        Kart::GoldStandard,
        Kart::GLA,
        Kart::W25SilverArrow,
        Kart::_300SLRoadster,
        Kart::BlueFalcon,
        Kart::TanookiKart,
        Kart::BDasher,
        Kart::Streetle,
        Kart::PWing,
        Kart::KoopaClown,
        Kart::StandardBike,
        Kart::Comet,
        Kart::SportBike,
        Kart::TheDuke,
        Kart::FlameRider,
        Kart::Varmint,
        Kart::MrScooty,
        Kart::JetBike,
        Kart::YoshiBike,
        Kart::MasterCycle,
        Kart::MasterCycleZero,
        Kart::CityTripper,
        Kart::StandardATV,
        Kart::WildWiggler,
        Kart::TeddyBuggy,
        Kart::BoneRattler,
        Kart::SplatBuggy,
        Kart::Inkstriker,
    ]
  }
}

#[derive(PartialEq, Eq,Clone,Copy)]
pub enum Wheel{
   Standard ,        Monster ,          Roller ,           Slim ,              Slick ,            
   Metal ,           Button ,           OffRoad ,          Sponge ,            Wood ,             Cushion ,
   BlueStandard ,    HotMonster ,       AzureRoller ,      CrimzonSlim ,       CyberSlick ,      
   RetroOffRoad ,    GoldTires ,        GLATires ,         TriforceTires ,     AncientTires ,     LeafTires 
}
impl Wheel{
  pub fn value(&self)->usize{
    match *self{
        Wheel::Standard       => 0,
        Wheel::Monster        => 1,
        Wheel::Roller         => 2,
        Wheel::Slim           => 3,
        Wheel::Slick          => 4,
        Wheel::Metal          => 5,
        Wheel::Button         => 6,
        Wheel::OffRoad        => 7,
        Wheel::Sponge         => 8,
        Wheel::Wood           => 9,
        Wheel::Cushion        => 10,
        Wheel::BlueStandard   => 11,
        Wheel::HotMonster     => 12,
        Wheel::AzureRoller    => 13,
        Wheel::CrimzonSlim    => 14,
        Wheel::CyberSlick     => 15,
        Wheel::RetroOffRoad   => 16,
        Wheel::GoldTires      => 17,
        Wheel::GLATires       => 18,
        Wheel::TriforceTires  => 19,
        Wheel::AncientTires   => 20,
        Wheel::LeafTires      => 21,
    }
  }
  pub fn from_str(s:&str)->Option<Wheel>{
    let n = s.parse::<usize>().unwrap();
    for i in Wheel::all(){
      if i.value() == n{
        return Some(i);
      }
    }
    return None;
  }
  fn all()->Vec<Wheel>{
    return vec![
        Wheel::Standard,
        Wheel::Monster,
        Wheel::Roller,
        Wheel::Slim,
        Wheel::Slick,
        Wheel::Metal,
        Wheel::Button,
        Wheel::OffRoad,
        Wheel::Sponge,
        Wheel::Wood,
        Wheel::Cushion,
        Wheel::BlueStandard,
        Wheel::HotMonster,
        Wheel::AzureRoller,
        Wheel::CrimzonSlim,
        Wheel::CyberSlick,
        Wheel::RetroOffRoad,
        Wheel::GoldTires,
        Wheel::GLATires,
        Wheel::TriforceTires,
        Wheel::AncientTires,
        Wheel::LeafTires,
    ]
  }
}

#[derive(PartialEq, Eq,Clone,Copy)]
pub enum Wing{
   SuperGlider ,      CloudGlider ,      WarioWing ,        WaddleWing ,       PeachParasol ,
   Parachute ,         Parafoil ,        FlowerGlider ,     BowserKite ,       PlaneGlider ,
   MKTVParafoil ,     GoldGlider ,       HylianKite ,       Paraglider ,       PaperGlider 
}
impl Wing{
  pub fn value(&self) -> usize{
    match *self{
        Wing::SuperGlider   => 0,
        Wing::CloudGlider   => 1,
        Wing::WarioWing     => 2,
        Wing::WaddleWing    => 3,
        Wing::PeachParasol  => 4,
        Wing::Parachute     => 5,
        Wing::Parafoil      => 6,
        Wing::FlowerGlider  => 7,
        Wing::BowserKite    => 8,
        Wing::PlaneGlider   => 9,
        Wing::MKTVParafoil  => 10,
        Wing::GoldGlider    => 11,
        Wing::HylianKite    => 12,
        Wing::Paraglider    => 13,
        Wing::PaperGlider   => 14,
    }
  }
  pub fn from_str(s:&str)->Option<Wing>{
    let n = s.parse::<usize>().unwrap();
    for i in Wing::all(){
      if i.value() == n{
        return Some(i);
      }
    }
    return None;
  }
  fn all() -> Vec<Wing>{
    return vec![
        Wing::SuperGlider,
        Wing::CloudGlider,
        Wing::WarioWing,
        Wing::WaddleWing,
        Wing::PeachParasol,
        Wing::Parachute,
        Wing::Parafoil,
        Wing::FlowerGlider,
        Wing::BowserKite,
        Wing::PlaneGlider,
        Wing::MKTVParafoil,
        Wing::GoldGlider,
        Wing::HylianKite,
        Wing::Paraglider,
        Wing::PaperGlider,
    ]
  }
}