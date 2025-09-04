# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
