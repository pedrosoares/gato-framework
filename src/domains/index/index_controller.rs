use gato_core::kernel::{get_body, get_param, get_query};
use gato_core::kernel::{Request, Response, Logger};
use serde_json::json;


pub struct IndexController {}
impl IndexController {
    pub fn index(request: &Request) -> Response {
        Logger::info("IndexController[index]");
        let params = request.json();
        let name = get_body!(request, "name");

        return Response::new().status(200).json(json!({
            "params": params,
            "name": name
        }));
    }

    pub fn test(request: &Request) -> Response {
        Logger::info("IndexController[test]");
        let id = get_param!(request, "id");
        let (utm_a, utm_b) = get_query!(request, "utm_a", "utm_b");

        return Response::new().status(200).json(json!({
            "id": id,
            "utms": {
                "utm_a": utm_a,
                "utm_b": utm_b
            }
        }));
    }
}
