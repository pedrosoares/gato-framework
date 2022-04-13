use gato_core::kernel::ServiceProvider;
use gato_apache_cgi::ApacheServiceProvider;
use gato_http_server::HttpServerServiceProvider;
use gato_simple_router::{SimpleRouterServiceProvider, SimpleRouter};
use gato_stdout_logger::StdoutLoggerServiceProvider;
use crate::domains::index::IndexController;

pub fn boot(service_provider: &mut ServiceProvider) -> () {
    /*
     * The code bellow is used to handle multiples environments, in this case the
     * application can run in ApacheCGI server (https://github.com/pedrosoares/gato-apache-cgi) OR
     * standalone HTTP server (https://github.com/pedrosoares/gato-http-server)
     */
    match option_env!("HTTP_DRIVER").unwrap_or("http_server") {
        "apache" => {
            service_provider.register_provider(ApacheServiceProvider::new())
        },
        "http_server" => {
            service_provider.register_provider(HttpServerServiceProvider::new());
            /*
             * By default the GatoFramework save the application logs to a file on the storage/logs
             * folder, the StdoutLogger (https://github.com/pedrosoares/gato-stdout-logger) crate
             * send it to the stdout.
             */
            service_provider.register_provider(StdoutLoggerServiceProvider::new());
        },
        _ => panic!("invalid HTTP_DRIVER driver!"),
    }
    /*
     * SimpleRouter (https://github.com/pedrosoares/gato-simple-router) is a basic route system that will handle request and response.
     */
    service_provider.register_provider(SimpleRouterServiceProvider::new());

    /*
     * Now just register the application endpoints accordingly to SimpleRouter specifications
     */
    SimpleRouter::get("/", &IndexController::index);
    SimpleRouter::patch("/", &IndexController::index);
    SimpleRouter::get("/{id}/home", &IndexController::index);
    SimpleRouter::get("/{id}/test", &IndexController::test);
}