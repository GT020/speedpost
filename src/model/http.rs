//! HTTP service: turns a [`RequestTask`] into an [`HttpResponse`].

use std::sync::OnceLock;
use std::time::Instant;

/// A fully-resolved request, ready to be handed to the worker.
#[derive(Debug)]
pub struct RequestTask {
    /// The tab that dispatched this request; used to route the response back.
    pub tab_id: u64,
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

/// A completed request paired with the tab that dispatched it.
#[derive(Debug)]
pub struct RequestOutcome {
    pub tab_id: u64,
    pub response: HttpResponse,
}

/// The result of executing a [`RequestTask`], successful or not.
#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub duration_ms: u128,
    pub size: usize,
    pub error: Option<String>,
}

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

/// Executes a request, never failing: transport errors are captured in the
/// returned [`HttpResponse::error`] field.
pub async fn perform_request(task: RequestTask) -> HttpResponse {
    let client = CLIENT.get_or_init(reqwest::Client::new);
    let method =
        reqwest::Method::from_bytes(task.method.as_bytes()).unwrap_or(reqwest::Method::GET);

    let mut builder = client.request(method, &task.url);
    for (key, value) in &task.headers {
        builder = builder.header(key.as_str(), value.as_str());
    }
    if !task.body.is_empty() {
        builder = builder.body(task.body.clone());
    }

    let start = Instant::now();
    match builder.send().await {
        Ok(response) => {
            let status = response.status();
            let headers = response
                .headers()
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("<binary>").to_string()))
                .collect();
            let bytes = response.bytes().await.unwrap_or_default();
            let size = bytes.len();
            let body = String::from_utf8_lossy(&bytes).to_string();
            HttpResponse {
                status: status.as_u16(),
                status_text: status.canonical_reason().unwrap_or("").to_string(),
                headers,
                body,
                duration_ms: start.elapsed().as_millis(),
                size,
                error: None,
            }
        }
        Err(err) => HttpResponse {
            status: 0,
            status_text: String::new(),
            headers: Vec::new(),
            body: String::new(),
            duration_ms: start.elapsed().as_millis(),
            size: 0,
            error: Some(err.to_string()),
        },
    }
}

/// Percent-encodes a string for use in a URL query component.
pub fn urlencode(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(byte as char);
            }
            b' ' => out.push_str("%20"),
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}
