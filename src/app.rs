use crate::{app_settings, azure, git};
use crate::azure::AzureClient;
use crate::cli::parse_args;

pub async fn run() -> anyhow::Result<()> {
    let run_config = parse_args()?;
    let app_config = app_settings::AppConfig::load()?;

    let azure_client = azure::AzureClient::new(&app_config);

    let client = git::GitClient::new(run_config.target.to_str().unwrap());
    client.stage_changes()?;
    let diff = client.get_diff()?;

    let commit_message = generate_commit_message(&azure_client, &diff).await?;
    client.commit(commit_message.as_str())?;

    Ok(())
}

async fn generate_commit_message<'a>(azure_client: &AzureClient<'a>, diff: &str) -> anyhow::Result<String> {
    let response = azure_client
        .send_openai_request::<OpenAiResponse>(
            &OpenAiRequest {
                messages: vec![
                    Message {
                        role: String::from("system"),
                        content: String::from("I want you to generate a single line commit message for the following git diff. The commit message should be concise and descriptive of the changes made in the diff. Do not provide any additional explanations or comments. Do not surround the message with quotes."),
                    },
                    Message {
                        role: String::from("user"),
                        content: diff.to_string()
                    }
                ],
                max_tokens: 4096,
                temperature: 0,
                top_p: 1,
                model: "gpt-4o".to_string(),
            },
        )
        .await?;

    Ok(response.choices.first().unwrap().message.content.clone())
}

#[derive(serde::Serialize, serde::Deserialize)]
struct OpenAiRequest {
    pub messages: Vec<Message>,
    pub max_tokens: i64,
    pub temperature: i64,
    pub top_p: i64,
    pub model: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Message {
    pub role: String,
    pub content: String,
}
#[derive(serde::Serialize, serde::Deserialize)]
struct OpenAiCompletion {
    pub content: String,
    pub role: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Choice {
    pub index: i64,
    pub message: OpenAiCompletion,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct OpenAiResponse {
    pub choices: Vec<Choice>,
}