use chrono::prelude::*;
use gio::prelude::*;
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

    //let backup: gtk::Button = builder.get_object("backup").unwrap();
    //let restart: gtk::Button = builder.get_object("restart").unwrap();
    //let shutdown: gtk::Button = builder.get_object("shutdown").unwrap();
    //backup.connect_clicked(move |_| println!("Backup!"));
    //restart.connect_clicked(move |_| println!("Restart!"));
    //shutdown.connect_clicked(move |_| println!("Shutdown!"));
    // backup.connect_clicked(glib::clone!(@weak window => move |_| {
    //     let folder_chooser = FileChooserDialog::new(Some("Choose folder"), Some(&window), gtk::FileChooserAction::SelectFolder);
    //     folder_chooser.show_all();
    // }));
    // executes the closure once every second

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
