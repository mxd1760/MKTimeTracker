use chrono;
mod mk8tt_consts;

enum CC{
  _150,
  _200,
  Spdc,
  // TODO add custom
}

struct Track{
  cc:CC,
  track_id:CourseTitle,
}

struct Entry{
  time:String,
  date:chrono::DateTime<chrono::Utc>,
  racer:Character,
  kart:Kart,
  wheel:Wheel,
  wing:Wing,
  extra:String,
}

struct Mk8ttFile{
  filename:String,
  ids150cc:Vec<i32>,
  ids200cc:Vec<i32>,
  ids_spdc:Vec<i32>,
  filelines:Vec<Entry>,
}

impl Mk8ttFile{
  fn load(filename:String)->Self{
    //TODO
  }
  fn get_times(&self,track:&Track)->Vec<Entry>{
    //TODO
  }
  fn add_track_entry(&self,track:&Track,entry:Entry){
    //TODO
  }
  fn get_track_entry(&self,track:&Track,entry_id:i32){
    //TODO
  }
  fn replace_entry(&self,track:&Track,entry_id:i32,new_entry:Entry){
    //TODO
  }
  fn edit_entry(&self,track:&Track,entry_id:i32,
    time:Option<String>,
    racer:Option<Character>,
    kart:Option<Kart>,
    wheel:Option<Wheel>,
    wing:Option<Wing>,
    extra:Option<String>,
  ){
    //TODO
  }
  fn delete_entry(&self,track:&Track,entry_id:i32){
    //TODO
  }
  fn save(&self){

  }


}