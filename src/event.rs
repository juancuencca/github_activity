use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug)]
pub struct Event {
    r#type: String, 
    repo: Repo,
    payload: Option<Payload>,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "- {} triggered", self.r#type)?;
        if let Some(payload) = &self.payload {
            if let Some(commits) = &payload.commits {
                if commits.len() == 1 {
                    write!(f, " {} commit", commits.len())?; 
                } else {
                    write!(f, " {} commits", commits.len())?;
                }
            }
        }
        write!(f, " on {}", self.repo.name)
    }
}

#[derive(Deserialize, Debug)]
struct Repo {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Payload {
    commits: Option<Vec<Commit>>,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
struct Commit {
    message: String,
}