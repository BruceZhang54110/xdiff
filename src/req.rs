use std::{any, str::FromStr};

use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use url::Url;
use reqwest::{header::{HeaderMap, HeaderName, HeaderValue}, Client, Method, Response};
use serde_json::{json, Value};
use http::header::CONTENT_TYPE;
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


impl RequestProfile {

    pub async fn send(&self, args: &ExtraAgrs) -> Result<Response> {
        let request_builder = Client::new().request(self.method.clone(), self.url.clone());



        let (headers, query, body) = self.generate(args)?;

        todo!()
    }


    /// 生成 header, query,body
    pub fn generate(&self, args: &ExtraAgrs) -> Result<(HeaderMap, Value, String)> {
        let mut headers = self.headers.clone();

        let mut query = self.params.clone().unwrap_or_else(|| json!({}));

        let mut body = self.body.clone().unwrap_or_else(|| json!({}));


        for (k, v) in &args.headers {
            headers.insert(HeaderName::from_str(k)?, HeaderValue::from_str(v)?);
        }

        // 请求头中没有设置 Content-Type 时，默认设置为 application/json
        if !headers.contains_key(CONTENT_TYPE) {
            headers.insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
        }

        for (k, v) in &args.query {
            query[k] = v.parse()?;
        }

        for (k, v) in &args.body {
            body[k] = v.parse()?
        }
        // let content_type_value = headers.get(CONTENT_TYPE);
        match headers.get(CONTENT_TYPE) {
            Some(content_type_value) => {
                let content_type_value = content_type_value.to_str().map_err(|_| anyhow::anyhow!("Invalid Content-Type"))?;
                match content_type_value {
                    "application/json" => {
                        // 处理 JSON 格式的请求体
                        let json_body = serde_json::to_string(&body)?;
                        Ok((headers, query, json_body))
                    }
                    "application/x-www-form-urlencoded" => {
                        // 处理 x-www-form-urlencoded 格式的请求体
                        let form_body = serde_urlencoded::to_string(&body)?;
                        Ok((headers, query, form_body))
                    }
                    "text/plain" => {
                        // 处理纯文本格式的请求体
                        let text_body = body.to_string();
                        Ok((headers, query, text_body))
                    }
                    _ => Err(anyhow::anyhow!("Unsupported Content-Type")),
                    
                }
            }
            _ => Err(anyhow::anyhow!("Unsupported Content-Type")),
        }

        //Ok((headers, query, body))

    }

}