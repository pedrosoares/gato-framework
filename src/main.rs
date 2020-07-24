pub mod app;
pub mod config;

use config::Bootstrap;
use gato_core::kernel::Application;

fn main() {
    let mut application = Application::new(std::env::args(), &Bootstrap::boot);
    application.run();
}
