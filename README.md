# Stravnik (Matrix bot)

Get today's meal list from strava.cz and post it on Matrix

## Installation and usage 

You need to have Node.js or Bun installed on your system and also you need to have a Matrix account and a room to post to.

```bash
git clone https://codeberg.org/Tomkoid/stravnik # clone the repo
cd stravnik # go to the repo directory
npm install # install node dependencies (or bun install if you use bun)
```

Then you need to create an environment file (.env). You can use the example config file as a template.

```bash
HOMESERVER=https://matrix.org
ACCESS_TOKEN=YOUR_ACCESS_TOKEN
ROOM_ID=YOUR_ROOM_ID
CANTEEN_NUMBER=0000
```

Then you can compile the TypeScript code and run the bot with `node src/index.js` or just run `bun run src/index.ts` if you use Bun.
