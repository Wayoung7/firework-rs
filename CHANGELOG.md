# Changelog

All notable changes to this project will be documented in this file.

## [0.3.1](https://github.com/Wayoung7/firework-rs/releases/tag/v0.3.1) - 2024-04-30

### Changed
 - Performance enhanced by using `VecDeque` to represent trail and by using macro initialization of `Vec`

## [0.3.0](https://github.com/Wayoung7/firework-rs/releases/tag/v0.3.0) - 2024-04-12

### Added
 - CJK(also UTF-8) characters support (it takes twice of space as normal ascii characters)


## [0.2.0](https://github.com/Wayoung7/firework-rs/releases/tag/v0.2.0) - 2024-03-22

### Added
 - Firework demo which generates fireworks infinitely and randomly
 - Command line argument of changing frame rate
 - New field in `FireworkManager` to control the installation of `Firework`s

### Changed
 - Implementation of filtering dead `Particle`s