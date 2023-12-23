use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

use chrono;

fn main() {
    let now: String = chrono::offset::Utc::now().to_rfc3339();
    println!("{}",&now);
    
    let path = Path::new("./test/t2.txt");
    let display = path.display();

    let mut file = match File::create(&path){ 
      Err(why)  =>  panic!("couldn't create {}: {}", display, why),
      Ok(file)  =>  file
    };

    match file.write_all(now.as_bytes()){
      Err(why)  =>  panic!("couldn't write to {}: {}",display, why),
      Ok(_)     =>  println!("successfully wrote to {}",display),
    }
}
