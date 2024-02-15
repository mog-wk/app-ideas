use gtk::prelude::*;
use gtk::{Button, Inhibit, Label, Window, WindowType};
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;
use rand::Rng;

fn main() {
    Win::run(()).unwrap();
}

struct Model {
    upper_bound: i32,
    lower_bound: i32,
    include_float: bool,
}

#[derive(Msg)]
enum Msg {
    UpperIncrement,
    UpperDecrement,
    LowerIncrement,
    LowerDecrement,
    SwitchDecimal,
    CalcRandom,
    Quit,
}

#[derive(Clone)]
struct Widgets {
    lower_minus_button: Button,
    lower_plus_button: Button,
    lower_bound_label: Label,

    counter_label: Label,
    calc_random_button: Button,
    include_float_numbers_button: Button,

    upper_minus_button: Button,
    upper_plus_button: Button,
    upper_bound_label: Label,


    window: Window,
}

struct Win {
    model: Model,
    widgets: Widgets,
}

impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            upper_bound: 20,
            lower_bound: 0,
            include_float: false,
        }
    }

    fn update(&mut self, event: Msg) {
        let lower_label = &self.widgets.lower_bound_label;
        let upper_label = &self.widgets.upper_bound_label;
        match event {
            Msg::UpperIncrement => {
                self.model.upper_bound += 1;
                upper_label.set_text(&self.model.upper_bound.to_string());
            }
            Msg::UpperDecrement => {
                self.model.upper_bound -= 1;
                upper_label.set_text(&self.model.upper_bound.to_string());
            }
            Msg::LowerIncrement => {
                self.model.lower_bound += 1;
                lower_label.set_text(&self.model.lower_bound.to_string());
            }
            Msg::LowerDecrement => {
                self.model.lower_bound -= 1;
                lower_label.set_text(&self.model.lower_bound.to_string());
            }
            Msg::CalcRandom => {
                let rand_num = if self.model.include_float {
                    get_random_decimal_number(self.model.lower_bound, self.model.upper_bound)
                } else {
                    get_random_natural_number(self.model.lower_bound, self.model.upper_bound)
                };

                match rand_num {
                    Ok(n) => self.widgets.counter_label.set_text(&n.to_string()),
                    Err(e) => display_input_error(&self, e),
                };
            }
            Msg::SwitchDecimal => {
                self.model.include_float = !self.model.include_float;
            }
            Msg::Quit => gtk::main_quit(),
        }
    }
}

fn get_random_natural_number(l: i32, h: i32) -> Result<f64, Error> {
    if h < l { return Err(Error::InvalidRange) }
    Ok(rand::thread_rng().gen_range(l..=h) as f64)
}

fn get_random_decimal_number(l: i32, h: i32) -> Result<f64, Error> {
    if h < l { return Err(Error::InvalidRange) }
    Ok(rand::thread_rng().gen_range(l as f64..=h as f64))
}

fn display_input_error(win: &Win, err: Error) {
    println!("ERROR: {:?}", err);
    win.widgets.counter_label.set_text("invalid range of inputs");
}


#[derive(Debug)]
enum Error {
    InvalidRange
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {

        let hbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(30)
            .expand(true)
            .margin_top(100)
            .build();

        // left box

        let lower_bound_vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(30)
            .margin_start(200)
            .build();

        let lower_bound_text = Label::new(Some("Lower\nBound"));
        lower_bound_vbox.add(&lower_bound_text);

        let lower_bound_label = Label::new(Some("0"));
        lower_bound_vbox.add(&lower_bound_label);

        let lower_plus_button = Button::with_label("+");
        lower_bound_vbox.add(&lower_plus_button);

        let lower_minus_button = Button::with_label("-");
        lower_bound_vbox.add(&lower_minus_button);

        hbox.add(&lower_bound_vbox);

        // middle box

        let middle_vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(60)
            .build();

        let info_text = Label::new(Some("||||||\tFNG-9000\t||||||"));
        middle_vbox.add(&info_text);

        let counter_label = Label::new(Some("0"));
        middle_vbox.add(&counter_label);

        let calc_random_button = Button::builder()
            .label("?")
            .margin_top(20)
            .build();
        middle_vbox.add(&calc_random_button);

        let include_float_numbers_button = Button::with_label("float?");
        middle_vbox.add(&include_float_numbers_button);

        hbox.add(&middle_vbox);

        // right box

        let upper_bound_vbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(30)
            .margin_end(200)
            .build();

        let upper_bound_text = Label::new(Some("Upper\nBound"));
        upper_bound_vbox.add(&upper_bound_text);

        let upper_bound_label = Label::new(Some("20"));
        upper_bound_vbox.add(&upper_bound_label);

        let upper_plus_button = Button::with_label("+");
        upper_bound_vbox.add(&upper_plus_button);

        let upper_minus_button = Button::with_label("-");
        upper_bound_vbox.add(&upper_minus_button);

        hbox.add(&upper_bound_vbox);

        let window = Window::new(WindowType::Toplevel);
        window.set_position(gtk::WindowPosition::Center);
        window.set_default_size(600, 600);

        window.add(&hbox);

        connect!(relm, lower_plus_button, connect_clicked(_), Msg::LowerIncrement);
        connect!(relm, lower_minus_button, connect_clicked(_), Msg::LowerDecrement);

        connect!(relm, calc_random_button, connect_clicked(_), Msg::CalcRandom);
        connect!(relm, include_float_numbers_button, connect_clicked(_), Msg::SwitchDecimal);

        connect!(relm, upper_plus_button, connect_clicked(_), Msg::UpperIncrement);
        connect!(relm, upper_minus_button, connect_clicked(_), Msg::UpperDecrement);


        // connect close button to Quit event
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        window.show_all();

        Win {
            model,
            widgets: Widgets {
                lower_minus_button,
                lower_plus_button,
                lower_bound_label,

                counter_label,
                calc_random_button,

                upper_minus_button,
                upper_plus_button,
                upper_bound_label,

                include_float_numbers_button,

                window: window,
            },
        }
    }
}
