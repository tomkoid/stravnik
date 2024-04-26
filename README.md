# Stravnik (Matrix bot)

Get today's meal list from strava.cz and post it on Matrix

## Installation and usage 

You need to have Rust installed installed on your system if you want to build the package yourself.

You need to have a Matrix account, a room to post to and a strava.cz canteen number.

```bash
git clone https://codeberg.org/tomkoid/stravnik # clone the repo
cd stravnik # go to the repo directory
cargo run # install node dependencies (or bun install if you use bun)
```

Then you need to create an environment file (.env). You can use the example config file as a template.

```bash
MATRIX_HOMESERVER=https://matrix.org
MATRIX_USERNAME=yourmatrixusername
MATRIX_PASSWORD=yourmatrixpassword
MATRIX_ROOM=!matrix_room:matrix_host
STRAVA_CANTEEN=0000
```
