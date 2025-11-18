// use gdk::Display;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button};
use dbus::blocking::Connection;
use std::time::Duration;
use std::process::Command;

const APP_ID: &str = "net.quackor.rust_exit";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to signals
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn logout() -> Result<(), Box<dyn std::error::Error>> {
    let _output = Command::new("openbox").arg("--exit").output()?;
    Ok(())
}

fn dbus(cmd: &str) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::new_system()?;
    let proxy = conn.with_proxy("org.freedesktop.login1", "/org/freedesktop/login1", Duration::from_millis(5000));
    let (_s,) : (String,)  = proxy.method_call("org.freedesktop.login1.Manager", cmd, (true, ))?;
    Ok(())
}

fn build_ui(app: &Application) {
    // Create buttons
    let button_1 = Button::with_label("Reboot");
    let button_2 = Button::with_label("Shutdown");
    let button_3 = Button::with_label("Logout");

    button_1.connect_clicked(|_| dbus("Reboot").expect("dbus connection should work"));
    button_2.connect_clicked(|_| dbus("PowerOff").expect("dbus connection should work"));
    button_3.connect_clicked(|_| logout().expect("openbox command should work"));

    // Create `gtk_box` and add buttons
    let gtk_box = gtk::Box::builder()
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .spacing(12)
        .build();
    gtk_box.append(&button_1);
    gtk_box.append(&button_2);
    gtk_box.append(&button_3);

    // Create a new window and present it
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Logout")
        .child(&gtk_box)
        .build();
    window.present();
}
