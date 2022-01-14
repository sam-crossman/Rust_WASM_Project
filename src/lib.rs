use wasm_bindgen::prelude::*;
//use web_sys::console;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
// #[wasm_bindgen(start)]
// pub fn main_js() -> Result<(), JsValue> {
//     // This provides better error messages in debug mode.
//     // It's disabled in release mode so it doesn't bloat up the file size.
//     #[cfg(debug_assertions)]
//     console_error_panic_hook::set_once();


//     // Your code goes here!
//     console::log_1(&JsValue::from_str("Hello world!"));

//     Ok(())
// }
#[derive(Clone, Copy)]
struct Rgb {
    r:u8,
    g:u8,
    b:u8,
}
#[derive(Clone, Copy)]
enum Element {
    Nothing,
    Sand,
    Water,
}
#[derive(Clone, Copy)]
struct Particle{
    element: Element,
    x:usize,
    y:usize, 
}
impl Particle {
    pub fn x(&self) -> usize {
        self.x
    }
    pub fn y(&self) -> usize {
        self.y
    }  
    pub fn element(&self) -> Element {
        self.element
    }
    pub fn color(&self) -> Rgb {
        match self.element{
            Element::Nothing => Rgb { r: 255, g: 255, b: 255 },
            Element::Sand => Rgb { r: 204, g: 102, b: 0 },
            Element::Water => Rgb { r: 0, g: 102, b: 204 }, 
        }
    }
}
#[wasm_bindgen]
pub struct Image{
    width: usize,
    height: usize,
    cell_size: usize,
    cells: Vec<Rgb>,
    particles: Vec<Vec<Particle>>,
}

#[wasm_bindgen]
impl Image {
    #[wasm_bindgen(constructor)]
    pub fn new(width: usize, height: usize, cell_size:usize) -> Image{
        let cells = vec![Rgb { r: 200, g: 200, b: 200 }; width * height];
        let mut particles = vec![vec![Particle{element:Element::Nothing, x:0, y:0}; height]; width];
        for y in 0..height {
            for x in 0..width{
                particles[x][y].x = x;
                particles[x][y].y = y;
            }
        } 
        Image {width, height, cell_size, cells, particles} 
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn cell_size(&self) -> usize {
        self.cell_size
    }
    pub fn cells(&self) -> Vec<u8>{
        self.cells
        .iter()
        .map(|&rgb| vec![rgb.r, rgb.g, rgb.b])
        .collect::<Vec<Vec<u8>>>()
        .concat()
    }

    pub fn brush(&mut self, x:usize, y:usize, color:Vec<u8>){
        let index = (y*self.width)+x;
        self.cells[index] = Rgb{r:color[0], g:color[1], b:color[2]};
    }
    pub fn add_particle(&mut self, x:usize, y:usize, input_element:&str){
        let current = self.particles[x][y];
        let element = match input_element{
            "sand" => Element::Sand,
            "water" => Element::Water,
            _ => Element::Sand,
        };
        match current.element{
            Element::Nothing => self.particles[x][y].element = element,
            _ => {},
        }
    }
    pub fn update_color(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = (y*self.width)+x;
                self.cells[index] = self.particles[x][y].color();
            }
        }
    }
    pub fn update_particle(&mut self){
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let current = self.particles[x][y];
                match current.element() {
                    Element::Nothing =>{},
                    Element::Sand => {
                        if y+1 < self.height {
                            match self.particles[x][y+1].element { // MATCH DOWN
                                Element::Nothing => {
                                    self.particles[x][y].element = Element::Nothing;
                                    self.particles[x][y+1].element = Element::Sand;
                                }, 
                                Element::Sand => {
                                    if x != 0 {
                                        match self.particles[x-1][y+1].element {  //MATCH DOWN_LEFT
                                            Element::Nothing => {
                                                self.particles[x][y].element = Element::Nothing;
                                                self.particles[x-1][y+1].element = Element::Sand;
                                            }, 
                                            Element::Sand => {
                                                if x + 1 < self.width {
                                                    match self.particles[x+1][y+1].element {  // MATCH DOWN_RIGHT
                                                        Element::Nothing => {
                                                            self.particles[x][y].element = Element::Nothing;
                                                            self.particles[x+1][y+1].element = Element::Sand;
                                                        }, 
                                                        Element::Sand => {},
                                                        Element::Water => {
                                                            self.particles[x][y].element = Element::Water;
                                                            self.particles[x+1][y+1].element = Element::Sand;
                                                        },
                                                    }
                                                }
                                            },
                                            Element::Water => {
                                                self.particles[x][y].element = Element::Water;
                                                self.particles[x-1][y+1].element = Element::Sand;
                                            },
                                        }
                                    }
                                },
                                Element::Water => {
                                    self.particles[x][y].element = Element::Water;
                                    self.particles[x][y+1].element = Element::Sand;
                                },
                            }
                        }
                    },
                    Element::Water => {
                        if y+1 < self.height {
                            if let Element::Nothing = self.particles[x][y+1].element { //below 
                                self.particles[x][y].element = Element::Nothing;
                                self.particles[x][y+1].element = Element::Water;
                            } else {
                                if (x != 0) && (x + 1 < self.width) {
                                    if let Element::Nothing = self.particles[x-1][y+1].element {
                                        self.particles[x][y].element = Element::Nothing;
                                        self.particles[x-1][y+1].element = Element::Water;
                                    } else if let Element::Nothing = self.particles[x+1][y+1].element {
                                        self.particles[x][y].element = Element::Nothing;
                                        self.particles[x+1][y+1].element = Element::Water;
                                    } else if let Element::Nothing = self.particles[x+1][y].element {
                                        self.particles[x][y].element = Element::Nothing;
                                        self.particles[x+1][y].element = Element::Water;
                                    } else if let Element::Nothing = self.particles[x-1][y].element {
                                        self.particles[x][y].element = Element::Nothing;
                                        self.particles[x-1][y].element = Element::Water;
                                    }
                                }
                            } 
                        }
                    },
                } 
            }
        } 
    }
}