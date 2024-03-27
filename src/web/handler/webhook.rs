use crate::crypto::verify_signature;
use crate::web::app::Context;
use crate::web::object::base::*;
use axum::{extract::Extension, http::header::HeaderMap};
use reqwest::blocking::{multipart::Form, Client};
use serde_json::{json, Value};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use tempfile::tempdir;

pub async fn post_webhooks_github<'a>(
    Extension(ctx): Extension<Context>,
    headers: HeaderMap,
    body: String,
    // Json(payload): Json<serde_json::Value>,
) -> Result<HttpSuccess<'a>, HttpError<'a>> {
    match headers.get("X-Hub-Signature-256") {
        Some(v) => {
            let signature = v.to_str().unwrap_or("");

            match verify_signature(
                ctx.config.github_webhook_secret.as_bytes(),
                body.as_bytes(),
                signature
                    .strip_prefix("sha256=")
                    .unwrap_or_default()
                    .as_bytes(),
            ) {
                Ok(matched) => {
                    if !matched {
                        return Err(HttpError::Unauthorized);
                    }
                }
                Err(e) => {
                    return Err(HttpError::Anyhow(e));
                }
            }
        }
        None => return Err(HttpError::Unauthorized),
    }

    match headers.get("X-GitHub-Event") {
        Some(v) => {
            let event = v.to_str().unwrap_or("");
            match event {
                "ping" => return Ok(HttpSuccess::Success("ping event ok")),
                "push" => {
                    let payload: Value = serde_json::from_str(&body).unwrap_or(Value::Null);
                    if payload == Value::Null {
                        return Err(HttpError::BadRequest("payload"));
                    }

                    let branch = payload["ref"]
                        .as_str()
                        .unwrap_or_default()
                        .strip_prefix("refs/heads/")
                        .unwrap_or_default()
                        .to_string();

                    if branch.is_empty() {
                        return Err(HttpError::BadRequest("ref"));
                    }

                    if branch == ctx.config.github_watch_push_branch {
                        let repo = payload["repository"]["full_name"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();
                        if repo.is_empty() {
                            return Err(HttpError::BadRequest("repository.full_name"));
                        }

                        let commit_id = payload["after"].as_str().unwrap_or_default().to_string();
                        if commit_id.is_empty() {
                            return Err(HttpError::BadRequest("after"));
                        }

                        let head_commit_url = payload["head_commit"]["url"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();
                        if head_commit_url.is_empty() {
                            return Err(HttpError::BadRequest("head_commit.url"));
                        }

                        let head_commit_committer_name = payload["head_commit"]["committer"]
                            ["name"]
                            .as_str()
                            .unwrap_or_default()
                            .to_string();
                        if head_commit_committer_name.is_empty() {
                            return Err(HttpError::BadRequest("head_commit.committer.name"));
                        }

                        tokio::spawn(async move {
                            let output = Command::new(&ctx.config.build_entry_script_path)
                                .arg(&commit_id)
                                .output()
                                .expect("failed to execute the process");

                            let stdout_str = String::from_utf8(output.stdout)
                                .expect("failed to process stdout content");
                            let stderr_str = String::from_utf8(output.stderr)
                                .expect("failed to process stderr content");

                            let temp_dir = tempdir().expect("failed to create temporary directory");
                            let stdout_file_path = temp_dir.path().join("stdout.txt");
                            let stderr_file_path = temp_dir.path().join("stderr.txt");
                            let mut stdout_file = File::create(&stdout_file_path)
                                .expect("failed to create stdout file");
                            let mut stderr_file = File::create(&stderr_file_path)
                                .expect("failed to create stderr file");
                            write!(stdout_file, "{}", stdout_str)
                                .expect("failed to write stdout file");
                            write!(stderr_file, "{}", stderr_str)
                                .expect("failed to write stderr file");

                            let payload_json = if output.status.success() {
                                json!({
                                    "embeds": [{
                                        "title": "Deployment Success",
                                        "url": head_commit_url,
                                        "color": 10731148, // #a3be8c
                                        "fields": [
                                            { "name": "Repository", "value": repo},
                                            { "name": "Branch", "value": branch, "inline": true},
                                            { "name": "Commit ID", "value": &commit_id[..7], "inline": true},
                                            { "name": "Committer", "value": head_commit_committer_name }
                                        ]
                                    }]
                                })
                            } else {
                                json!({
                                    "embeds": [{
                                        "title": "Deployment Failed",
                                        "url": head_commit_url,
                                        "color": 12542314, // "#bf616a"
                                        "fields": [
                                            { "name": "Repository", "value": repo},
                                            { "name": "Branch", "value": branch, "inline": true},
                                            { "name": "Commit ID", "value": &commit_id[..7], "inline": true},
                                            { "name": "Committer", "value": head_commit_committer_name }
                                        ]
                                    }]
                                })
                            }
                            .to_string();

                            let form = Form::new()
                                .text("payload_json", payload_json)
                                .file("file1", &stdout_file_path)
                                .expect("failed to attach file1")
                                .file("file2", &stderr_file_path)
                                .expect("failed to attach file2");

                            let _resp = Client::new()
                                .post(&ctx.config.discord_webhook_url)
                                .multipart(form)
                                .send()
                                .expect("failed to send the request to Discord");

                            drop(stdout_file);
                            drop(stderr_file);
                            let _ = temp_dir.close();
                        });

                        return Ok(HttpSuccess::Success("push event accepted"));
                    }
                    return Ok(HttpSuccess::Success("unhandled branch"));
                }
                _ => return Ok(HttpSuccess::Success("unhandled event")),
            }
        }
        None => return Ok(HttpSuccess::Success("no event")),
    }
}
