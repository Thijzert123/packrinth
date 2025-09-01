---
title: Configuration reference
layout: default
nav_order: 3
---

1. TOC
{:toc}

# `modpack.json`

| Key                 | Type                              | Default                        | Description                                                                                             |
|---------------------|-----------------------------------|:-------------------------------|---------------------------------------------------------------------------------------------------------|
| `pack_format`       | number 0-65535                    | 1                              | The format of the configuration. This helps Packrinth decide whether to update to a new version or not. |
| `name`              | string                            | My Modrinth modpack            | The name of the modpack.                                                                                |
| `summary`           | string                            | Short summary for this modpack | A sort summary of the modpack.                                                                          |
| `author`            | string                            | John Doe                       | The author of the modpack.                                                                              |
| `require_all`       | boolean                           | `false`                        | Whether all projects are set to _required_ for all environments.                                        |
| `auto_dependencies` | boolean                           | `false`                        | Whether Packrinth should automatically add dependencies for projects.                                   |
| `branches`          | string array                      | _empty_                        | All branch names.                                                                                       |
| `projects`          | [project object](#project-object) | _empty_                        | All projects of the modpack.                                                                            |

## `project` object

| Key          | Type                                                | Default | Description                                                                                            |
|--------------|-----------------------------------------------------|---------|--------------------------------------------------------------------------------------------------------|
| _project id_ | [project settings object](#project-settings-object) | _empty_ | This project ID must be from Modrinth. It has to be the actual id (4ffh482k) or the slug (fabric-api). |

### `project settings` object

| Key                            | Type                                                | Default       | Description                                                                                                                               |
|--------------------------------|-----------------------------------------------------|---------------|-------------------------------------------------------------------------------------------------------------------------------------------|
| `version_overrides` (optional) | [version override object](#version-override-object) | _not present_ | The version overrides for a project. This allows you to specify the exact version that a branch has to use for the project.               |
| `include` (optional)           | string array                                        | _not present_ | All the branches that are allowed to include the project. No other branches are allowed to do so. Not compatible with `exclude`.          |
| `exclude` (optional)           | string array                                        | _not present_ | All the branches that are NOT allowed to include the project. All the other branches are allowed to do so. Not compatible with `include`. |

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