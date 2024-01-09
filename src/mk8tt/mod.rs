use std::path::Path;
use std::fs::{self,File};
use std::io::Write;
use std::io::prelude::*;
use chrono;
use directories::ProjectDirs;

pub mod consts;
use consts::*;

const APP_NAME: &'static str = "Mk8tt";
const ORG_NAME: &'static str = "MDM";
const QUALIFIER: &'static str = "com";


#[derive(PartialEq, Eq)]
pub enum CC{
  _150,
  _200,
  Spdc,
  // TODO add custom
}

pub struct Track{
  pub cc:CC,
  pub track_id:CourseTitle,
}

#[derive(Clone,Copy)]
pub struct Time{
  pub minutes:u8,
  pub seconds:u8,
  pub miliseconds:u16,
}
impl Time{
  pub fn to_string(&self)->String{
    return format!("{}:{}.{}",self.minutes,self.seconds,self.miliseconds);
  }
  pub fn value(&self)->f64{
    return self.minutes as f64*60.+self.seconds as f64+self.miliseconds as f64/1000.;
  }
  pub fn from_string(s:&str)->Option<Time>{
    if s.contains(":") && s.contains("."){
      let mut one = s.split(":").collect::<Vec<&str>>();
      let mut two = one[1].split(".").collect::<Vec<&str>>();
      return Some(Time { minutes: one[0].parse::<u8>().unwrap(), seconds: two[0].parse::<u8>().unwrap(), miliseconds: two[1].parse::<u16>().unwrap() });
    }
    return None;
  }
}

pub struct Entry{
  pub time:Time,
  date:chrono::DateTime<chrono::Utc>,
  pub racer:Character,
  pub kart:Kart,
  pub wheel:Wheel,
  pub wing:Wing,
  pub extra:String,
}
impl Entry{
  pub fn clone(&self)->Entry{
    return Entry { time: self.time, date: self.date, racer: self.racer, kart: self.kart, wheel: self.wheel, wing: self.wing, extra: self.extra.clone() }
  }
  pub fn new(time:Time,racer:Character,kart:Kart,wheel:Wheel,wing:Wing,extra:String)->Self{
    return Entry { time: time, 
      date: chrono::offset::Utc::now(), 
      racer: racer, kart: kart, wheel: wheel, 
      wing: wing, extra: extra }
  }
  pub fn to_string(&self)->String{
    return format!("{};{};{};{};{};{};{};",
      self.time.to_string(),
      self.date,
      self.racer.value(),
      self.kart.value(),
      self.wheel.value(),
      self.wing.value(),
      &self.extra);
  }
  pub fn from_string(s:&str)->Entry{
    let elements = s.split(";").collect::<Vec<&str>>();
    return Entry{ 
      time: Time::from_string(elements[0]).unwrap(), 
      date: elements[1].parse().unwrap(), 
      racer: Character::from_str(elements[2]).unwrap(), 
      kart: Kart::from_str(elements[3]).unwrap(), 
      wheel: Wheel::from_str(elements[4]).unwrap(), 
      wing: Wing::from_str(elements[5]).unwrap(), 
      extra: elements[6].to_owned()
    }
  }
}

pub struct Mk8ttFile{
  filename:String,
  ids150cc:Vec<usize>,
  ids200cc:Vec<usize>,
  ids_spdc:Vec<usize>,
  filelines:Vec<Entry>,
}
impl Mk8ttFile{
  const EXTENTION:&'static str = ".mk8tt";
  const EMPTY_FILE_CONSTANT:usize = 0;
  const NUM_TRACKS:i32 = 96;
  const NUM_SPDC_CATEGORIES:i32 = 36;
}

impl Mk8ttFile{
  pub fn get_visible_filenames() -> Vec<String>{
    let mut out:Vec<String> = vec![];
    if let Some(proj_dirs) = ProjectDirs::from(QUALIFIER,ORG_NAME,APP_NAME){
      if let Ok(path) = fs::read_dir(proj_dirs.data_dir()){
        path.for_each(|entry|{
          let en = entry.unwrap();
          let ep = en.path();
          let file_name = ep.file_name().unwrap();
          let fn_string = file_name.to_str().unwrap();
          out.push(fn_string.to_owned());
        })
      }
    }
    return out;
  }

