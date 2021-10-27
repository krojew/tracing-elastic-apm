# Changelog

## [3.2.0]

- Fixed `tracing-subscriber` compatibility.

## [3.1.0]

- Added support for specifying custom trace id to new spans via `config::TRACE_ID_FIELD_NAME` field.

## [3.0.0]

### New

- Added possibility to specify root CA path.

### Fixed

- Fixed sending invalid api key to APM.

## [2.2.0]

- Added possibility to allow invalid certificates.

## [2.1.0]

### New

- Added `default-tls` and `rustls-tls` features.

## [2.0.0]

### Changed

- Updated to `tokio` 1.0.

## [1.0.1]

### Fixed

- Fixed duration computation when entering/exiting spans.
