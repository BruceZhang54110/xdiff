use std::collections::HashMap;
use anyhow::{Ok, Result};
use reqwest::{header::HeaderMap, Method};
use tokio::fs;
use url::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ExtraAgrs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RequestProfile {

    #[serde(with = "http_serde::method", default)]
    pub method: Method,

    pub url: Url,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub params: Option<Value>,

    #[serde(skip_serializing_if = "HeaderMap::is_empty"
    , with = "http_serde::header_map"
    , default)]
    pub headers: HeaderMap,

    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub body: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseProfile {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_headers: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub skip_body: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiffProfile {
    pub req1: RequestProfile,
    pub req2: RequestProfile,
    pub res: ResponseProfile,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct DiffConfig {
    #[serde(flatten)]
    pub profiles: HashMap<String, DiffProfile>
}

impl DiffConfig {

    pub async fn load_yml(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path).await?;
        Self::from_yml(&content)
    }

    pub fn from_yml(content: &str) -> Result<Self> {
        Ok(serde_yaml::from_str(content)?)
    }

    pub fn get_profile(&self, name: &str) -> Option<&DiffProfile> {
        self.profiles.get(name)
    }
}

impl DiffProfile {
    pub fn diff(&self, args: ExtraAgrs) -> Result<String> {
        println!("profile: {:?}", self);
        println!("args: {:?}", args);
        Ok("".to_string())
    }
}