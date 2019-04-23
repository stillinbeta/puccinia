extern crate cursive;
extern crate puccinia;

use cursive::Cursive;

fn main() {
    let mut siv = Cursive::termion().expect("Failed to open terminal");

    puccinia::setup_cursive(&mut siv);
    siv.run();
}
