# Change Log
All notable changes to this project will be documented in this file.
 
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [0.9.1] - 2022-02-24
### Fixed
- Fix link to [API Documentation on docs.rs](https://docs.rs/kalgan).
- Fix bug in ```kalgan::http::response::Response``` when working behind a reverse proxy.
- Fix bug in ```kalgan::http::request::Request``` to work with case insesitive requests.
 
### Added
- Add this changelog.
- Add links in internal HTML error template.

### Changed
- Update ```kalgan-*``` crates dependencies.
- Improve ```kalgan::http::request::Request``` debug console output.
