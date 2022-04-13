mod bootstrap;
pub mod domains;

use gato_core::kernel::Application;

fn main() {
    // Create a new Application and pass the execution arguments
    let mut application = Application::new(std::env::args(), &bootstrap::boot);
    // run the application
    application.run();
}
