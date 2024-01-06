//use std::path::Path;
//use std::io::prelude::*;

//use chrono;


mod mk8tt;
use mk8tt::*;
use mk8tt::consts::*;
mod gui;
use gui::*;

fn main() {
    // let now: String = chrono::offset::Utc::now().to_rfc3339();
    // println!("{}",&now);
    
    // let path = Path::new("./test/t2.txt");
    // let display = path.display();

    // let mut file = match File::create(&path){ 
    //   Err(why)  =>  panic!("couldn't create {}: {}", display, why),
    //   Ok(file)  =>  file
    // };

    // match file.write_all(now.as_bytes()){
    //   Err(why)  =>  panic!("couldn't write to {}: {}",display, why),
    //   Ok(_)     =>  println!("successfully wrote to {}",display),
    // }


  let mut x = Mk8ttFile::new("Bongo");
  x.add_entry(&Track{
    cc: CC::_150,
    track_id: CourseTitle::BigBlue,
  },Entry::new(
    Time{minutes:1,seconds:20,miliseconds:0},
    Character::Daisy,
    Kart::CatCruiser,
    Wheel::Cushion,
    Wing::CloudGlider,
    "LOOK AT ME!!!".to_owned(),
  ));
  x.add_entry(&Track{
    cc: CC::_150,
    track_id: CourseTitle::_3DSNeoBowserCity,
  },Entry::new(
    Time { minutes: 2, seconds: 10, miliseconds: 935 },
    Character::Daisy,
    Kart::CatCruiser,
    Wheel::Cushion,
    Wing::CloudGlider,
    "LOOK AT ME!!!".to_owned(),
  ));
  x.add_entry(&Track{
    cc: CC::_200,
    track_id: CourseTitle::WiiRainbowRoad,
  },Entry::new(
    Time { minutes: 1, seconds: 12, miliseconds: 115 },
    Character::Daisy,
    Kart::CatCruiser,
    Wheel::Cushion,
    Wing::CloudGlider,
    "LOOK AT ME!!!".to_owned(),
  ));
  x.save();

  let y = Mk8ttFile::load("Bongo").unwrap();
  println!("{}",y.to_string());

  
  let mut win = Mk8ttWindow::new(Mk8WinView::MainMenu);

  loop{
    while let Some(event) = win.window.poll_event(){
      match event{
        sfml::window::Event::Closed => return,
        _=>{}
      }
    }
    win.draw();
  }

}
