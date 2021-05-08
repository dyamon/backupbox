use battery;
use chrono::prelude::*;
use gio::prelude::*;
use gio::VolumeMonitor;
use gtk::prelude::*;
use gtk::Application;
use std::cmp;
use std::{cell::RefCell, rc::Rc};

struct GUIState {
    page: u8,          // current page
    tot: u8,           // total pages
    body: gtk::Stack,  // main content
    prev: gtk::Button, // "prev" button
    next: gtk::Button, // "next" button
}

impl GUIState {
    fn new(tot: u8, body: gtk::Stack) -> Self {
        let mut state = GUIState {
            page: 1,
            tot,
            body,
            prev: gtk::Button::new(),
            next: gtk::Button::new(),
        };
        state.update();
        state
    }

    fn next(&mut self) {
        self.page = cmp::min(self.page + 1, self.tot);
        self.update();
    }

    fn prev(&mut self) {
        self.page = cmp::max(self.page - 1, 1);
        self.update();
    }

    fn set_prev_button(&mut self, prev: gtk::Button) {
        self.prev = prev;
        self.update();
    }

    fn set_next_button(&mut self, next: gtk::Button) {
        self.next = next;
        self.update();
    }

    fn update(&mut self) {
        self.body.set_visible_child_name(&format!("{}", self.page));
        self.prev.set_sensitive(self.page != 1);
        if self.page == self.tot {
            self.next.set_label("Backup!");
        } else {
            self.next.set_label("Next");
        }
    }
}

fn main() {
    let application = Application::new(Some("com.federicoigne.backupbox"), Default::default())
        .expect("failed to initialize GTK application");

    application.connect_activate(setup_gui);

    application.run(&[]);
}

fn setup_gui(app: &Application) {
    // Build initial GUI from Glade file
    let builder = gtk::Builder::from_string(include_str!("../main_view.glade"));

    // Attach window to application
    let window: gtk::Window = builder
        .get_object("main")
        .expect("Unable to find 'main' Window object");
    window.set_application(Some(app));

    let body: gtk::Stack = builder
        .get_object("body")
        .expect("Unable to find 'body' Stack object");

    // Setup steps
    let welcome = gtk::Label::new(Some("Welcome to BackupBox!"));
    body.add_named(&welcome, "1");
    let step1 = gtk::Label::new(Some("Step 1: select origin drive"));
    body.add_named(&step1, "2");
    let step2 = gtk::Label::new(Some("Step 2: select target drive"));
    body.add_named(&step2, "3");
    body.set_visible_child_name("1");

    // Initialize state
    let state = Rc::new(RefCell::new(GUIState::new(3, body)));

    // Updating button callback does not work, you should recreate the buttons
    let prev: gtk::Button = builder
        .get_object("prev")
        .expect("Unable to find 'prev' Button object");
    prev.connect_clicked({
        let state = Rc::clone(&state);
        move |_| state.borrow_mut().prev()
    });

    let next: gtk::Button = builder
        .get_object("next")
        .expect("Unable to find 'next' Button object");
    next.connect_clicked({
        let state = Rc::clone(&state);
        move |_| state.borrow_mut().next()
    });

    {
        let mut mstate = state.borrow_mut();
        mstate.set_prev_button(prev);
        mstate.set_next_button(next);
    }

    // Setup clock in headerbar
    setup_clock(
        &builder
            .get_object("clock")
            .expect("Unable to find 'clock' Label object"),
    );

    // Setup battery in headerbar
    setup_battery(
        &builder
            .get_object("battery_label")
            .expect("Unable to find 'clock' Label object"),
    );

    //retrieve_volumes();

    //window.fullscreen();
    window.show_all();
}

fn setup_clock(clock: &gtk::Label) {
    let c = clock.clone();
    let time = Local::now();
    c.set_text(&format!("{:02}:{:02}", time.hour(), time.minute()));
    glib::timeout_add_seconds_local(60, move || {
        let time = Local::now();
        c.set_text(&format!("{:02}:{:02}", time.hour(), time.minute()));
        glib::Continue(true)
    });
}

fn setup_battery(bat_label: &gtk::Label) {
    let b = bat_label.clone();
    let manager = battery::Manager::new().expect("Unable to instantiate battery manager");
    let mut batteries = manager.batteries().expect("Unable to detect any battery");
    if let Some(battery) = batteries.next() {
        let mut battery = battery.expect("Unable to access battery");
        b.set_text(&format!(
            "{}%",
            (battery.state_of_charge().value * 100.0) as i32
        ));
        glib::timeout_add_seconds_local(60, move || {
            manager
                .refresh(&mut battery)
                .expect("Unable to refresh battery state");
            b.set_text(&format!(
                "{}%",
                (battery.state_of_charge().value * 100.0) as i32
            ));
            glib::Continue(true)
        });
    } else {
        b.set_text("NaN");
    }
}

// fn setup_page(stack: gtk::Stack, prev: gtk::Button, next: gtk::Button, cur: i32, tot: i32) {
//     println!("Setting stack {}", cur);
//     stack.set_visible_child_name(&format!("{}", cur));
//     prev.set_sensitive(cur != 0);
//     prev.connect_clicked(
//         glib::clone!(@weak stack, @weak prev, @weak next => move |_| setup_page(stack, prev, next, cur - 1, tot))
//     );
//     if cur == tot - 1 {
//         next.connect_clicked(move |_| println!("Backup!"));
//     } else {
//         next.connect_clicked(
//             glib::clone!(@weak stack, @weak prev, @weak next => move |_| setup_page(stack, prev, next, cur + 1, tot))
//         );
//     };
// }

//fn setup_navigators_at<'a>(
//    stack: &'a gtk::Stack,
//    prev: &'a gtk::Button,
//    next: &'a gtk::Button,
//    cur: u8,
//    tot: u8,
//) -> impl Fn(&gtk::Button) + 'a {
//    move |_| {
//        stack.set_visible_child_name(&format!("{}", cur));
//        prev.set_sensitive(cur != 0);
//        prev.connect_clicked(setup_navigators_at(stack, prev, next, cur - 1, tot));
//        if cur == tot - 1 {
//            next.connect_clicked(move |_| println!("Backup!"));
//        } else {
//            next.connect_clicked(setup_navigators_at(stack, prev, next, cur + 1, tot));
//        };
//    }
//}

fn retrieve_volumes() {
    let monitor = VolumeMonitor::get();
    let volumes = monitor.get_volumes();
    volumes.iter().for_each(|v| {
        println!(
            "{} ({})",
            v.get_mount().unwrap().get_root().unwrap().get_uri(),
            v.get_name().unwrap()
        )
    });
}
