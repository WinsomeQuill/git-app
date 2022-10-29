use gtk::gio::SimpleAction;
use gtk::{prelude::*, Button, Align, Label, Orientation};
use gtk::{Application, ApplicationWindow};

const APP_ID: &str = "org.gtk_rs.git_app";

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("GIT APP")
        .build();

    let button = Button::builder()
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .halign(Align::Center)
        .valign(Align::Center)
        .name("count")
        .label("Click me!")
        .build();

    let counter = 0;
    let label = Label::builder()
        .label(&format!("Count: {}", counter))
        .name("ltext")
        .build();

    let gtk_box = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .margin_top(10)
        .margin_bottom(10)
        .margin_start(10)
        .margin_end(10)
        .halign(Align::Center)
        .build();

    gtk_box.append(&label);
    gtk_box.append(&button);

    button.connect_clicked(move |button| {
        let parameter = 1;
        button
            .activate_action("win.count", Some(&parameter.to_variant()))
            .expect("The action does not exist.");
    });

    let action_count = SimpleAction::new_stateful(
        "count",
        Some(&i32::static_variant_type()),
        &counter.to_variant(),
    );

    action_count.connect_activate(move |action, parameter| {
        let mut state = action
            .state()
            .expect("Could not get state.")
            .get::<i32>()
            .expect("The variant needs to be of type `i32`.");

        let parameter = parameter
            .expect("Could not get parameter.")
            .get::<i32>()
            .expect("The variant needs to be of type `i32`.");

        state += parameter;
        action.set_state(&state.to_variant());

        label.set_label(&format!("Counter: {state}"));
    });

    window.add_action(&action_count);
    window.set_child(Some(&gtk_box));

    window.present();
}