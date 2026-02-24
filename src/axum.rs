use crate::Serialize;
use crate::{OPENAPI_JSON, SCALAR_API_REFERENCE_JS, SCALAR_SCRIPT, Scalar};
use axum::response::IntoResponse;
use axum::{Router, routing};
use http::{HeaderValue, header};

impl<S: Serialize, R> From<Scalar<S>> for Router<R>
where
    R: Clone + Send + Sync + 'static,
{
    fn from(scalar: Scalar<S>) -> Router<R> {
        let markup = scalar.to_markup();
        let api_json = serde_json::to_string(&scalar.openapi).unwrap();
        let scalar_url = scalar.url.as_ref();
        Router::<R>::new()
            .route(
                scalar_url,
                routing::get(move || async {
                    let headers = [(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static("text/html; charset=utf-8"),
                    )];
                    (headers, markup).into_response()
                }),
            )
            .route(
                format!("{scalar_url}/{SCALAR_SCRIPT}").as_str(),
                routing::get(move || async {
                    let headers = [(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static("application/javascript"),
                    )];
                    (headers, SCALAR_API_REFERENCE_JS).into_response()
                }),
            )
            .route(
                format!("{scalar_url}/{OPENAPI_JSON}").as_str(),
                routing::get(move || async { api_json.into_response() }),
            )
    }
}
