---
title: Configuration reference
layout: default
nav_order: 7
---

# Configuration reference
{: .no_toc }

## Table of contents
{: .no_toc .text-delta }

1. TOC
{:toc}

---

# `modpack.json`

| Key                 | Type                                                | Default                          | Description                                                                                             |
|---------------------|-----------------------------------------------------|:---------------------------------|---------------------------------------------------------------------------------------------------------|
| `pack_format`       | `u16` number                                        | `1`                              | The format of the configuration. This helps Packrinth decide whether to update to a new version or not. |
| `name`              | string                                              | `My Modrinth modpack`            | The name of the modpack.                                                                                |
| `summary`           | string                                              | `Short summary for this modpack` | A sort summary of the modpack.                                                                          |
| `author`            | string                                              | `John Doe`                       | The author of the modpack.                                                                              |
| `require_all`       | boolean                                             | `false`                          | Whether all projects are set to _required_ for all environments.                                        |
| `auto_dependencies` | boolean                                             | `false`                          | Whether Packrinth should automatically add dependencies for projects.                                   |
| `branches`          | string array                                        | _empty_                          | All branch names.                                                                                       |
| `projects`          | [`modpack project` object](#modpack-project-object) | _empty_                          | All projects of the modpack.                                                                            |

## `modpack project` object

| Key          | Type                                                  | Default | Description                                                                                            |
|--------------|-------------------------------------------------------|---------|--------------------------------------------------------------------------------------------------------|
| _project id_ | [`project settings` object](#project-settings-object) | _empty_ | This project ID must be from Modrinth. It has to be the actual id (4ffh482k) or the slug (fabric-api). |

### `project settings` object

| Key                            | Type                                                  | Default       | Description                                                                                                                               |
|--------------------------------|-------------------------------------------------------|---------------|-------------------------------------------------------------------------------------------------------------------------------------------|
| `version_overrides` (optional) | [`version override` object](#version-override-object) | _not present_ | The version overrides for a project. This allows you to specify the exact version that a branch has to use for the project.               |
| `include` (optional)           | string array                                          | _not present_ | All the branches that are allowed to include the project. No other branches are allowed to do so. Not compatible with `exclude`.          |
| `exclude` (optional)           | string array                                          | _not present_ | All the branches that are NOT allowed to include the project. All the other branches are allowed to do so. Not compatible with `include`. |

#### `version override` object

| Key           | Type   | Default       | Description                                                   |
|---------------|--------|---------------|---------------------------------------------------------------|
| _branch name_ | string | _not present_ | The branch to override with a Modrinth version ID (423fue84). |

## Default JSON
```json
{
        "pack_format": 1,
        "name": "My Modrinth modpack",
        "summary": "Short summary for this modpack",
        "author": "John Doe",
        "require_all": false,
        "auto_dependencies": false,
        "branches": [],
        "projects": {}
}
```

## Example JSON
```json
{
	"pack_format": 1,
	"name": "Super Amazing Modpack",
	"summary": "Modpack focused being amazing.",
	"author": "Mr. Awesome",
	"require_all": true,
	"auto_dependencies": true,
	"branches": [
		"1.21.4",
		"1.20.4",
		"1.20.1"
	],
	"projects": {
		"better-stats": {},
		"yosbr": {
			"version_overrides": {
				"1.20.1": "KMOzdYko",
				"1.20.4": "KMOzdYko"
			}
		},
		"visuals": {
			"include": [
				"1.20.1",
				"1.20.4",
				"1.21.4"
			]
		},
		"noisium": {
			"exclude": [
				"1.20.1",
				"1.20.4"
			]
		}
	}
}
```

# `branch.json`

| Key                             | Type                                | Default                | Description                                                                       |
|---------------------------------|-------------------------------------|:-----------------------|-----------------------------------------------------------------------------------|
| `version`                       | string                              | `1.0.0-fabric`         | The version of the branch that should be appended to the final modpack file.      |
| `minecraft_version`             | string                              | `1.21.8`               | The Minecraft version to use with this modpack.                                   |
| `acceptable_minecraft_versions` | string array                        | `1.21.6`, `1.21.7`     | Minecraft versions that are acceptable for downloading Modrinth mods.             |
| `mod_loader` (optional)         | [main mod loader](#main-mod-loader) | `fabric`               | The mod loader to install alongside the modpack.                                  |
| `loader_version` (optional)     | string                              | `0.17.2`               | The version of the mod loader. Has to be present if `mod_loader` is also present. |
| `acceptable_loaders`            | [loader](#other-loaders) array      | `minecraft`, `vanilla` | All the loaders that are acceptable for downloading Modrinth mods.                |
| `manual_files`                  | [`file` object](#file-object) array | _not present_          | Manual files to add while updating the branch files.                              |

## Main mod loader

| Name     | Configuration value |
|----------|---------------------|
| Fabric   | `fabric`            |
| Forge    | `forge`             |
| NeoForge | `neoforge`          |
| Quilt    | `quilt`             |

## Other loaders

| Name                 | Configuration value |
|----------------------|---------------------|
| Minecraft            | `minecraft`         |
| Fabric               | `fabric`            |
| Forge                | `forge`             |
| NeoForge             | `neoforge`          |
| Quilt                | `quilt`             |
| Babric               | `babric`            |
| BTA (Babric)         | `bta-babric`        |
| Java Agent           | `java-agent`        |
| Legacy Fabric        | `legacy-fabric`     |
| LiteLoader           | `liteloader`        |
| Risugami's ModLoader | `modloader`         |
| NilLoader            | `nilloader`         |
| Ornithe              | `ornithe`           |
| Rift                 | `rift`              |
| Canvas               | `canvas`            |
| Iris                 | `iris`              |
| OptiFine             | `optifine`          |
| Vanilla Shader       | `vanilla`           |
| Bukkit               | `bukkit`            |
| Folia                | `folia`             |
| Paper                | `paper`             |
| Purpur               | `purpur`            |
| Spigot               | `spigot`            |
| Sponge               | `sponge`            |
| BungeeCord           | `bungeecord`        |
| Velocity             | `velocity`          |
| Waterfall            | `waterfall`         |

## Default JSON
```json
{
        "version": "1.0.0-fabric",
        "minecraft_version": "1.21.8",
        "acceptable_minecraft_versions": [
                "1.21.6",
                "1.21.7"
        ],
        "mod_loader": "fabric",
        "loader_version": "0.17.2",
        "acceptable_loaders": [
                "minecraft",
                "vanilla"
        ]
}
```

## Example JSON
```json
{
        "version": "2.34.2-fabric+beta1",
        "minecraft_version": "1.21.4",
        "acceptable_minecraft_versions": [
                "1.21.3"
        ],
        "mod_loader": "fabric",
        "loader_version": "0.16.2",
        "acceptable_loaders": [
                "minecraft",
                "vanilla",
                "iris",
                "canvas"
        ],
        "manual_files": [
                {
                        "path": "mods/Flashback-0.37.0-for-MC1.21.4.jar",
                        "hashes": {
                                "sha1": "73a6ddfe86e03954f7bd7699d2ff68af791b6715",
                                "sha512": "3f8cb625a169700ce8ec09becbeaff60c416d1b7b30b31c7665758622f001734ecfef624196ec2291fd05eda993fd88cbac065cb66f16e81e424b8b25d1b9161"
                        },
                        "env": {
                                "client": "required",
                                "server": "required"
                        },
                        "downloads": [
                                "https://cdn.modrinth.com/data/4das1Fjq/versions/LsJ9G3Ab/Flashback-0.37.0-for-MC1.21.4.jar"
                        ],
                        "fileSize": 210926730
                }
        ]
}
```

# `.branch_files.json`

{: .important}
This file is never intended to edit manually or with third party software. You should always use the Packrinth CLI
or library. If you still find limitations, [file an issue on GitHub](https://github.com/Thijzert123/packrinth/issues).

| Key        | Type                                                                | Default                                                                                                                      | Description                                                                                                 |
|------------|---------------------------------------------------------------------|:-----------------------------------------------------------------------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------|
| `info`     | string                                                              | `This file is managed by Packrinth and not intended for manual editing. You should, however, add it to your Git repository.` | Information about manually editing this file. This fields gets reset by Packrinth everytime the file saves! |
| `projects` | [`branch files project` object](#branch-files-project-object) array | _empty_                                                                                                                      | Project information used for generating a documentation page.                                               |
| `files`    | [`file` object](#file-object) array                                 | _empty_                                                                                                                      | All the final files to be put in the modpack.                                                               |

## `branch files project` object

| Key             | Type   | Default          | Description                             |
|-----------------|--------|:-----------------|-----------------------------------------|
| `name`          | string | _not applicable_ | The human-friendly name of the project. |
| `id` (optional) | string | _not applicable_ | The project ID on Modrinth.             |

## `file` object

| Key              | Type                                        | Default          | Description                                                                                 |
|------------------|---------------------------------------------|:-----------------|---------------------------------------------------------------------------------------------|
| `path`           | string                                      | _not applicable_ | The path the file has to go to when installing the modpack from the `.minecraft` directory. |
| `hashes`         | [`file hashes` object](#file-hashes-object) | _not applicable_ | Hashes for the project file.                                                                |
| `env` (optional) | [`env` object](#env-object)                 | _not applicable_ | Information about the support for environment.                                              |
| `downloads`      | string array                                | _not applicable_ | Array of URLs where the file can be downloaded from.                                        |
| `file_size`      | `u64` number                                | _not applicable_ | The file size in bytes.                                                                     |

### `file hashes` object

| Key      | Type   | Default          | Description             |
|----------|--------|:-----------------|-------------------------|
| `sha1`   | string | _not applicable_ | `sha1` hash for file.   |
| `sha512` | string | _not applicable_ | `sha512` hash for file. |

### `env` object

| Key      | Type                                          | Default          | Description                                      |
|----------|-----------------------------------------------|:-----------------|--------------------------------------------------|
| `client` | [`side support` object](#side-support-object) | _not applicable_ | The projects support for the client environment. |
| `server` | [`side support` object](#side-support-object) | _not applicable_ | The projects support for the server environment. |

#### `side support` object

| Name        | Configuration value |
|-------------|---------------------|
| Required    | `required`          |
| Optional    | `optional`          |
| Unsupported | `unsupported`       |

## Example JSON
```json
{
	"info": "This file is managed by Packrinth and not intended for manual editing. You should, however, add it to your Git repository.",
	"projects": [
		{
			"name": "Better Advancements",
			"id": "better-advancements"
		},
		{
			"name": "Client Sort",
			"id": "clientsort"
		}
	],
	"files": [
		{
			"path": "mods/BetterAdvancements-Fabric-1.21.1-0.4.3.21.jar",
			"hashes": {
				"sha1": "f6c6193c8711340e52645b65dbb7ffde7fbf6f30",
				"sha512": "7f0eaf623a357b5fab04faef4133651ec0972206f404619350d71ade632132d041ee1a93ab287ef16d18b99d73cae508da5fe8766ecc40f20bd131ba20edc0dc"
			},
			"env": {
				"client": "required",
				"server": "required"
			},
			"downloads": [
				"https://cdn.modrinth.com/data/Q2OqKxDG/versions/j80BmLRo/BetterAdvancements-Fabric-1.21.1-0.4.3.21.jar"
			],
			"fileSize": 100151
		},
		{
			"path": "mods/clientsort-fabric-2.0.0-beta.14+1.21.1.jar",
			"hashes": {
				"sha1": "c6c58e6cec22fe04c83e0973b55c24baeac298fc",
				"sha512": "df86b4876ff861e61d832e767cc32c00ca5a83474ec42d18f1cf22c08b58a689f76d98d64f95aae1a8b234e474d664baa3d4c2314575079438716b830aa3a605"
			},
			"env": {
				"client": "required",
				"server": "required"
			},
			"downloads": [
				"https://cdn.modrinth.com/data/K0AkAin6/versions/Yp64RITq/clientsort-fabric-2.0.0-beta.14%2B1.21.1.jar"
			],
			"fileSize": 279742
		}
	]
}
```