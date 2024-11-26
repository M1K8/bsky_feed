use tokio::sync::mpsc;

use crate::server::types;

#[derive(Debug)]
pub struct FetchMessage {
    pub did: String,
    pub cursor: Option<String>,
    pub resp: mpsc::Sender<PostResp>,
}

pub struct PostMsg {
    pub uri: String,
    pub reason: String,
}

pub struct PostResp {
    pub posts: Vec<PostMsg>,
    pub cursor: Option<String>,
}

impl From<&PostMsg> for types::Post {
    fn from(value: &PostMsg) -> Self {
        types::Post {
            post: value.uri.clone(),
        }
    }
}