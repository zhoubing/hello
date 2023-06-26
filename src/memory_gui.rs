use piston_window::*;

pub fn run() {
    let (width, height) = (1280.0, 960.0);
    let window: PistonWindow = WindowSettings::new("Particles", [width, height]).exit_on_esc(true).build().expect("Not Create Window!!!");
    
}