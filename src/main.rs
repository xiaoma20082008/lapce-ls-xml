// Deny usage of print and eprint as it won't have same result
// in WASI as if doing in standard program, you must really know
// what you are doing to disable that lint (and you don't know)
#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]

use std::path::PathBuf;

use anyhow::Result;
use lapce_plugin::{
    Http,
    LapcePlugin, PLUGIN_RPC,
    psp_types::{
        lsp_types::{DocumentFilter, DocumentSelector, InitializeParams, MessageType, request::Initialize, Url},
        Request,
    },
    register_plugin,
    VoltEnvironment,
};
use serde_json::Value;

#[derive(Default)]
struct State {}

register_plugin!(State);

fn initialize(params: InitializeParams) -> Result<()> {
    let document_selector: DocumentSelector = vec![DocumentFilter {
        // lsp language id
        language: Some(String::from("xml")),
        // glob pattern
        pattern: Some(String::from("**/*.xml")),
        // like file:
        scheme: None,
    }];
    let mut server_args = vec![];
    let mut lemminx_version = "0.23.2";
    let mut lemminx_port = 5008;
    let mut lemminx_args = "";

    // Check for user specified LSP server path
    // ```
    // [lapce-lsp-xml]
    // serverPath = "[path or filename]"
    // serverArgs = ["--arg1", "--arg2"]
    //
    // [lapce-lsp-xml.lemminx]
    // version = "0.23.2"
    // port = 5008
    // args = ""
    // ```
    if let Some(options) = params.initialization_options.as_ref() {
        if let Some(lsp) = options.get("lapce-lsp-xml") {
            if let Some(args) = lsp.get("serverArgs") {
                if let Some(args) = args.as_array() {
                    if !args.is_empty() {
                        server_args = vec![];
                    }
                    for arg in args {
                        if let Some(arg) = arg.as_str() {
                            server_args.push(arg.to_string());
                        }
                    }
                }
            }

            if let Some(server_path) = lsp.get("serverPath") {
                if let Some(server_path) = server_path.as_str() {
                    if !server_path.is_empty() {
                        let server_uri = Url::parse(&format!("urn:{}", server_path))?;
                        PLUGIN_RPC.start_lsp(
                            server_uri,
                            server_args,
                            document_selector,
                            params.initialization_options,
                        );
                        return Ok(());
                    }
                }
            }

            if let Some(lemminx) = lsp.get("lemminx") {
                if let Some(version) = lemminx.get("version") {
                    if let Some(version) = version.as_str() {
                        if !version.is_empty() {
                            lemminx_version = version.as_str();
                        }
                    }
                }

                if let Some(port) = lemminx.get("port") {
                    if let Some(port) = port.as_i64() {
                        lemminx_port = port;
                    }
                }

                if let Some(args) = lemminx.get("args") {
                    if let Some(args) = args.as_str() {
                        if !args.is_empty() {
                            lemminx_args = args;
                        }
                    }
                }
            }
        }
    }

    // Download URL
    let lemminx_uber = "org.eclipse.lemminx-uber.jar";
    let lemminx_download_url = format!("https://download.eclipse.org/lemminx/releases/{lemminx_version}/{lemminx_uber}");

    // see lapce_plugin::Http for available API to download files
    if !PathBuf::from(lemminx_uber).exists() {
        let mut resp = Http::get(&lemminx_download_url)?;
        let body = resp.body_read_all()?;
        std::fs::write(&lombok_jar, body)?;
    }

    let mut args = server_args.join(" ");
    if args.is_empty() {
        lemminx_args = "-Xmx32M -Xms32M";
    }
    // Plugin working directory
    let volt_uri = VoltEnvironment::uri()?;
    let base_path = Url::parse(&volt_uri)?;
    let server_uri = base_path.join(&format!("java -server {args} -jar {lemminx_uber}"))?;

    PLUGIN_RPC.start_lsp(
        server_uri,
        server_args,
        document_selector,
        params.initialization_options,
    );

    return Ok(());
}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        #[allow(clippy::single_match)]
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                if let Err(e) = initialize(params) {
                    PLUGIN_RPC.window_show_message(MessageType::ERROR, format!("plugin returned with error: {e}"))
                }
            }
            _ => {}
        }
    }
}
