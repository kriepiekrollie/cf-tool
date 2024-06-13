use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Template {
    pub alias: String,
    pub lang: usize,
    pub path: String,
    pub suffix: Vec<String>,
    
    pub before_script: Option<String>,
    pub script: Option<String>,
    pub after_script: Option<String>,
}
