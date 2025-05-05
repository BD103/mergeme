# Changelog

All notable user-facing changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to [Semantic Versioning].

[Keep a Changelog]: https://keepachangelog.com/en/1.1.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html

## v0.1.1 - 2025-05-05

**All Changes**: [`v0.1.0...v0.1.1`](https://github.com/BD103/mergeme/compare/v0.1.0...v0.1.1)

### Fixed

- `#[derive(Merge)]` no longer passes field attributes to the generated partial struct.
    - A later release will support passing through both struct and field attributes to the partial struct, so this feature isn't going away forever.

## v0.1.0 - 2025-05-04

**All Changes**: [`v0.1.0`](https://github.com/BD103/mergeme/commits/v0.1.0)

### Added

- Initial commit! :)
