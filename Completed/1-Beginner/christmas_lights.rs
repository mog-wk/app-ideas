use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

const WINDOW_WIDTH: u32 = 600;
const WINDOW_HEIGHT: u32 = 400;
const FREQUENCY: u64 = 180;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let video_subsystem = sdl_context.video().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("Christmas lights", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position(0, 0)
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut render = Render::new(window.into_canvas().build().unwrap_or_else(|err| {
        println!("Failed to create render: {:?}", err);
        println!("Exiting...");
        std::process::exit(1);
    }));

    let mut event_pump = sdl_context.event_pump().unwrap_or_else(|err| {
        println!("Error in creating event pump: {:?}", err);
        println!("Exiting...");
        std::process::exit(1);
    });

    let mut step: u32 = 1;
    let mut paused: bool = true;

    let mut lights = Lights::new();

    'run: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'run,
                Event::KeyDown {
                    keycode: Some(Keycode::N),
                    ..
                } => step += 1,
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => paused = !paused,
                _ => (),
            }
        }

        if step >= 1 || !paused {
            // render stuff
            render.update(&mut lights).unwrap_or_else(|err| {
                println!("Error in render update: {:?}", err);
                std::process::exit(1);
            });
            if step > 0 {
                step -= 1;
            }
            println!("{} {}", step, paused);
        }

        if !paused {
            std::thread::sleep(std::time::Duration::from_millis(FREQUENCY));
        }
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_u32 / 120));
    }
    Ok(())
}

struct Lights {
    state: Box<[bool]>,
}

impl Lights {
    fn new() -> Self {
        let mut state = [false; 7];
        for i in (0..7).step_by(2) {
            state[i] = true;
        }

        Self {
            state: Box::new(state),
        }
    }

    fn rev(&mut self) {
        for i in self.state.iter_mut() {
            *i = !*i;
        }
    }
}

struct Render {
    canvas: Canvas<Window>,
}

impl Render {
    fn new(canvas: Canvas<Window>) -> Self {
        Self { canvas }
    }

    fn update(&mut self, lights: &mut Lights) -> Result<(), String> {
        self.canvas.set_draw_color(Color::RGB(127, 127, 127));
        self.canvas.clear();

        // render lights
        for (i, light) in lights.state.iter().enumerate() {
            self.process_lights(i as i32, light)?;
        }
        lights.rev();

        self.canvas.present();

        Ok(())
    }
    fn process_lights(&mut self, padding: i32, state: &bool) -> Result<(), String> {
        let x = 60 + (padding + 1) * 60;
        let y = 170;
        let r = 30;
        let d = r * 2;
        //let pos = sdl2::rect::Rect::new(x, y, 30, 30);
        self.canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
        if *state == true {
            //self.canvas.fill_rect(pos).unwrap();
            self.draw_circle_filled(x, y, r, d)?;
        } else {
            //self.canvas.draw_rect(pos);
            self.draw_circle_hollow(x, y, r, d)?;
        }
        Ok(())
    }
    fn draw_circle_filled(&mut self, x: i32, y: i32, r: i32, d: i32) -> Result<(), String> {
        for i in 0..d {
            for j in 0..d {
                let dx = r - i;
                let dy = r - j;
                if (dx * dx + dy * dy) <= (r * r) {
                    self.canvas
                        .draw_point(sdl2::rect::Point::new(x + dx, y + dy))?;
                }
            }
        }
        Ok(())
    }
    fn draw_circle_hollow(&mut self, x: i32, y: i32, r: i32, diameter: i32) -> Result<(), String> {
        let mut dx = r - 1;
        let mut dy = 0;

        let mut tx = 1;
        let mut ty = 1;

        let mut err = tx - diameter;
        while dx >= dy {
            self.canvas.draw_point(sdl2::rect::Point::new(x + dx, y + dy))?;
            self.canvas.draw_point(sdl2::rect::Point::new(x + dx, y - dy))?;
            self.canvas.draw_point(sdl2::rect::Point::new(x - dx, y + dy))?;
            self.canvas.draw_point(sdl2::rect::Point::new(x - dx, y - dy))?;
            self.canvas.draw_point(sdl2::rect::Point::new(x + dy, y + dx))?;
            self.canvas.draw_point(sdl2::rect::Point::new(x + dy, y - dx))?;
            self.canvas.draw_point(sdl2::rect::Point::new(x - dy, y + dx))?;
            self.canvas.draw_point(sdl2::rect::Point::new(x - dy, y - dx))?;

            if err <= 0 {
                dy += 1;
                err += ty;
                ty += 2;
            }
            if err > 0 {
                dx -= 1;
                tx += 2;
                err += tx - diameter;
            }
        }
        Ok(())
    }
}
