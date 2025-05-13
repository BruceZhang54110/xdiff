use std::str::FromStr;
use tokio::time::Duration;
use anyhow::{anyhow, Ok, Result};
use serde::{Deserialize, Serialize};
use url::Url;
use reqwest::{header::{HeaderMap, HeaderName, HeaderValue}, Client, Method, Response};
use serde_json::{json, Value};
use http::header::CONTENT_TYPE;
use crate::{ExtraAgrs, ResponseProfile};


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


#[derive(Debug)]
pub struct ResponseExt(Response);


impl RequestProfile {

    pub async fn send(&self, args: &ExtraAgrs) -> Result<ResponseExt> {

        let (headers, query, body) = self.generate(args).expect("generate error");

        let client = Client::new();

        let req = client.request(self.method.clone(), self.url.clone())
        .query(&query)
        .headers(headers)
        .body(body)
        .timeout(Duration::from_secs(10))
        .build()?;
        println!("start execute");
        let res = client.execute(req).await?;
        Ok(ResponseExt(res))
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
        let content_type_value = get_content_type(&headers);
        match content_type_value.as_deref() {
            Some("application/json") => {
                // 处理 JSON 格式的请求体
                let json_body = serde_json::to_string(&body)?;
                Ok((headers, query, json_body))
            }
            Some("application/x-www-form-urlencoded") | Some("multipart/form-data") => {
                // 处理 x-www-form-urlencoded 格式的请求体
                let form_body = serde_urlencoded::to_string(&body)?;
                Ok((headers, query, form_body))
            }
            _ => Err(anyhow::anyhow!("Unsupported Content-Type")),
                    
        }
    }

}


impl ResponseExt {

    pub async fn filter_text(self, profile: &ResponseProfile) -> Result<String> {
        let res = self.0;
        let mut ouptput = String::new();
        println!("start filter text");
        ouptput.push_str(&format!("response result: {:?} {}\r\n", res.version(), res.status()));
        
        match res.status() {
            reqwest::StatusCode::OK => {
                ouptput.push_str("response is ok\r\n");
            }
            _ => return Err(anyhow!("url is :{}, response is not ok, status code is {}", res.url(), res.status())),
            
        }

        let headers = res.headers();
        for (header_key, header_value) in headers.iter() {
            if !profile.skip_headers.iter().any(|sh| sh == header_key.as_str()) {
                ouptput.push_str(&format!("{}: {:?}\r", header_key, header_value));
            }
        }
        println!("start filter body");

        ouptput.push_str("\n");
        let content_type =  get_content_type(&headers);

        let text = res.text().await?;
        match content_type.as_deref() {
            Some("application/json") => {
                let text = filter_json(&text, &profile.skip_body)?;
                ouptput.push_str(&text);
            }
            _ => {
                ouptput.push_str(&text);
            }
        }
        println!("start filter json");
        let mut json: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|err| {
            println!("Failed to parse JSON: {}, error is :{} ", &text, err);
            json!({})
        });
        for key in &profile.skip_body {
            json[key] = json!(null);
        }
        println!("finish filter json");
        Ok(ouptput)
    }


}

fn filter_json(text: &str, skip_body: &[String]) -> Result<String> {
    let mut json: serde_json::Value = serde_json::from_str(text)?;

    match json {
        Value::Object(ref mut obj) => {
            for key in skip_body {
                obj.remove(key);
                
            }
        }
        _ => 
            // TODO: 处理其他类型的 JSON
            {}
    }

    for key in skip_body {
        json[key] = json!(null);
    }
    Ok(serde_json::to_string_pretty(&json)?)
}

fn get_content_type(headers: &HeaderMap) -> Option<String> {
    headers.get(CONTENT_TYPE)
    .map(|v| v.to_str().unwrap().split(";").next())
    .flatten()
    .map(|v| v.to_string())
}