use crate::Serialize;
use crate::{SCALAR_API_REFERENCE_JS, Scalar};
use axum::response::IntoResponse;
use axum::{Router, routing};
use http::{HeaderValue, header};

impl<S: Serialize, R> From<Scalar<S>> for Router<R>
where
    R: Clone + Send + Sync + 'static,
{
    fn from(scalar: Scalar<S>) -> Router<R> {
        let markup = scalar.markup();
        let scalar_url = scalar.url.as_ref();
        let script_url = scalar.script_url();
        let api_json = scalar.api_json();
        let api_json_url = scalar.api_json_url();
        Router::<R>::new()
            .route(
                scalar_url,
                routing::get(move || async { markup.into_response() }),
            )
            .route(
                script_url.as_str(),
                routing::get(move || async {
                    let headers = [(
                        header::CONTENT_TYPE,
                        HeaderValue::from_static("application/javascript"),
                    )];
                    (headers, SCALAR_API_REFERENCE_JS).into_response()
                }),
            )
            .route(
                api_json_url.as_str(),
                routing::get(move || async { api_json.into_response() }),
            )
    }
}
