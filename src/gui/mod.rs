use std::ffi::FromBytesUntilNulError;

use sfml::{graphics::*,window::*,system::*,SfBox};


// interface
//  there should be a window
//  the window can be swapped between views
//  the window will gather events that have occured to be processed by the main function
//  this gui should be aware of how each element is meant to be shaped and manage things like collision detections 






pub enum Mk8WinElementType{
  RectangleButton(f32,f32),// maybe add image/text options
  CircleButton(f32),
  Rectangle(f32,f32),
  Polygon(Vec<(f32,f32)>),
  Text(String,u32),
}

pub struct Mk8WinElement{
  pub x:f32,
  pub y:f32,
  pub color:Color,
  pub element_type:Mk8WinElementType,
}
impl Mk8WinElement{
  const CIRCLE_POINTS:usize = 32;
  fn draw_on(&self,win:&mut RenderWindow,evs:&mut Vec<Mk8WinEvent>,font:&SfBox<Font>){
    match &self.element_type{
        Mk8WinElementType::RectangleButton(width, height) => Self::draw_rectangle_button(win,evs,self.color,self.x,self.y,*width,*height),
        Mk8WinElementType::CircleButton(radius) => Self::draw_circle_button(win,evs,self.color,self.x,self.y,*radius),
        Mk8WinElementType::Rectangle(width, height) => Self::draw_rectangle(win,self.color,self.x,self.y,*width,*height), // TODO
        Mk8WinElementType::Polygon(points) => Self::draw_polygon(win,self.color,self.x,self.y,points), // TODO
        Mk8WinElementType::Text(text,font_size) => Self::draw_text(win,font,self.color,self.x,self.y,text,*font_size), //TODO
    }
  }
  fn draw_circle_button(win:&mut RenderWindow,evs:&mut Vec<Mk8WinEvent>,color:Color,x:f32,y:f32,radius:f32){
    Self::draw_circle(win, color, x, y, radius);
    // todo add events if needed
  }
  fn draw_circle(win:&mut RenderWindow,color:Color,x:f32,y:f32,radius:f32){
    let mut circle = CircleShape::new(radius,Self::CIRCLE_POINTS);
    circle.set_position((x,y));
    circle.set_fill_color(color);
    win.draw(&circle);
  }
  fn draw_rectangle_button(win:&mut RenderWindow,evs:&mut Vec<Mk8WinEvent>,color:Color,x:f32,y:f32,width:f32,height:f32){
    Self::draw_rectangle(win, color, x, y, width, height);
    //Todo add events if needed
  }
  fn draw_rectangle(win:&mut RenderWindow,color:Color,x:f32,y:f32,width:f32,height:f32){
    let mut rect = RectangleShape::new();
    rect.set_position((x,y));
    rect.set_size((width,height));
    rect.set_fill_color(color);
    win.draw(&rect);
  }
  fn draw_polygon(win:&mut RenderWindow,color:Color,x:f32,y:f32,points:&Vec<(f32,f32)>){
    let mut cs = ConvexShape::new(points.len());
    for i in 0..points.len(){
      cs.set_point(i,(points[i].0+x,points[i].1+y));
    }
    cs.set_fill_color(color);
    win.draw(&cs);
  }
  fn draw_text(win:&mut RenderWindow,font:&SfBox<Font>,color:Color,x:f32,y:f32,text:&String,font_size:u32){
    let mut text_element = Text::new(text,font,font_size);
    text_element.set_position((x,y));
    text_element.set_fill_color(color);
    win.draw(&text_element);
  }

}

pub enum Mk8WinEvent{

}

pub enum Mk8WinView{
  Testing,
  MainMenu,
  SelectPage,
  TrackSelectPage,
  FileEditingPage
}
impl Mk8WinView{

  const CLOUD:Color = Color::rgb(0xd2,0xe6,0xec);
  const SKY:Color = Color::rgb(0xf8,0xc5,0xb8);
  const LAND:Color = Color::rgb(0xaf,0x96,0xa8);
  const LIGHT_SEA:Color=Color::rgb(0x17,0x51,0x82);
  const MID_SEA:Color=Color::rgb(0x14,0x2f,0x57);
  const DEEP_SEA:Color=Color::rgb(0x01,0x14,0x2d);

  const HEADER_SIZE:u32 = 40;

  const MAIN_MENU_HEADER_X1:f32 = 270.;
  const MAIN_MENU_HEADER_Y1:f32 = 30.;
  const MAIN_MENU_HEADER_X2:f32 = 285.;
  const MAIN_MENU_HEADER_Y2:f32 = 70.;
  const MAIN_MENU_BACKSHADOW_OFFSET:f32=2.;
  const MAIN_MENU_BUTTON1_POS:(f32,f32) = (20.,150.);
  const MAIN_MENU_BUTTON2_POS:(f32,f32) = (20.,250.);
  const MAIN_MENU_BUTTON3_POS:(f32,f32) = (20.,350.);

