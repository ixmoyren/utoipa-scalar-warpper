use crate::{SCALAR_API_REFERENCE_JS, Scalar, Serialize};
use actix_web::dev::HttpServiceFactory;
use actix_web::guard::Get;
use actix_web::web::Data;
use actix_web::{HttpResponse, Resource, Responder};

impl<S: Serialize> HttpServiceFactory for Scalar<S> {
    fn register(self, config: &mut actix_web::dev::AppService) {
        let markup = self.markup();
        let scalar_url = self.url.as_ref();
        let script_url = self.script_url();
        let api_json = self.api_json();
        let api_json_url = self.api_json_url();

        async fn serve_scalar(scalar: Data<String>) -> impl Responder {
            HttpResponse::Ok()
                .content_type("text/html")
                .body(scalar.to_string())
        }

        async fn serve_scalar_api_js(api_js: Data<&str>) -> impl Responder {
            HttpResponse::Ok()
                .content_type("application/javascript")
                .body(api_js.to_string())
        }

        async fn serve_scalar_api_json(api_json: Data<String>) -> impl Responder {
            HttpResponse::Ok()
                .content_type("application/json")
                .body(api_json.to_string())
        }

        Resource::new(scalar_url)
            .guard(Get())
            .app_data(Data::new(markup.0))
            .to(serve_scalar)
            .register(config);

        Resource::new(script_url)
            .guard(Get())
            .app_data(Data::new(SCALAR_API_REFERENCE_JS))
            .to(serve_scalar_api_js)
            .register(config);

        Resource::new(api_json_url)
            .guard(Get())
            .app_data(Data::new(api_json))
            .to(serve_scalar_api_json)
            .register(config);
    }
}
