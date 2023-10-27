import { MatrixClient } from "matrix-bot-sdk";
import { listenMode } from ".";
import { getMessage } from "./meals";

export async function listenBot(client: MatrixClient) {
  // Before we start the bot, register our command handler
  if (listenMode) client.on("room.message", handleCommand);

  // Now that everything is set up, start the bot. This will start the sync loop and run until killed.
  if (listenMode) client.start().then(() => console.log("Bot started!"));

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
}
