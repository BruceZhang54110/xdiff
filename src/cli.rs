use clap::Parser;
use clap::Subcommand;
use anyhow::Result;
use anyhow::anyhow;

use crate::ExtraAgrs;

/// Diff HTTP requests and responses

#[derive(Parser, Debug, Clone)]
#[clap(version, author, about, long_about = None)]
pub struct Args {

    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, Clone)]
#[non_exhaustive]
pub enum Action {
    /// Diff two API responses based on given profile
    Run(RunArgs),
}

#[derive(Parser, Debug, Clone)]
pub struct RunArgs {
    /// profile name
    #[clap(short, long, value_parser)]
    pub profile: String,

    /// Overrides args, Could be used to override the quary, headers and body of the request.
    /// For query params, use `-e key=value`
    /// For headers, use `-e %key=value`
    /// For body, use `-e @key=value`
    #[clap(short, long, value_parser = parse_key_val, number_of_values = 1)]
    pub extra_params: Vec<KeyVal>,

    #[clap(short, long, value_parser)]
    pub config: Option<String>,
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum KeyValType {
    Query,
    Header,
    Body,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyVal {
    key_type: KeyValType,
    key: String,
    value: String,
}

fn parse_key_val(s: &str) -> Result<KeyVal> {
    let mut parts = s.splitn(2, '=');
    let key = parts.next()
        .ok_or_else(|| anyhow!("Missing key"))?.trim();

    let value = parts.next()
        .ok_or_else(|| anyhow!("Missing key"))?
        .trim();

    let (key_type, key) = match key.chars().next() {
        Some('%') => (KeyValType::Header, &key[1..]),
        Some('@') => (KeyValType::Body, &key[1..]),
        Some(v) if v.is_ascii_alphanumeric() => (KeyValType::Query, key),
        _ => return Err(anyhow!("Invalid key format")),
    };

    Ok(KeyVal { 
        key_type: key_type, 
        key: key.to_string(),
        value: value.to_string(), 
    })
}

// 命令行获取到的参数集合转换为 `ExtraAgrs`
impl From<Vec<KeyVal>> for ExtraAgrs {
    fn from(args: Vec<KeyVal>) -> Self {
        let mut headers: Vec<(String, String)> = vec![];
        let mut query: Vec<(String, String)> = vec![];
        let mut body: Vec<(String, String)> = vec![];

        for arg in args {
            match arg.key_type {
                KeyValType::Header => headers.push((arg.key, arg.value)),
                KeyValType::Body => body.push((arg.key, arg.value)),
                KeyValType::Query => query.push((arg.key, arg.value)),
            }
        }
        Self {
            headers: headers,
            query: query,
            body: body,
        }
    }
}

