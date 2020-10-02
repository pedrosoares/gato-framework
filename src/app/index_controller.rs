use gato_core::kernel::{Request, Response, Logger};

pub struct IndexController {}
impl IndexController {
    pub fn index(request: &Request) -> Response {
        Logger::info("IndexController[index]");
        let params = request.json();
        return Response::json(params);
    }

    pub fn test(request: &Request) -> Response {
        Logger::info("IndexController[test]");
        let id = request.get_param("id", "");
        return Response::new(id.as_str());
    }
}
