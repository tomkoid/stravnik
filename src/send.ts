import { MatrixClient, MatrixError } from "matrix-bot-sdk";
import { getMessage } from "./meals";
import { stravaDate } from ".";

export async function sendMealMessage(client: MatrixClient, roomId: string) {
  // start the bot
  client.start().then(() => {
    console.log("bot started!")

    console.log("sending message...")
    getMessage().then(async (message) => {
      if (message == "empty") {
        console.log("no meals found for the date of " + stravaDate)
        process.exit(0)
      }

      await client.sendHtmlText(roomId, message).catch((err: MatrixError) => {
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
