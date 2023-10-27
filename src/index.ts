import { getPublicMenu } from 'strava-cz-sdk';
import dotenv from 'dotenv';

// get all the env variables from .env
dotenv.config();

// getPublicMenu(process.env.CANTEEN_NUMBER!).then((info) => {
//   console.log(info);
// }); // returns canteen info object
//

import {
  MatrixClient,
  SimpleFsStorageProvider,
  AutojoinRoomsMixin,
  MatrixError,
} from "matrix-bot-sdk";

import { getMessage } from './meals';

if (!process.env.HOMESERVER) throw new Error("No homeserver specified in .env! (HOMESERVER)");
if (!process.env.ACCESS_TOKEN) throw new Error("No access token specified in .env! (ACCESS_TOKEN)");
if (!process.env.LISTEN_MODE && !process.env.ROOM_ID) throw new Error("No matrix ROOM_ID specified in .env!");
if (process.env.LISTEN_MODE && process.env.ROOM_ID) throw new Error("Both LISTEN_MODE and ROOM_ID specified in .env!");

const homeserverUrl: string = process.env.HOMESERVER;
const accessToken: string = process.env.ACCESS_TOKEN;
const listenMode: boolean = process.env.LISTEN_MODE == "true";
const roomId: string = process.env.ROOM_ID!;

const storage = new SimpleFsStorageProvider("stravnik-bot.json");
const client = new MatrixClient(homeserverUrl, accessToken, storage);

AutojoinRoomsMixin.setupOnClient(client);

// Before we start the bot, register our command handler
if (listenMode) client.on("room.message", handleCommand);

// Now that everything is set up, start the bot. This will start the sync loop and run until killed.
if (listenMode) client.start().then(() => console.log("Bot started!"));

if (!listenMode) {
  // start the bot
  client.start().then(() => {
    console.log("bot started!")

    console.log("sending message...")
    getMessage().then(async (message) => {
      await client.sendText(roomId, message).catch((err: MatrixError) => {
        if (err.message.includes("M_FORBIDDEN")) {
          console.log("Bot is not allowed to send messages to this room!");
        } else {
          console.log(err.message);
        }
      });

      process.exit(0)
    });
  });
}

// This is the command handler we registered a few lines up
async function handleCommand(roomId: string, event: any) {
  // Don't handle unhelpful events (ones that aren't text messages, are redacted, or sent by us)
  if (event['content']?.['msgtype'] !== 'm.text') return;
  if (event['sender'] === await client.getUserId()) return;

  // Check to ensure that the `!hello` command is being run
  const body = event['content']['body'];
  if (!body?.startsWith("!meals") && !body?.startsWith("!obedy")) return;

  const message = await getMessage();

  // Now that we've passed all the checks, we can actually act upon the command
  console.log(message)

  await client.sendText(roomId, message);
}
