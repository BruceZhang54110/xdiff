pub mod config;
pub use config::{DiffConfig, DiffProfile, RequestProfile, ResponseProfile};
pub mod cli;



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtraAgrs {

    pub headers: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
    pub body: Vec<(String, String)>,

}