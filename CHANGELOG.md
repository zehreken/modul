# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- Added scene.rs. It is very basic currently but there is at least a container for
visualization objects.
- Added a basic camera.
- Added basic calculation for frame time.

### Changed
- Update miniquad, egui-miniquad and egui packages.

### Fixed
- Overlapping triangles and depth issue.

## [0.0.2] - 2022-12-25
### Added
- Add solo feature.
- Add multiple selection feature for tapes.
- Sphere rendering and general rendering improvements.

### Changed
- Update miniquad, egui-miniquad and egui versions to the latest.
- Preallocate 10 minutes worth of memory.
- Reduce SAMPLE_GRAPH_SIZE to 100, temporary fix for segmentation fault.
- Improve user interface.

### Fixed
- Fix metronome. It used to drag but now it keeps time well.
- Fix tape waveform graphs, they are scaled based on the maximum value.
- Fix vertex shaders, remove unnecessary arithmetic.

## [0.0.1] - 2022-09-14
### Added
- Add this CHANGELOG file to hopefully serve as an evolving example of a
standardized open source project CHANGELOG.
