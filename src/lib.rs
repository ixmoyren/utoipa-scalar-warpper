#[cfg(feature = "actix-web")]
mod actix;

#[cfg(feature = "axum")]
mod axum;

use maud::{DOCTYPE, Markup, html};
use serde::Serialize as SerdeSerialize;
use serde_json::Value;
use std::borrow::Cow;
use utoipa::openapi::OpenApi;

const SCALAR_API_REFERENCE_JS: &str = include_str!("../static/scalar-api-reference.js");

const SCALAR_SCRIPT: &str = "scalar-api-reference.js";
const OPENAPI_JSON: &str = "api-docs/openapi.json";

pub trait Serialize: SerdeSerialize {}

impl Serialize for OpenApi {}

impl Serialize for Value {}

#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Scalar<S: Serialize> {
    url: Cow<'static, str>,
    title: Cow<'static, str>,
    openapi: S,
    config: Config,
}

impl<S: Serialize> Scalar<S> {
    pub fn new(openapi: S) -> Self {
        Self {
            url: Cow::Borrowed("/"),
            title: Cow::Borrowed("Scalar"),
            openapi,
            config: Config::default(),
        }
    }

    pub fn with_url<U>(mut self, url: U) -> Self
    where
        U: Into<Cow<'static, str>>,
    {
        self.url = url.into();
        self
    }

    pub fn with_title(mut self, title: &'static str) -> Self {
        self.title = Cow::Borrowed(title);
        self
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    fn config_json(&self) -> String {
        serde_json::to_string(&self.config).unwrap()
    }

    fn script_url(&self) -> String {
        let url = self.url.as_ref();
        format!("{url}/{SCALAR_SCRIPT}")
    }

    fn api_json(&self) -> String {
        serde_json::to_string(&self.openapi).unwrap()
    }

    fn api_json_url(&self) -> String {
        let url = self.url.as_ref();
        format!("{url}/{OPENAPI_JSON}")
    }

    fn markup(&self) -> Markup {
        let config = self.config_json();
        let title = self.title.as_ref();
        let url = self.url.as_ref();
        let data_url = format!("{url}/{OPENAPI_JSON}");
        let script_src = format!("{url}/{SCALAR_SCRIPT}");
        html! {
            (DOCTYPE)
            head {
                title { (title) }
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1.0";
            }
            body {
                script id="api-reference" type="application/json" data-url=(data_url)
                data-configuration=(config) {
                }
                script src=(script_src) type="module" {
                }
            }
        }
    }
}

#[derive(SerdeSerialize, Debug, Clone, Default)]
pub struct MetaInfo {
    title: String,
    description: String,
    #[serde(rename(serialize = "ogDescription"))]
    og_description: String,
    #[serde(rename(serialize = "ogTitle"))]
    og_title: String,
    #[serde(rename(serialize = "ogImage"))]
    og_image: String,
    #[serde(rename(serialize = "twitterCard"))]
    twitter_card: String,
}

impl MetaInfo {
    pub fn title(mut self, title: impl AsRef<str>) -> Self {
        self.title = title.as_ref().to_owned();
        self
    }

    pub fn description(mut self, description: impl AsRef<str>) -> Self {
        self.description = description.as_ref().to_owned();
        self
    }

    pub fn og_description(mut self, og_description: impl AsRef<str>) -> Self {
        self.og_description = og_description.as_ref().to_owned();
        self
    }

    pub fn og_title(mut self, og_title: impl AsRef<str>) -> Self {
        self.og_title = og_title.as_ref().to_owned();
        self
    }

    pub fn og_image(mut self, og_image: impl AsRef<str>) -> Self {
        self.og_image = og_image.as_ref().to_owned();
        self
    }

    pub fn twitter_card(mut self, twitter_card: impl AsRef<str>) -> Self {
        self.twitter_card = twitter_card.as_ref().to_owned();
        self
    }
}

#[derive(SerdeSerialize, Debug, Clone)]
pub struct Config {
    theme: String,
    #[serde(rename(serialize = "isEditable"))]
    is_editable: bool,
    #[serde(rename(serialize = "hideModels"))]
    hide_models: bool,
    #[serde(rename(serialize = "hideClientButton"))]
    hide_client_button: bool,
    #[serde(rename(serialize = "hideClients"))]
    hidden_clients: bool,
    #[serde(rename(serialize = "defaultOpenAllTags"))]
    default_open_all_tags: bool,
    #[serde(rename(serialize = "showSidebar"))]
    show_sidebar: bool,
    #[serde(
        rename(serialize = "metaData"),
        skip_serializing_if = "Option::is_none"
    )]
    meta_data: Option<MetaInfo>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "saturn".to_owned(),
            is_editable: false,
            hide_models: false,
            hide_client_button: true,
            hidden_clients: true,
            default_open_all_tags: false,
            show_sidebar: true,
            meta_data: None,
        }
    }
}

impl Config {
    pub fn theme(mut self, theme: impl AsRef<str>) -> Self {
        self.theme = theme.as_ref().to_owned();
        self
    }

    pub fn editable(mut self, is_editable: bool) -> Self {
        self.is_editable = is_editable;
        self
    }

    pub fn hide_models(mut self, hide_models: bool) -> Self {
        self.hide_models = hide_models;
        self
    }

    pub fn hide_client_button(mut self, hide_client_button: bool) -> Self {
        self.hide_client_button = hide_client_button;
        self
    }

    pub fn hidden_clients(mut self, hidden_clients: bool) -> Self {
        self.hidden_clients = hidden_clients;
        self
    }

    pub fn default_open_all_tags(mut self, default_open_all_tags: bool) -> Self {
        self.default_open_all_tags = default_open_all_tags;
        self
    }

    pub fn show_sidebar(mut self, show_sidebar: bool) -> Self {
        self.show_sidebar = show_sidebar;
        self
    }

    pub fn meta_data(mut self, meta_data: MetaInfo) -> Self {
        self.meta_data = Some(meta_data);
        self
    }
}
