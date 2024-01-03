use sfml::{graphics::*,window::*,system::*};


// interface
//  there should be a window
//  the window can be swapped between views
//  the window will gather events that have occured to be processed by the main function
//  this gui should be aware of how each element is meant to be shaped and manage things like collision detections 


pub struct Mk8ttWindow{
  pub window:RenderWindow
}

impl Mk8ttWindow{
  pub fn new()->Self{
    return Mk8ttWindow{
      window: RenderWindow::new(
        (800,500),
        "Mario Kart 8 Deluxe Time Tracker",
        Style::CLOSE,
        &Default::default()
      )
    }
  }

  pub fn draw(&mut self){
    self.window.clear(Color::BLUE);
    self.window.display();
  }
}