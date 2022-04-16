//use gtk::TreeView;
use gtk::glib;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, MessageDialog, TreeView};

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("lv-gui.ui");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.object("window1").expect("Couldn't get window1");
    window.set_application(Some(application));
    let dialog: MessageDialog = builder
        .object("messagedialog1")
        .expect("Couldn't get messagedialog1");
    
    let treeview: TreeView = builder
        .object("treeview1")
        .expect("Couldn't get treeview1");

    dialog.connect_delete_event(|dialog, _| {
        dialog.hide();
        gtk::Inhibit(true)
    });

    treeview.connect_delete_event(|treeview, _| {
        treeview.show();
        gtk::Inhibit(true)
    });

    builder.connect_signals(move |_, handler_name| {
        // This is the one-time callback to register signals.
        // Here we map each handler name to its handler.

        if handler_name == "button1_clicked" {
            // Return the signal handler.
            Box::new(
                glib::clone!(@weak dialog => @default-return None, move |_| {
                    dialog.show_all();
                    None
                }),
            )
        } else {
            panic!("Unknown handler name {}", handler_name)
        }
    });

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.amstelchen.lv-gui"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}
