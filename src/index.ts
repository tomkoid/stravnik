import { getPublicMenu } from 'strava-cz-sdk';
import dotenv from 'dotenv';

// get all the env variables from .env
dotenv.config();

const canteenInfo = getPublicMenu(process.env.CANTEEN_NUMBER!).then((info) => {
  console.log(info);
}); // returns canteen info object
