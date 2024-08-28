![stravnik logo](https://codeberg.org/tomkoid/stravnik/media/branch/main/assets/stravnik.jpg)

# Stravnik (Matrix bot)

Get today's meal list from strava.cz and send notification of it on Matrix

## Installation and usage 

You need to have Rust installed installed on your system if you want to build the package yourself.

You need to have a Matrix account, a room to post to and a [strava.cz](https://strava.cz) canteen number.

```bash
git clone https://codeberg.org/tomkoid/stravnik # clone the repo
cd stravnik # go to the repo directory
cargo run # compile rust app 
```

Then you need to create an environment file (.env). You can use the example config file from `env.example`.
