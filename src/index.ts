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
} from "matrix-bot-sdk";

if (!process.env.HOMESERVER) throw new Error("No homeserver specified!");
if (!process.env.ACCESS_TOKEN) throw new Error("No access token specified!");

const homeserverUrl: string = process.env.HOMESERVER;
const accessToken: string = process.env.ACCESS_TOKEN;
const listenMode: boolean = process.env.LISTEN_MODE == "true";

// In order to make sure the bot doesn't lose its state between restarts, we'll give it a place to cache
// any information it needs to. You can implement your own storage provider if you like, but a JSON file
// will work fine for this example.
const storage = new SimpleFsStorageProvider("stravnik-bot.json");

// Finally, let's create the client and set it to autojoin rooms. Autojoining is typical of bots to ensure
// they can be easily added to any room.
const client = new MatrixClient(homeserverUrl, accessToken, storage);
AutojoinRoomsMixin.setupOnClient(client);

// Before we start the bot, register our command handler
if (listenMode) client.on("room.message", handleCommand);

// Now that everything is set up, start the bot. This will start the sync loop and run until killed.
client.start().then(() => console.log("Bot started!"));

// This is the command handler we registered a few lines up
async function handleCommand(roomId: string, event: any) {
  // Don't handle unhelpful events (ones that aren't text messages, are redacted, or sent by us)
  if (event['content']?.['msgtype'] !== 'm.text') return;
  if (event['sender'] === await client.getUserId()) return;

  // Check to ensure that the `!hello` command is being run
  const body = event['content']['body'];
  if (!body?.startsWith("!meals") && !body?.startsWith("!obedy")) return;

  // Now that we've passed all the checks, we can actually act upon the command
  const json: any = await getPublicMenu(process.env.CANTEEN_NUMBER!);
  // console.log(json)

  let meals: any[] = [];
  for (const item in json) {
    if (json[item].length === 0) continue;

    for (const meal in json[item]) {
      if (json[item][meal].length === 0) continue;

      if (json[item][meal]["datum"] == "30.11.2023") {
        meals.push(json[item][meal]);
      }
    }
  }

  console.log(meals);

  const messageMeals = meals.map((meal) => {
    return `${meal["id"] + 1}. ${meal["nazev"]}`
  })

  const introMessage = `Obědy pro ${meals[0]["datum"]}:`
  const message = [introMessage, `${"#".repeat(introMessage.length)}`, messageMeals.join('\n')].join('\n');

  console.log(message)

  await client.sendText(roomId, message);
}
