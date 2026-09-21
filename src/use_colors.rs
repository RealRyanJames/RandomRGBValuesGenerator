use system_pause::pause;
use crate::colors;


pub fn setup() {
    for (color, id) in &colors::colors::get_r() {
        println!("ID: {}, Value: {}", id, color);
    }

    for (color, id) in &colors::colors::get_g() {
        println!("ID: {}, Value: {}", id, color)
    }

    for (color, id) in &colors::colors::get_b() {
        println!("ID: {}, Value: {}", id, color)
    }

    pause!()
}