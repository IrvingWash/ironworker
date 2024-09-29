![Crates.io Version](https://img.shields.io/crates/v/ironworker)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/IrvingWash/ironworker/rust.yml?branch=master&label=ci)

## ironworker

a simple cli tool to manually scrobble tracks and albums to lastfm

### installation
```bash
cargo install ironworker
```

### usage
authenticate:
```bash
ironworker auth
```

list recent tracks:
```bash
ironworker list
```

scrobble a track:
```bash
ironworker scrobble-track Krallice "Ygg Huur" "Over Spirit"
```

scrobble an album:
```bash
ironworker scrobble-album Orgone "The Goliath"
```

for more:
```bash
ironworker --help
```
