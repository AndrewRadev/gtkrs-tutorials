pub mod preview;
pub mod state;
pub mod ui;

use ui::App;

fn main() {
    // Initialize the UI's initial state
    App::new()
        // Connect events to the UI
        .connect_events()
        // Display the UI and execute the program
        .then_execute();
}
