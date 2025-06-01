use schemars::JsonSchema;
use serde::Deserialize;
use std::fs;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const REPO_NAME: &str = "jmylchreest/kagimcp-zed";
const BINARY_NAME: &str = "kagi-mcp-server";

#[derive(Debug, Deserialize, JsonSchema)]
struct KagiContextServerSettings {
    kagi_api_key: String,
    #[serde(default)]
    kagi_summarizer_engine: Option<String>,
    #[serde(default = "default_search_api_version")]
    kagi_search_api_version: String,
    #[serde(default = "default_summarizer_api_version")]
    kagi_summarizer_api_version: String,
    #[serde(default = "default_fastgpt_api_version")]
    kagi_fastgpt_api_version: String,
    #[serde(default = "default_enrich_api_version")]
    kagi_enrich_api_version: String,
}

// Default API versions
fn default_search_api_version() -> String {
    "v0".to_string()
}

fn default_summarizer_api_version() -> String {
    "v0".to_string()
}

fn default_fastgpt_api_version() -> String {
    "v0".to_string()
}

fn default_enrich_api_version() -> String {
    "v0".to_string()
}



struct KagiModelContextExtension {
    cached_binary_path: Option<String>,
}

impl KagiModelContextExtension {
    fn context_server_binary_path(
        &mut self,
        _context_server_id: &ContextServerId,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        // Fetch the latest release from GitHub
        let release_version: &str = &format!("v{}", env!("CARGO_PKG_VERSION"));
        let release = match zed::github_release_by_tag_name(REPO_NAME, release_version) {
            Ok(release) => release,
            Err(e) => {
                let url = format!(
                    "https://api.github.com/repos/{}/releases/tags/{}",
                    REPO_NAME, release_version
                );
                return Err(format!("Failed to fetch release from {}: {}", url, e).into());
            }
        };

        // Define which asset we're looking for
        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "{BINARY_NAME}_{os}_{arch}.{ext}",
            arch = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X86 => "i386",
                zed::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zed::Os::Mac => "darwin",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "windows",
            },
            ext = match platform {
                zed::Os::Mac | zed::Os::Linux => "tgz",
                zed::Os::Windows => "zip",
            }
        );

        // // Print all available assets for debugging
        // println!("Available assets for kagi-mcp-server:");
        // for available_asset in &release.assets {
        //     println!("  - {}", available_asset.name);
        // }

        // Find that asset
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("{BINARY_NAME}-{}", release.version);
        fs::create_dir_all(&version_dir)
            .map_err(|err| format!("failed to create directory '{version_dir}': {err}"))?;
        let binary_path = format!("{version_dir}/{BINARY_NAME}");

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            let file_kind = match platform {
                zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
                zed::Os::Windows => zed::DownloadedFileType::Zip,
            };

            zed::download_file(&asset.download_url, &version_dir, file_kind)
                .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            // Remove old versions
            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for KagiModelContextExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let settings = ContextServerSettings::for_project("kagimcp", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `kagi_api_key` setting".into());
        };
        let settings: KagiContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        let mut env = vec![("KAGI_API_KEY".into(), settings.kagi_api_key)];

        if let Some(engine) = settings.kagi_summarizer_engine {
            env.push(("KAGI_SUMMARIZER_ENGINE".into(), engine));
        }
        
        // Add API version environment variables
        env.push(("KAGI_SEARCH_API_VERSION".into(), settings.kagi_search_api_version));
        env.push(("KAGI_SUMMARIZER_API_VERSION".into(), settings.kagi_summarizer_api_version));
        env.push(("KAGI_FASTGPT_API_VERSION".into(), settings.kagi_fastgpt_api_version));
        env.push(("KAGI_ENRICH_API_VERSION".into(), settings.kagi_enrich_api_version));

        Ok(Command {
            command: self.context_server_binary_path(context_server_id)?,
            args: vec![],
            env,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(KagiContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(KagiModelContextExtension);
