// main.rs
use crate::lib::Lib;

mod lib;

fn main() {
    let lib = Lib::init();
    lib.start();
}
