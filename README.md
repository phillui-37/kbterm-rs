# KBTerm
### Introduction
K8s manager with tui. Constructed with Rust

### Features
- [ ] namespace management
- [ ] pods management
- [ ] resources monitoring
- [ ] ssh connection
- [ ] livetime log
- [ ] log extration

### Dependency
- [Cursive](https://github.com/gyscos/Cursive) - tui framework
  - using `pancurses` feature, which introduced `ncurses`(*nix)/`pdcurses`(Windows) link dependency
- [russh](https://github.com/warp-tech/russh) - ssh connectivity

### License
Apache-2.0