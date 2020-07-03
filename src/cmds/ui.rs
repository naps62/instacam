extern crate iui;

use iui::controls::{
    Button, Entry, Group, HorizontalBox, HorizontalSeparator, Label, Slider, Spacer, Spinbox,
    VerticalBox,
};
use iui::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

struct State {
    input: String,
}

pub fn run() {
    let ui = UI::init().expect("Couldn't initialize UI library");

    let state = Rc::new(RefCell::new(State { input: "".into() }));

    let (input_group, mut input) = {
        let mut input_group = Group::new(&ui, "Inputs");
        let mut input_vbox = VerticalBox::new(&ui);
        input_vbox.set_padded(&ui, true);

        let input = Entry::new(&ui);
        let button = Button::new(&ui, "Load");

        input_vbox.append(&ui, input.clone(), LayoutStrategy::Compact);

        input_group.set_child(&ui, input_vbox);

        (input_group, input)
    };

    let (output_group, text_label) = {
        let mut output_group = Group::new(&ui, "Outputs");
        let mut output_vbox = VerticalBox::new(&ui);

        let text_label = Label::new(&ui, "");

        output_vbox.append(&ui, text_label.clone(), LayoutStrategy::Compact);

        output_group.set_child(&ui, output_vbox);
        (output_group, text_label)
    };

    let mut hbox = HorizontalBox::new(&ui);
    hbox.append(&ui, input_group, LayoutStrategy::Stretchy);
    hbox.append(&ui, output_group, LayoutStrategy::Stretchy);

    let mut window = Window::new(&ui, "Input Output Test", 300, 150, WindowType::NoMenubar);

    window.set_child(&ui, hbox);
    window.show(&ui);

    input.on_changed(&ui, {
        let state = state.clone();
        move |val| {
            state.borrow_mut().input = val;
        }
    });

    let mut event_loop = ui.event_loop();
    event_loop.on_tick(&ui, {
        let ui = ui.clone();
        let mut text_label = text_label.clone();

        move || {
            let state = state.borrow();

            text_label.set_text(&ui, &format!("Text: {}", state.input));
        }
    });

    event_loop.run(&ui);
}
