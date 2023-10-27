import { getPublicMenu } from "strava-cz-sdk";
import { stravaDate } from ".";

export async function getMessage(): Promise<string> {
  const json: any = await getPublicMenu(process.env.CANTEEN_NUMBER!);

  let meals: any[] = [];
  for (const item in json) {
    if (json[item].length === 0) continue;

    for (const meal in json[item]) {
      if (json[item][meal].length === 0) continue;

      if (json[item][meal]["datum"] == stravaDate) {
        meals.push(json[item][meal]);
      }
    }
  }

  console.log(meals);

  const messageMeals = meals.map((meal) => {
    return `${meal["id"] + 1}. ${meal["nazev"]}`
  })

  const mealsMessage = meals.length !== 0 ? messageMeals.join('\n') : "Žádné obědy nenalezeny!";

  const introMessage = `Obědy pro ${stravaDate}:`
  const message = [introMessage, `${"#".repeat(introMessage.length)}`, mealsMessage].join('\n');

  return message
}
