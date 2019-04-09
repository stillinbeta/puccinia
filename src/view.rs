extern crate cursive;

use crate::{Cell, Grid};
use cursive::event::{Callback, Event, EventResult, MouseEvent};
use cursive::traits::Identifiable;
use cursive::vec::Vec2;
use cursive::view::View;
use cursive::views::{Dialog, LinearLayout, ListView, PaddedView, SliderView, TextView, ViewRef};

pub struct ResizableGrid {
    grid: Grid,
    pause: bool,
}

impl Default for ResizableGrid {
    fn default() -> Self {
        ResizableGrid {
            grid: Grid { grid: vec![] },
            pause: true,
        }
    }
}

const SLIDER_SIZE: usize = 20;

pub fn help_dialog() -> Box<dyn View> {
    let mut layout = ListView::new();
    layout.add_child("h", TextView::new("Display this help menu"));
    layout.add_child("r", TextView::new("Reset the simulation"));
    layout.add_child("q", TextView::new("quit"));
    layout.add_child("space", TextView::new("Pause the simulation"));
    layout.add_child("mouse click", TextView::new("Toggle a cell"));
    Box::new(
        Dialog::around(layout)
            .button("OK", |siv| {
                siv.find_id("grid").map(|mut view: ViewRef<ResizableGrid>| {
                    view.pause = false;
                });
                siv.pop_layer();
            })
            .title("Welcome to the game of life"),
    )
}

pub fn reset_dialog() -> Box<dyn View> {
    let slider = SliderView::horizontal(SLIDER_SIZE)
        .on_change(|siv, value| {
            siv.find_id("percentage")
                .map(|mut view: ViewRef<TextView>| {
                    view.set_content(format!("{}%", value * 100 / SLIDER_SIZE))
                });
            siv.set_user_data(value);
        })
        .with_id("slider");
    let mut linear = LinearLayout::vertical();
    linear.add_child(TextView::new("Probability a cell is alive:"));
    let mut row = LinearLayout::horizontal();
    let percent = PaddedView::new(
        ((2, 0), (0, 0)),
        TextView::new("0%").center().with_id("percentage"),
    );

    row.add_child(slider);
    row.add_child(percent);
    row.set_weight(1, 20);
    linear.add_child(PaddedView::new(((0, 0), (1, 0)), row));

    let dialog = Dialog::around(linear)
        .dismiss_button("Cancel")
        .button("OK", |siv| {
            let percent = siv.user_data::<usize>().map(|p| *p).unwrap_or(0);
            let probability = (percent as f64) / (SLIDER_SIZE as f64);

            siv.find_id("grid").map(|mut view: ViewRef<ResizableGrid>| {
                view.reset(probability);
                view.pause = false;
            });
            siv.pop_layer();
        })
        .title("Create a new grid?");
    Box::new(dialog)
}

impl ResizableGrid {
    fn as_rows(&self) -> Vec<Vec<String>> {
        self.grid
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| match cell {
                        Cell::Alive => "*".into(),
                        Cell::Dead => ".".into(),
                    })
                    .collect()
            })
            .collect()
    }

    fn reset(&mut self, probability: f64) {
        let (x, y) = self.grid.size();
        self.grid = Grid::random(x, y, probability);
    }
}

impl View for ResizableGrid {
    fn draw(&self, printer: &cursive::Printer) {
        for (i, row) in self.as_rows().iter().enumerate() {
            for (j, c) in row.iter().enumerate() {
                printer.print((i, j), c)
            }
        }
    }

    fn layout(&mut self, size: Vec2) {
        if self.grid.size() == (size.x, size.y) {
            if !self.pause {
                self.grid = self.grid.tick()
            }
        } else {
            self.grid = Grid::random(size.x, size.y, 0.1)
        }
    }

    fn on_event(&mut self, evt: Event) -> EventResult {
        match evt {
            Event::Refresh => {
                self.grid = self.grid.tick();
            }
            Event::Char(' ') => self.pause = !self.pause,
            Event::Char('r') => {
                return EventResult::Consumed(Some(Callback::from_fn(|siv| {
                    siv.add_layer(reset_dialog())
                })))
            }
            Event::Mouse {
                position: p,
                event: evt,
                ..
            } => {
                if let MouseEvent::Press(_) = evt {
                    self.grid
                        .grid
                        .get_mut(p.x)
                        .and_then(|r| r.get_mut(p.y))
                        .map(|v| *v = v.toggle());
                }
            }
            _ => return EventResult::Ignored,
        };

        EventResult::Consumed(None)
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        constraint
    }
}
