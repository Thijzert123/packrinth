use packrinth::PackrinthError;
use packrinth::config::{BranchConfig, MainLoader, ProjectSettings};
use packrinth::modrinth::{Env, File, FileHashes, FileResult, Project, ProjectType, SideSupport};

#[test]
fn project_from_id() -> Result<(), PackrinthError> {
    let expected_project = Project {
        id: "P7dR8mSH".to_string(),
        title: "Fabric API".to_string(),
        server_side: SideSupport::Optional,
        client_side: SideSupport::Optional,
        project_type: ProjectType::Mod,
    };
    let project = Project::from_id("fabric-api")?;

    assert_eq!(expected_project, project);
    Ok(())
}

#[test]
fn file_from_project() {
    let branch_config = BranchConfig {
        version: "test".to_string(),
        minecraft_version: "1.17.1".to_string(),
        acceptable_minecraft_versions: vec![],
        mod_loader: Some(MainLoader::Fabric),
        loader_version: Some("0.17.2".to_string()),
        acceptable_loaders: vec![],
        manual_files: vec![],
    };
    let project_settings = ProjectSettings {
        version_overrides: None,
        include_or_exclude: None,
    };

    // Test with all versions (alpha and beta included)
    let file = File::from_project(
        &"test".to_string(),
        &branch_config,
        "fabric-api",
        &project_settings,
        false,
        false,
    );
    assert_eq!(FileResult::Ok {
        file: File {
            project_name: "Fabric API".to_string(),
            path: "mods/fabric-api-0.46.1+1.17.jar".to_string(),
            hashes: FileHashes { sha1: "a7d86f36c5b27bdb0008a84c3ce91e2b095a2834".to_string(), sha512: "3348593a87d0f9dd7b10e2583c2d02c8120a0eb78c4c220fe6833d3cb36bc1820fa719d546e8cf9e6acd2035918a95589484a4eccc52d58e8d319a8979b8473b".to_string() },
            env: Some(Env {
                client: SideSupport::Optional,
                server: SideSupport::Optional,
            }),
            downloads: vec!["https://cdn.modrinth.com/data/P7dR8mSH/versions/0.46.1%2B1.17/fabric-api-0.46.1%2B1.17.jar".to_string()],
            file_size: 1207739,
        },
        dependencies: vec![],
        project_id: "P7dR8mSH".to_string(),
    }, file);

    // Test without alpha and beta
    let file = File::from_project(
        &"test".to_string(),
        &branch_config,
        "fabric-api",
        &project_settings,
        true,
        true,
    );
    assert_eq!(FileResult::Ok {
        file: File {
            project_name: "Fabric API".to_string(),
            path: "mods/fabric-api-0.46.1+1.17.jar".to_string(),
            hashes: FileHashes { sha1: "a7d86f36c5b27bdb0008a84c3ce91e2b095a2834".to_string(), sha512: "3348593a87d0f9dd7b10e2583c2d02c8120a0eb78c4c220fe6833d3cb36bc1820fa719d546e8cf9e6acd2035918a95589484a4eccc52d58e8d319a8979b8473b".to_string() },
            env: Some(Env {
                client: SideSupport::Optional,
                server: SideSupport::Optional,
            }),
            downloads: vec!["https://cdn.modrinth.com/data/P7dR8mSH/versions/0.46.1%2B1.17/fabric-api-0.46.1%2B1.17.jar".to_string()],
            file_size: 1207739,
        },
        dependencies: vec![],
        project_id: "P7dR8mSH".to_string(),
    }, file);
}
