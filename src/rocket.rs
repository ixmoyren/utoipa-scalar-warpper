use rocket::http::Method;
use rocket::response::content::{RawHtml, RawJavaScript, RawJson};
use rocket::route::{Handler, Outcome};
use rocket::{Data, Request, Route};

use crate::{SCALAR_API_REFERENCE_JS, Scalar, Serialize};

impl<S: Serialize> From<Scalar<S>> for Vec<Route> {
    fn from(scalar: Scalar<S>) -> Self {
        let markup = scalar.markup();
        let scalar_url = scalar.url.as_ref();
        let script_url = scalar.script_url();
        let api_json = scalar.api_json();
        let api_json_url = scalar.api_json_url();
        vec![
            Route::new(Method::Get, scalar_url, ScalarHandler(markup.0)),
            Route::new(
                Method::Get,
                script_url.as_ref(),
                ScalarScriptHandler(SCALAR_API_REFERENCE_JS.to_owned()),
            ),
            Route::new(
                Method::Get,
                api_json_url.as_ref(),
                ScalarApiJsonHandler(api_json),
            ),
        ]
    }
}

#[derive(Clone)]
struct ScalarHandler(String);

#[rocket::async_trait]
impl Handler for ScalarHandler {
    async fn handle<'r>(&self, request: &'r Request<'_>, _: Data<'r>) -> Outcome<'r> {
        Outcome::from(request, RawHtml(self.0.clone()))
    }
}

#[derive(Clone)]
struct ScalarScriptHandler(String);

#[rocket::async_trait]
impl Handler for ScalarScriptHandler {
    async fn handle<'r>(&self, request: &'r Request<'_>, _: Data<'r>) -> Outcome<'r> {
        Outcome::from(request, RawJavaScript(self.0.clone()))
    }
}

#[derive(Clone)]
struct ScalarApiJsonHandler(String);

#[rocket::async_trait]
impl Handler for ScalarApiJsonHandler {
    async fn handle<'r>(&self, request: &'r Request<'_>, _: Data<'r>) -> Outcome<'r> {
        Outcome::from(request, RawJson(self.0.clone()))
    }
}
