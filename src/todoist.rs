use chrono::NaiveDate;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fmt::{Display, Formatter};
use std::io::Read;

pub struct TodoistClient {
    token: String,
}

impl TodoistClient {
    pub fn new() -> TodoistClient {
        TodoistClient {
            token: env::var("TODOIST_TOKEN").expect("TODOIST_TOKEN is not set"),
        }
    }

    pub fn post_task(&self, task: TodoistTask) -> anyhow::Result<TodoistTask> {
        let todoist_endpoint = "https://api.todoist.com/rest/v2/tasks";

        let mut res = Client::new()
            .post(todoist_endpoint)
            .header("Accept", "application/json")
            .header("User-Agent", "burocrate")
            .bearer_auth(self.token.clone())
            .json(&task)
            .send()?;

        let mut body = String::new();
        res.read_to_string(&mut body)?;

        let task: TodoistTask = serde_json::from_str(&body)?;
        Ok(task)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TodoistTask {
    pub id: Option<String>,
    pub project_id: String,
    pub parent_id: Option<String>,
    pub content: String,
    pub description: Option<String>,
    pub priority: Option<i32>,
    due_string: Option<String>,
    assignee: Option<String>,
    labels: Vec<String>,
}

impl Display for TodoistTask {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (&self.parent_id, &self.id) {
            (None, None) => write!(f, "{}", self.content),
            (None, Some(i)) => write!(f, "{} (id={i})", self.content),
            (Some(p), Some(i)) => write!(f, "{} (id={i}, parent={p})", self.content),
            (Some(p), None) => write!(f, "{} (parent={p})", self.content),
        }
    }
}

impl TodoistTask {
    pub fn new(
        parent_id: Option<&String>,
        project_id: &str,
        content: &str,
        due_date: Option<NaiveDate>,
        description: Option<String>,
        priority: Option<i32>,
        assignee: Option<String>,
        labels: Vec<String>
    ) -> TodoistTask {
        TodoistTask {
            id: None,
            content: content.to_string(),
            description,
            priority,
            project_id: project_id.to_string(),
            parent_id: parent_id.cloned(),
            due_string: due_date.map(|d| d.format("%Y-%m-%d").to_string()),
            assignee,
            labels,
        }
    }
}
