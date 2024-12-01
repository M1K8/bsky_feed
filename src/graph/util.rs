use std::sync::Arc;

use neo4rs::Graph;
use tokio::sync::Mutex;

use crate::graph::queries::PURGE_OLD_POSTS;

const PURGE_TIME: u64 = 15 * 60;

pub fn get_post_uri(did: String, rkey: String) -> String {
    format!("at://{did}/app.bsky.feed.post/{rkey}")
}
pub fn pluralize(word: &str) -> String {
    let word_len = word.len();
    let snip = &word[..word_len - 1];
    let last_char = word.chars().nth(word_len - 1).unwrap();

    if last_char == 'y' || word.ends_with("ay") {
        return format!("{}ies", snip);
    } else if last_char == 's' || last_char == 'x' || last_char == 'z' {
        return format!("{}es", word);
    } else if last_char == 'o' && word.ends_with("o") && !word.ends_with("oo") {
        return format!("{}oes", snip);
    } else if last_char == 'u' && word.ends_with("u") {
        return format!("{}i", snip);
    } else {
        return format!("{}s", word);
    }
}

pub async fn kickoff_purge(spin: Arc<Mutex<()>>, conn: Graph) -> Result<(), neo4rs::Error> {
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(PURGE_TIME)).await;
        println!("Purging old posts");
        let lock = spin.lock().await;
        let qry = neo4rs::query(PURGE_OLD_POSTS);
        conn.run(qry).await?;
        drop(lock);
        println!("Done!");
    }
}