  fn modify_metadata(&mut self,track:&Track,up:bool){                             // TODO test
    let arr = [&mut self.ids150cc,&mut self.ids200cc,&mut self.ids_spdc];
    let mut starter = track.track_id.value()+1;
    for i in 0..arr.len() {
      if i==0 && !(track.cc == CC::_150) {continue;}
      if i==1 && (track.cc == CC::Spdc) {continue;}
      for j in starter..arr[i].len(){
        if up{
          arr[i][j]+=1;
        }else{
          arr[i][j]-=1;
        }
      }
      starter = 0;
    }
  }
  pub fn to_string(&self)->String {                                               // TODO test
    let mut line1 = "".to_owned();
    for i in &self.ids150cc{
      line1.push_str(&i.to_string());
      line1.push(';');
    }
    let mut line2 = "".to_owned();
    for i in &self.ids200cc{
      line2.push_str(&i.to_string());
      line2.push(';');
    }
    let mut line3 = "".to_owned();
    for i in &self.ids_spdc{
      line3.push_str(&i.to_string());
      line3.push(';');
    }
    let mut out = "".to_owned();
    out.push_str(&line1);
    out.push('\n');
    out.push_str(&line2);
    out.push('\n');
    out.push_str(&line3);
    out.push('\n');
    for i in &self.filelines{
      out.push_str(&i.to_string());
      out.push('\n');
    }
    return out;
  }
  
  pub fn new(file_name:&str)->Self{                                               // TODO test
    let mut fname = file_name.to_owned();
    if !fname.ends_with(Mk8ttFile::EXTENTION){
      fname+=Mk8ttFile::EXTENTION;
    }
    let mut out = Self{
      filename:fname,
      ids150cc:vec![],
      ids200cc:vec![],
      ids_spdc:vec![],
      filelines:vec![]
    };
    for _ in 0..Mk8ttFile::NUM_SPDC_CATEGORIES{
      for vector in [&mut out.ids150cc,&mut out.ids200cc,&mut out.ids_spdc]{
        vector.push(Mk8ttFile::EMPTY_FILE_CONSTANT);
      }
    }
    for _ in Mk8ttFile::NUM_SPDC_CATEGORIES..Mk8ttFile::NUM_TRACKS{
      for vector in [&mut out.ids150cc,&mut out.ids200cc]{
        vector.push(Mk8ttFile:: EMPTY_FILE_CONSTANT);
      }
    }
    return out;
  }

