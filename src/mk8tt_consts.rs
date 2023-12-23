
enum CourseTitle{
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

enum SrdcCategorie{
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

enum ItemsAndCc{
  Items200cc,
  Items150cc,
  NoItems200cc,
  NoItems150cc
}

enum Character{
  Mario ,          Luigi ,            Peach ,           Daisy ,          Rosalina,         TanookiMario ,    CatPeach ,        Birdo(u8) ,            
  Yoshi(u8) ,      Toad ,             KoopaTroopa ,     ShyGuy(u8) ,     Lakitu,           Toadette ,        KingBoo ,         PeteyPiranha ,    
  BabyMario ,      BabyLuigi ,        BabyPeach ,       BabyDaisy ,      BabyRosalina ,    MetalMario(u8) ,  PinkGoldPeach ,   Wiggler ,
  Wario ,          Waluigi ,          DonkeyKong ,      Bowser ,         DryBones ,        BowserJr ,        DryBowser ,       Kamek ,
  Lemmy ,          Larry ,            Wendy ,           Ludwig ,         Iggy ,            Roy ,             Morton ,          Peachette ,
  Inkling(u8) ,    Villager(u8) ,     Isabelle ,        Link(u8) ,       DiddyKong,        FunkyKong ,       Pauline ,         Mii(u8,u8) 
}

enum Kart{
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

enum Wheel{
   Standard ,        Monster ,          Roller ,           Slim ,              Slick ,            
   Metal ,           Button ,           OffRoad ,          Sponge ,            Wood ,             Cushion ,
   BlueStandard ,    HotMonster ,       AzureRoller ,      CrimzonSlim ,       CyberSlick ,      
   RetroOffRoad ,    GoldTires ,        GLATires ,         TriforceTires ,     AncientTires ,     LeafTires 
}

enum Wing{
   SuperGlider ,      CloudGlider ,      WarioWing ,        WaddleWing ,       PeachParasol ,
   Parachute ,         Parafoil ,        FlowerGlider ,     BowserKite ,       PlaneGlider ,
   MKTVParafoil ,     GoldGlider ,       HylianKite ,       Paraglider ,       PaperGlider 
}