# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2023-10-18

### Added

- Added coefficient D to make today work time affect resting time
- Added ability to change notification thresholds: 25 and 52 minutes, and phases' names

### Changed

- Critical notifications are not used anymore
- Changed the app's CLI interface

## [0.2.1] - 2023-08-12

### Added

- Using work-break icon (if exists in system, Linux/BSD)

## [0.2.0] - 2023-07-23

### Added

- Cross-platform support using Desktop notifications
- Counting the total work time of the day
- Notifying on configured daily work time limit
- Ability to change the formula's coefficients

### Changed

- Desktop notifications instead of beeping

### Removed

- Xfce plugin support