use crate::config::{BranchConfig, BranchFiles, MainLoader, Modpack};
use crate::modrinth::{Dependencies, MrPack};
use anyhow::Result;
use log::{debug, trace};
use reqwest::blocking::Client;
use std::fs::File;
use std::io;
use std::io::Write;
use std::sync::OnceLock;
use walkdir::WalkDir;
use zip::ZipWriter;
use zip::write::{FileOptions, SimpleFileOptions};

const MODRINTH_API_BASE_URL: &str = "https://api.modrinth.com/v2";
static CLIENT: OnceLock<Client> = OnceLock::new();
const USER_AGENT: &str = concat!(
    "Thijzert123",
    "/",
    "packrinth",
    "/",
    env!("CARGO_PKG_VERSION")
);

pub fn request_text<T: ToString>(api_endpoint: T) -> Result<String, Box<dyn std::error::Error>> {
    let client = CLIENT.get_or_init(|| {
        Client::builder()
            .user_agent(USER_AGENT)
            .build()
            .expect("Failed to build client")
    });

    let full_url = MODRINTH_API_BASE_URL.to_string() + api_endpoint.to_string().as_str();
    debug!("Making GET request to {}", full_url);
    let text = client.get(&full_url).send()?.text()?;
    trace!("Got text {} from {}", text, full_url);
    Ok(text)
}

/// The current most recent pack format of a .mrpack file.
const MODRINTH_PACK_FORMAT: u16 = 1;
/// The game to put in the mrpack.
const GAME: &str = "minecraft";
const MRPACK_CONFIG_FILE_NAME: &str = "modrinth.index.json";
const OVERRIDES_DIR_NAME: &str = "overrides";

pub fn export_to_mrpack(modpack: &Modpack, branch: &String) -> Result<()> {
    let branch_config = BranchConfig::from_directory(&modpack.directory, branch)?;
    let branch_files = BranchFiles::from_directory(&modpack.directory, branch)?;

    let mrpack_file_name = format!("{}_{}.mrpack", modpack.name, branch_config.version);
    let mrpack_path = modpack.directory.join(&mrpack_file_name);

    let mrpack = MrPack {
        format_version: MODRINTH_PACK_FORMAT,
        game: GAME.to_string(),
        version_id: branch_config.version.clone(),
        name: modpack.name.clone(),
        summary: Some(modpack.summary.clone()),
        files: branch_files.files,
        dependencies: create_dependencies(branch_config),
    };

    let mrpack_json = serde_json::to_string_pretty(&mrpack)?;
    let options = SimpleFileOptions::default();

    let mut zip = ZipWriter::new(File::create(&mrpack_path)?);
    zip.start_file(MRPACK_CONFIG_FILE_NAME, options)?;
    zip.write_all(mrpack_json.as_bytes())?;

    let branch_dir = modpack.directory.join(branch);
    // Loop every file/dir in the overrides dir
    for entry in WalkDir::new(branch_dir.join(OVERRIDES_DIR_NAME)) {
        let entry = entry?;
        // The actual path on the file system
        let path = entry.path();
        // The path the file will be in the zip (/ being the root of the zip)
        let zip_path = path.strip_prefix(&branch_dir)?.to_str().unwrap();

        if path.is_file() {
            zip.start_file(zip_path, options)?;
            let mut buffer = Vec::new();
            io::copy(&mut File::open(path)?, &mut buffer)?;
            zip.write_all(&buffer)?;
        } else if path.is_dir() {
            zip.add_directory(zip_path, options)?;
        }
    }

    zip.finish()?;

    Ok(())
}

fn create_dependencies(branch_config: BranchConfig) -> Dependencies {
    let mut forge = None;
    let mut neoforge = None;
    let mut fabric_loader = None;
    let mut quilt_loader = None;

    match branch_config.main_mod_loader {
        MainLoader::Forge => forge = Some(branch_config.loader_version),
        MainLoader::NeoForge => neoforge = Some(branch_config.loader_version),
        MainLoader::Fabric => fabric_loader = Some(branch_config.loader_version),
        MainLoader::Quilt => quilt_loader = Some(branch_config.loader_version),
    }

    Dependencies {
        minecraft: branch_config.main_minecraft_version,
        forge,
        neoforge,
        fabric_loader,
        quilt_loader,
    }
}
