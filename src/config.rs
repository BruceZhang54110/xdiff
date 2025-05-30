use std::collections::HashMap;
use anyhow::{Ok, Result};
use tokio::fs;
use serde::{Deserialize, Serialize};

use crate::{utils::diff_text, ExtraAgrs, RequestProfile};

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
    pub async fn diff(&self, args: ExtraAgrs) -> Result<String> {
        println!("start diff");
        let res1 = self.req1.send(&args).await?;
        let res2 = self.req2.send(&args).await?;

        let text1 = res1.filter_text(&self.res).await?;
        let text2 = res2.filter_text(&self.res).await?;

        Ok(diff_text(&text1, &text2)?)
    }
}