  pub fn get_times(&self,track:&Track)->Vec<Entry>{                               // TODO test
    let begin:usize;
    let end:usize;
    match track.cc{
      CC::_150 =>{
        begin = self.ids150cc[track.track_id.value()];
        if track.track_id==CourseTitle::WiiRainbowRoad{
          end=self.ids200cc[0];
        }else{
          end=self.ids150cc[track.track_id.value()+1];
        }
      },
      CC::_200 =>{
        begin = self.ids200cc[track.track_id.value()];
        if track.track_id==CourseTitle::WiiRainbowRoad{
          end=self.ids_spdc[0];
        }else{
          end=self.ids200cc[track.track_id.value()+1];
        }
      },
      CC::Spdc =>{
        begin = self.ids_spdc[track.track_id.value()];
        if track.track_id==CourseTitle::WiiRainbowRoad{ //TODO this is different for SPDC
          end=self.filelines.len(); //TODO add custom categories
        }else{
          end=self.ids_spdc[track.track_id.value()+1];
        }
      }
    }
    let mut out:Vec<Entry> = vec![];
    for i in begin..end{
      out.push(self.filelines[i].clone());
    }
    return out;
    //TODO test
  }
  pub fn add_entry(&mut self,track:&Track,entry:Entry){                           // TODO test
    let id:usize;
    match track.cc{
      CC::_150=>{
        id = self.ids150cc[track.track_id.value()];
      },
      CC::_200=>{
        id = self.ids200cc[track.track_id.value()];
      },
      CC::Spdc=>{
        id = self.ids_spdc[track.track_id.value()];
      }
    }
    self.filelines.insert(id,entry);
    self.modify_metadata(track,true);

  }
  pub fn get_entry(&self,entry_id:usize)->Option<&Entry>{                         // TODO test
    if entry_id>=self.filelines.len(){
      return None;
    }else{
      return Some(&self.filelines[entry_id]);
    }
    

  }
  pub fn get_id_from_track_and_number(&self,track:&Track,number:usize) -> usize{  // TODO test
    match track.cc{
      CC::_150 =>{
        return self.ids150cc[track.track_id.value()] + number-1;
      }
      CC::_200 =>{
        return self.ids200cc[track.track_id.value()] + number-1;
      }
      CC::Spdc =>{
        return self.ids_spdc[track.track_id.value()] + number-1;
      }
    }
  }
  pub fn edit_entry(&mut self,entry_id:usize,                                     // TODO test
    time:Option<Time>,
    racer:Option<Character>,
    kart:Option<Kart>,
    wheel:Option<Wheel>,
    wing:Option<Wing>,
    extra:Option<String>
  ){ 
    let old_entry = &mut self.filelines[entry_id];
    if let Some(t) = time{
      old_entry.time = t;
    }
    if let Some(r) = racer{
      old_entry.racer = r;
    }
    if let Some(k) = kart{
      old_entry.kart = k;
    }
    if let Some(w) = wheel{
      old_entry.wheel = w;
    }
    if let Some(w) = wing{
      old_entry.wing = w;
    }
    if let Some(e) = extra{
      old_entry.extra = e;
    }
  }
  pub fn delete_entry(&mut self,track:&Track,entry_id:usize)->Entry{              //TODO test
    let deleted = self.filelines.remove(entry_id);
    self.modify_metadata(track, false);
    return deleted;

  }
  pub fn save(&self){                                                             // TODO test
    let path = Path::new(&consts::SAVE_LOCATION).join(&self.filename);
    let mut file = match File::create(&path){
          Err(why2) => {
            panic!("FILE_WRITE_ERROR oh no! {}, {}",why2,path.display());
            
          },
          Ok(f)=>f
        };
    let string= self.to_string();
    match file.write_all(string.as_bytes()){
      Err(why) => println!("FILE_WRITE_ERROR oh no! {} {}", why, path.display()),
      Ok(_)=>{}
    }
  
  }
  
  pub fn load(file_name:&str)->Result<Self,i32>{
    let mut fname = file_name.to_owned();
    if !fname.ends_with(Mk8ttFile::EXTENTION){
      fname+=Mk8ttFile::EXTENTION;
    }
    let path = Path::new(&consts::SAVE_LOCATION).join(&fname);
    let display = path.display();
    let mut file = match File::open(&path){
      Err(why)=> {
        println!("can't open {}: {}",display,why);
        return Err(0);
      },
      Ok(f)=>f
    };
    let mut string = String::new();
    match file.read_to_string(&mut string){
      Err(why) => {
        println!("can't read {}: {}",display,why);
        return Err(1);
      }
      Ok(_)=>{}
    }
    let mut lines = string.split("\n").collect::<Vec<&str>>();
    let cc150 = Mk8ttFile::to_vec(lines[0].split(";").collect::<Vec<&str>>());
    let cc200 = Mk8ttFile::to_vec(lines[1].split(";").collect::<Vec<&str>>());
    let spdc = Mk8ttFile::to_vec(lines[2].split(";").collect::<Vec<&str>>());
    let mut fl = vec![];
    for n in 3..lines.len(){
      if lines[n].len()>=3{ // sometimes the lines have nothing.
        fl.push(Entry::from_string(lines[n]));
      }
    }


    return Ok(Mk8ttFile{
        filename: fname,
        ids150cc: cc150,
        ids200cc: cc200,
        ids_spdc: spdc,
        filelines: fl,
    });

    //TODO test & do better error handling
  }

  fn to_vec(str_arr:Vec<&str>) -> Vec<usize>{
    let mut out = vec![];
    for i in str_arr.iter(){
      if let Ok(x) = i.parse::<usize>(){
        out.push(x);
      }
    }
    return out;
  }

}