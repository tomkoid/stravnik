<img src="https://codeberg.org/tomkoid/stravnik/media/branch/main/assets/stravnik.jpg" alt="stravnik logo" width="200"/>

# Strávník

Get today's meal list from Strava.cz or any iCanteen URL and send notification of it on Matrix or other supported notification services.

## Installation and usage

You need to have Rust installed installed on your system if you want to build the package yourself.

<strike>
Then you need to create an environment file (.env). You can use the example config file from `.env.example`. Also make sure you fill in your credentials there for your selected services.
If you try to use any service that doesn't have its credentials filled in in `.env`, you will get an error telling you what env variable you need to fill in.
Why or how to fill in the credentials is not covered here, just use your brain lol.
</strike>

In the newest version you pass in all credentials and options as command line arguments.

To run the script, run the following command:

```bash
git clone https://codeberg.org/tomkoid/stravnik # clone the repo
cd stravnik # go to the repo directory
cargo run # run the app
```

Scheduling the script to run at a specific time can be done with `crontab` or any other method you prefer, it is fully up to you.

## Supported notification services

- [Matrix](https://matrix.org/)
- [Ntfy](https://ntfy.sh/)
- [Discord Webhook](https://discord.com/)
