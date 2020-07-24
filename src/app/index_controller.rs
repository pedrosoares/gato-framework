use serde_json::json;
use gato_core::kernel::{Request, Response, Logger};

pub struct IndexController {}
impl IndexController {
    pub fn index(request: &Request) -> Response {
        Logger::info("IndexController[index]");
        let params = request.all();
        return Response::json(json!(params));
    }

    pub fn test(request: &Request) -> Response {
        Logger::info("IndexController[test]");
        let id = request.get("id", "");
        return Response::new(id.as_str());
    }
}