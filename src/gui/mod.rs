use sfml::{graphics::*,window::*,system::*};


// interface
//  there should be a window
//  the window can be swapped between views
//  the window will gather events that have occured to be processed by the main function
//  this gui should be aware of how each element is meant to be shaped and manage things like collision detections 


pub enum Mk8WinElementType{
  RectangleButton(f32,f32),// maybe add image/text options
  CircleButton(f32),
}

pub struct MK8WinElement{
  pub x:f32,
  pub y:f32,
  pub element_type:Mk8WinElementType,
}
impl MK8WinElement{
  const CIRCLE_POINTS:usize = 32;
  fn draw_on(&self,win:&mut RenderWindow,evs:&mut Vec<MK8WinEvent>){
    match self.element_type{
        Mk8WinElementType::RectangleButton(width, height) => Self::draw_rectangle(win,evs,self.x,self.y,width,height),
        Mk8WinElementType::CircleButton(radius) => Self::draw_circle(win,evs,self.x,self.y,radius),
    }
  }
  fn draw_circle(win:&mut RenderWindow,evs:&mut Vec<MK8WinEvent>,x:f32,y:f32,radius:f32){
    let mut circle = CircleShape::new(radius,Self::CIRCLE_POINTS);
    circle.set_position((x,y));
    circle.set_fill_color(Color::RED);
    
    win.draw(&circle);
  }
  fn draw_rectangle(win:&mut RenderWindow,evs:&mut Vec<MK8WinEvent>,x:f32,y:f32,width:f32,height:f32){
    let mut rect = RectangleShape::new();
    rect.set_position((x,y));
    rect.set_size((width,height));
    rect.set_fill_color(Color::RED);
    win.draw(&rect);
  }
}

pub enum MK8WinEvent{

}

pub enum MK8WinView{
  Testing
}
impl MK8WinView{
  fn get_elements_from(v:&MK8WinView) -> Vec<MK8WinElement>{
    match v{
        MK8WinView::Testing => vec![
          MK8WinElement{x:10.,y:10.,element_type:Mk8WinElementType::RectangleButton(300., 100.)},
          MK8WinElement{x:200.,y:200.,element_type:Mk8WinElementType::CircleButton(30.)}
        ],
    }
  }
}

pub struct Mk8ttWindow{
  pub window:RenderWindow,
  pub elements:Vec<MK8WinElement>,
  pub events:Vec<MK8WinEvent>,
  pub current_view:MK8WinView
}

impl Mk8ttWindow{
  pub fn new(view:MK8WinView)->Self{
    return Mk8ttWindow{
      window: RenderWindow::new(
        (800,500),
        "Mario Kart 8 Deluxe Time Tracker",
        Style::CLOSE,
        &Default::default()
      ),
      elements:MK8WinView::get_elements_from(&view),
      events:vec![],
      current_view:view
    }
  }

  pub fn draw(&mut self){
    self.window.clear(Color::BLUE);
    for i in 0..self.elements.len(){
      self.elements[i].draw_on(&mut self.window,&mut self.events);
    }
    self.window.display();
  }
}