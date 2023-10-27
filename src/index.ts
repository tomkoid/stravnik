import {
  MatrixClient,
  SimpleFsStorageProvider,
  AutojoinRoomsMixin,
  MatrixError,
} from "matrix-bot-sdk";

import { getMessage } from './meals';
import dotenv from 'dotenv';
import { listenBot } from "./listen";
import { sendMealMessage } from "./send";

// get all the env variables from .env
dotenv.config();

if (!process.env.HOMESERVER) throw new Error("No homeserver specified in .env! (HOMESERVER)");
if (!process.env.ACCESS_TOKEN) throw new Error("No access token specified in .env! (ACCESS_TOKEN)");
if (!process.env.LISTEN_MODE && !process.env.ROOM_ID) throw new Error("No matrix ROOM_ID specified in .env!");
if (process.env.LISTEN_MODE && process.env.ROOM_ID) throw new Error("Both LISTEN_MODE and ROOM_ID specified in .env!");

const homeserverUrl: string = process.env.HOMESERVER;
const accessToken: string = process.env.ACCESS_TOKEN;
export const listenMode: boolean = process.env.LISTEN_MODE == "true";
export const roomId: string = process.env.ROOM_ID!;

export const currentDate = new Date();

const storage = new SimpleFsStorageProvider("stravnik-bot.json");
const client = new MatrixClient(homeserverUrl, accessToken, storage);

export const stravaDate = currentDate.toLocaleDateString("cs-CZ", {
  day: "2-digit",
  month: "2-digit",
  year: "numeric",
}).replaceAll(" ", "");

AutojoinRoomsMixin.setupOnClient(client);

if (!listenMode) sendMealMessage(client, roomId);
if (listenMode) listenBot(client);
