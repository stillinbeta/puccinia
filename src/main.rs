extern crate cursive;
extern crate puccinia;

use cursive::traits::Identifiable;
use cursive::Cursive;

fn main() {
    let mut siv = Cursive::default();

    siv.add_global_callback('q', |s| s.quit());

    siv.add_fullscreen_layer(puccinia::view::ResizableGrid::default().with_id("grid"));
    siv.add_layer(puccinia::view::help_dialog());
    siv.set_autorefresh(true);

    siv.run();
}