  const MAIN_MENU_30DEG_POLYGON:&'static [(f32,f32)]=&[
    (0.,0.),(150.,0.),(400.,500.),(0.,500.)
  ];
  const MAIN_MENU_SMALL_BUTTON:&'static [(f32,f32)]=&[
    (0.,0.),(25.,-40.),(225.,-40.),(250.,0.),(225.,40.),(25.,40.)
  ];
  fn get_elements_from(v:&Mk8WinView) -> Vec<Mk8WinElement>{
    match v{
        Mk8WinView::Testing => vec![
          Mk8WinElement{x:10.,y:10.,color:Color::RED,element_type:Mk8WinElementType::RectangleButton(300., 100.)},
          Mk8WinElement{x:200.,y:200.,color:Color::RED,element_type:Mk8WinElementType::CircleButton(30.)}
        ],
        Mk8WinView::MainMenu => vec![
          Mk8WinElement{x:0.,y:0.,color:Self::DEEP_SEA,element_type:Mk8WinElementType::Rectangle(800., 800.)},//background
          Mk8WinElement{x:0.,y:0.,color:Self::MID_SEA,element_type:Mk8WinElementType::Rectangle(800., 150.)},// header
          Mk8WinElement{x:Self::MAIN_MENU_BACKSHADOW_OFFSET+Self::MAIN_MENU_HEADER_X1,
            y:Self::MAIN_MENU_HEADER_Y1+Self::MAIN_MENU_BACKSHADOW_OFFSET,
            color:Self::LAND,element_type:Mk8WinElementType::Text("Mario Kart 8 Deluxe".to_owned(),Self::HEADER_SIZE)}, //backshadow for "Mario Kart 8 Deluxe"
          Mk8WinElement{x:Self::MAIN_MENU_HEADER_X2+Self::MAIN_MENU_BACKSHADOW_OFFSET,
            y:Self::MAIN_MENU_HEADER_Y2+Self::MAIN_MENU_BACKSHADOW_OFFSET,
            color:Self::LAND,element_type:Mk8WinElementType::Text("Time Tracker App".to_owned(), Self::HEADER_SIZE)}, //backshadow for "Time Tracker App"
          Mk8WinElement{x:Self::MAIN_MENU_HEADER_X1,y:Self::MAIN_MENU_HEADER_Y1,
            color:Self::SKY,element_type:Mk8WinElementType::Text("Mario Kart 8 Deluxe".to_owned(),Self::HEADER_SIZE)}, //text for "Mario Kart 8 Deluxe"
          Mk8WinElement{x:Self::MAIN_MENU_HEADER_X2,y:Self::MAIN_MENU_HEADER_Y2,
            color:Self::SKY,element_type:Mk8WinElementType::Text("Time Tracker App".to_owned(), Self::HEADER_SIZE)}, //text for "Time Tracker App"
          // fancy graph element?
          Mk8WinElement{x:0.,y:0.,color:Self::LIGHT_SEA,element_type:Mk8WinElementType::Polygon(Self::MAIN_MENU_30DEG_POLYGON.to_vec())},// Polygon for side bar
          Mk8WinElement{x:Self::MAIN_MENU_BUTTON1_POS.0,y:Self::MAIN_MENU_BUTTON1_POS.1,color:Self::LAND,element_type:Mk8WinElementType::Polygon(Self::MAIN_MENU_SMALL_BUTTON.to_vec())},// diamond for button1
          Mk8WinElement{x:Self::MAIN_MENU_BUTTON2_POS.0,y:Self::MAIN_MENU_BUTTON2_POS.1,color:Self::LAND,element_type:Mk8WinElementType::Polygon(Self::MAIN_MENU_SMALL_BUTTON.to_vec())},// diamond for button2
          Mk8WinElement{x:Self::MAIN_MENU_BUTTON3_POS.0,y:Self::MAIN_MENU_BUTTON3_POS.1,color:Self::LAND,element_type:Mk8WinElementType::Polygon(Self::MAIN_MENU_SMALL_BUTTON.to_vec())},// diamond for button3
          // text for button1 
          // text for button2
          // text for button3  

        ],
        Mk8WinView::SelectPage => vec![
          Mk8WinElement{x:0.,y:0.,color:Self::LIGHT_SEA,element_type:Mk8WinElementType::Rectangle(800.,800.)},// background
          //up    // top button
          //down  // bottom button
          //left  // info ui element
          //ccText // ccText
          //newBtn // button for adding categories
          //scrollBar // bar to scroll through cups
          // cups (8-12 cup elements that can be rearranged depending on the state of the page)
          // context menu (conditionally rendered context menu)
          // track buttons (4 track buttons to appear on context menu)
          // tracks (4 track labels to appear on track buttons in context menu)   %PROGRAMMING_STUFF%
        ],
        Mk8WinView::TrackSelectPage => todo!(),
        Mk8WinView::FileEditingPage => todo!(),
    }
  }
}

pub struct Mk8ttWindow{
  pub window:RenderWindow,
  pub font:SfBox<Font>,
  pub elements:Vec<Mk8WinElement>,
  pub events:Vec<Mk8WinEvent>,
  pub current_view:Mk8WinView
}

impl Mk8ttWindow{
  pub fn new(view:Mk8WinView)->Self{
    let font = match Font::from_file("C:/Windows/Fonts/BIZ-UDGothicB.ttc"){
        Some(f) => f,
        None => match Font::from_file("C:/Windows/Fonts/arial.ttf"){
            Some(f) => f,
            None => panic!("Failed To match Font"), //TODO maybe support other platforms?
        },
    };

    return Mk8ttWindow{
      window: RenderWindow::new(
        (800,500),
        "Mario Kart 8 Deluxe Time Tracker",
        Style::CLOSE,
        &Default::default()
      ),
      font: font,
      elements:Mk8WinView::get_elements_from(&view),
      events:vec![],
      current_view:view
    }
  }

  pub fn draw(&mut self){
    self.window.clear(Color::BLUE);
    for i in 0..self.elements.len(){
      self.elements[i].draw_on(&mut self.window,&mut self.events,&self.font);
    }
    self.window.display();
  }
}