//FBTest: Program to test Rust port of MiniFB
//(c) William Eustace 2017; MIT licensed.
//Core buffer setup code is from Daniel Collin's MiniFB readme. https://github.com/emoon/rust_minifb

extern crate minifb;
use minifb::{Key,MouseMode,MouseButton, WindowOptions, Window};

const WIDTH:usize = 1024;
const HEIGHT:usize=768;

const N_BARS:usize = 16;
const ACC_RATE:f32 = 12f32; //rate at which bars accelerate when mouse above them, pixel/s
const CENTRE_HEIGHT:usize=256;
const SPRING_RATE:f32 = 0.06f32;//spring constant divided by mass
const DAMPING:f32 = 0.07;
/// Represents one `Bar` as displayed on screen
struct Bar{
    width:usize,
    lpos:usize,
    height:usize,
    velocity:f32, //v is in units of pixels per timestep
    colour:u32,
    debug:bool
}
impl Bar {
    /// Draws the `Bar` it's called upon in the `buffer` it's passed.
    fn draw_bar(&self,buffer:&mut Vec<u32>){
        for yval in (HEIGHT-self.height)..HEIGHT {
            for xval in self.lpos..(self.lpos+self.width) {
                buffer[xval+WIDTH*yval] = self.colour;
            }
        }
    }
    /// Updates height and velocity of the bar depending on mouse position etc.
    fn update_bar(&mut self,mouse_xpos:f32, mousedown:bool){
        let mut height:isize = self.height as isize;//Cast to isize first, in case this ends up going negative
        if self.debug {
            println!("{}",height);
        }
        height = ((height as f32) + self.velocity) as isize;//velocity is in units of pixels/timestep, so just add
        if mouse_xpos < (self.lpos + self.width) as f32 && mouse_xpos> self.lpos as f32 && mousedown {
            self.velocity += ACC_RATE; //the bar accelerates if the cursor is in its area on screen
        }
        else {self.velocity -= ((height as f32) - (CENTRE_HEIGHT as f32)) * SPRING_RATE ;} //spring force
        self.velocity -= self.velocity * DAMPING;//apply some damping
        if height > HEIGHT as isize { //now bounds check and cast height back to usize
            height = HEIGHT as isize;
        }
        else if height < 0isize {
            height = 0isize;
        }
        self.height =height as usize;
    }
}


fn main(){
    let mut xpos:f32 = 0f32; //mouse x position
    let mut buffer: Vec<u32> = vec![0;WIDTH * HEIGHT];//the mystical frame buffer!
    let bar_width = (WIDTH / N_BARS) as usize;
    let mut window = Window::new("weirdbars",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default())
                                 .unwrap_or_else(|e| { panic!("{}",e);});
//please excuse this rather messy instantiation... will be automated one day, maybe...
 let mut bars:Vec<Bar> = vec![Bar{width : bar_width, lpos : 0, height: CENTRE_HEIGHT, velocity:0f32, colour: 0x0000CC, debug:false},
 Bar{width : bar_width, lpos : bar_width*1, height: CENTRE_HEIGHT, velocity:0f32, colour: 0xCC00CC, debug:false},
 Bar{width : bar_width, lpos : bar_width*2, height: CENTRE_HEIGHT, velocity:0f32, colour: 0x99004C, debug:false},
 Bar{width : bar_width, lpos : bar_width*3, height: CENTRE_HEIGHT, velocity:0f32, colour: 0x00CC66, debug:false},
 Bar{width : bar_width, lpos : bar_width*4, height: CENTRE_HEIGHT, velocity:0f32, colour: 0x0000FF, debug:false},
 Bar{width : bar_width, lpos : bar_width*5, height: CENTRE_HEIGHT, velocity:0f32, colour: 0x00FF00, debug:false},
 Bar{width : bar_width, lpos : bar_width*6, height: CENTRE_HEIGHT, velocity:0f32, colour: 0x808080, debug:false},
 Bar{width : bar_width, lpos : bar_width*7, height: CENTRE_HEIGHT, velocity:0f32, colour: 0x009900, debug:false},
 Bar{width : bar_width, lpos : bar_width*8, height: CENTRE_HEIGHT, velocity:0f32, colour: 0xCCCC00, debug:false},
 Bar{width : bar_width, lpos : bar_width*9, height: CENTRE_HEIGHT, velocity:0f32, colour: 0x009900, debug:false},
 Bar{width : bar_width, lpos : bar_width*10, height: CENTRE_HEIGHT, velocity:0f32, colour: 0x808080, debug:false},
 Bar{width : bar_width, lpos : bar_width*11, height: CENTRE_HEIGHT, velocity:0f32, colour: 0x00FF00, debug:false},
 Bar{width : bar_width, lpos : bar_width*12, height: CENTRE_HEIGHT, velocity:0f32, colour: 0x0000FF, debug:false},
 Bar{width : bar_width, lpos : bar_width*13, height: CENTRE_HEIGHT, velocity:0f32, colour: 0x00CC66, debug:false},
 Bar{width : bar_width, lpos : bar_width*14, height: CENTRE_HEIGHT, velocity:0f32, colour: 0x99004C, debug:false},
 Bar{width : bar_width, lpos : bar_width*15, height: CENTRE_HEIGHT, velocity:0f32, colour: 0xCC00CC, debug:false}];

    while window.is_open() && !window.is_key_down(Key::Escape) {

     for i in buffer.iter_mut() {//clear buffer to erase bars that are retracting etc.
         *i = 0;
     }
     match window.get_mouse_pos(MouseMode::Discard) {
         Some(r) => xpos = r.0 ,
         None => {},
     }

     for bar in bars.iter(){
         bar.draw_bar(&mut buffer);
     }
     let mousedown:bool = window.get_mouse_down(MouseButton::Left);
     for bar in bars.iter_mut(){//should really try and run this independently of the graphics...
         bar.update_bar(xpos,mousedown);
     }


     // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
     window.update_with_buffer(&buffer).unwrap(); //(courtesy of the MiniFB readme)
    }
}
