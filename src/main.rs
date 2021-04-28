use chrono::prelude::*;
use gio::prelude::*;
use gio::VolumeMonitor;
use gtk::prelude::*;
use gtk::Application;

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

    // Setup clock in headerbar
    setup_clock(
        &builder
            .get_object("clock")
            .expect("Unable to find 'clock' Label object"),
    );

    let body: gtk::Stack = builder
        .get_object("body")
        .expect("Unable to find 'body' Stack object");

    // Setup steps
    let welcome = gtk::Label::new(Some("Welcome to BackupBox!"));
    body.add_named(&welcome, "0");
    let step1 = gtk::Label::new(Some("Step 1: select origin drive"));
    body.add_named(&step1, "1");
    let step2 = gtk::Label::new(Some("Step 2: select target drive"));
    body.add_named(&step2, "2");
    body.set_visible_child_name("0");

    // Updating button callback does not work, you should recreate the buttons

    //retrieve_volumes();

    //window.fullscreen();
    window.show_all();
}

fn setup_clock(clock: &gtk::Label) {
    let c = clock.clone();
    let time = Local::now();
    c.set_text(&format!("{}:{}", time.hour(), time.minute()));
    glib::timeout_add_seconds_local(60, move || {
        let time = Local::now();
        c.set_text(&format!("{}:{}", time.hour(), time.minute()));
        glib::Continue(true)
    });
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
