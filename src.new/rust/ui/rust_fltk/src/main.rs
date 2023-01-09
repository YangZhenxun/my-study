#![windows_subsystem = "windows"]
use fltk::{prelude::*, *};

fn main() {
    let app = app :: App :: default();
    let mut wind = window :: Window :: new(500, 500, 400, 500, "hello");
    let mut _but  = button :: Button :: new(160, 200, 60, 40, "Click me!");
    wind.end();
    wind.show();
    app.run().unwrap();
}
