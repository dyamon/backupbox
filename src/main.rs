//use gio::prelude::*;
use chrono::prelude::*;
use gtk::prelude::*;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    // First we get the file content.
    let glade_src = include_str!("../main_view.glade");
    // Then we call the Builder call.
    let builder = gtk::Builder::from_string(glade_src);

    let main: gtk::Window = builder.get_object("main").unwrap();

    let backup: gtk::Button = builder.get_object("backup").unwrap();
    let restart: gtk::Button = builder.get_object("restart").unwrap();
    let shutdown: gtk::Button = builder.get_object("shutdown").unwrap();
    let clock: gtk::Label = builder.get_object("clock").unwrap();

    backup.connect_clicked(move |_| println!("Backup!"));
    restart.connect_clicked(move |_| println!("Restart!"));
    shutdown.connect_clicked(move |_| println!("Shutdown!"));

    // executes the closure once every second
    glib::timeout_add_seconds_local(60, move || {
        let time = Local::now();
        clock.set_text(&format!("{}:{}", time.hour(), time.minute()));
        // we could return glib::Continue(false) to stop our clock after this tick
        glib::Continue(true)
    });

    //main.fullscreen();
    main.show_all();

    // We start the gtk main loop.
    gtk::main();
}
