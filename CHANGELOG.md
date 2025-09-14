# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.3](https://github.com/Thijzert123/packrinth/compare/v0.7.2...v0.7.3) - 2025-09-14

### Changed

- Show error when removing a branch directory failed

### Fixed

- Removing a branch not modifying the modpack configuration file

## [0.7.2](https://github.com/Thijzert123/packrinth/compare/v0.7.1...v0.7.2) - 2025-09-14

### Added

- Possibility to set the modpack name when initializing a new Packrinth modpack

### Changed

- When updating projects with verbose output, the test `dependency` will be shown if an automatic dependency was successfully added

## [0.7.1](https://github.com/Thijzert123/packrinth/compare/v0.7.0...v0.7.1) - 2025-09-11

### Changed

- Show a progress percentage when a progress bar is used instead of a number

## [0.7.0](https://github.com/Thijzert123/packrinth/compare/v0.6.0...v0.7.0) - 2025-09-10

### Changed

- Made some configuration file name constants public
- Packrinth will now work if `modpack.json` isn't in the current directory, but it is in one of the parent directories
- Check for dirty Git repository when importing a modpack with adding projects enabled

## [0.6.0](https://github.com/Thijzert123/packrinth/compare/v0.5.0...v0.6.0) - 2025-09-08

### Added

- [**breaking**] Added `packrinth import` subcommand for importing a Modrinth pack to a Packrinth modpack
- Added `#[non_exhaustive]` attribute to `PackrinthError`
- Implemented `Default` for `BranchConfig`
- Added `save` method to `BranchConfig`

## [0.5.0](https://github.com/Thijzert123/packrinth/compare/v0.4.1...v0.5.0) - 2025-09-07

### Added

- Add `PartialEq` impl for some `modrinth` structs

### Changed

- The success message for adding version overrides has been changed
- [**breaking**] Add `PackrinthError::ModpackHasNoBranchesToUpdate`

### Fixed

- When a project is successfully added when updating, it will now display the slug instead of the Modrinth ID
- Don't show `What is included?` header in documentation when no projects are in any `.branch_files.json`

## [0.4.1](https://github.com/Thijzert123/packrinth/compare/v0.4.0...v0.4.1) - 2025-09-06

### Fixed

- Don't require a modpack configuration file anymore when generating shell completions

## [0.4.0](https://github.com/Thijzert123/packrinth/compare/v0.3.0...v0.4.0) - 2025-09-05

### Added

- Add `rust-version` to `Cargo.toml`

### Fixed

- Remove `doc project` subcommand and replace it with `doc` subcommand

## [0.3.0](https://github.com/Thijzert123/packrinth/compare/v0.2.1...v0.3.0) - 2025-09-04

### Added

- Add original error message to `PackrinthErrror::FailedToSerialize`
- [**breaking**] When an error is shown, the original error message containing more information is shown more often

### Changed

- [**breaking**] Move the `default` function of `BranchFiles` to trait `Default`

### Other

- Add documentation to library structs and functions

## [0.2.1](https://github.com/Thijzert123/packrinth/compare/v0.2.0...v0.2.1) - 2025-09-02

### Fixed

- Adding projects via CLI without inclusions or exclusions will now result in an empty object instead of `version_overrides` having a null-value

### Other

- Use rustls-tls for better cross-compilation

## [0.2.0](https://github.com/Thijzert123/packrinth/compare/v0.1.1...v0.2.0) - 2025-09-02

### Added

- Add version subcommand

### Fixed

- Main mod loader will now be shown when listing branches
- Allow not providing `mod_loader` and `loader_version` in `branch.json`

## [0.1.1](https://github.com/Thijzert123/packrinth/compare/v0.1.0...v0.1.1) - 2025-09-02

### Fixed

- Some vec arguments will now be required (empty isn't allowed anymore)
- Update default branch config to be more accurate

### Other

- Update docs
- Bump progress_bar from 1.2.1 to 1.3.0
