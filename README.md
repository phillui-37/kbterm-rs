# KBTerm
### Introduction
K8s manager with tui. Constructed with Rust.

Will not implement full features support of k8s. Please fork this repo or use other tools if you need features not provided by this.

### Features
- [ ] namespace management
- [ ] pods management
- [ ] resources monitoring
- [ ] ssh connection
- [ ] livetime log
- [ ] log extraction

### Dependency
- [Cursive](https://github.com/gyscos/Cursive) - tui framework
  - using `pancurses` feature, which introduced `ncurses`(*nix)/`pdcurses`(Windows) link dependency
- [russh](https://github.com/warp-tech/russh) - ssh connectivity
- [kube](https://github.com/kube-rs/kube) - kubernetes lib
  - [k8s-openapi](https://github.com/Arnavion/k8s-openapi)

### License
Apache-2.0