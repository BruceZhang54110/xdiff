pub mod config;
pub use config::{DiffConfig, DiffProfile, ResponseProfile};
pub mod cli;
pub mod req;
pub mod utils;

pub use req::RequestProfile;



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtraAgrs {

    pub headers: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
    pub body: Vec<(String, String)>,

}