import { MatrixClient, MatrixError } from "matrix-bot-sdk";
import { getMessage } from "./meals";

export async function sendMealMessage(client: MatrixClient, roomId: string) {
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
