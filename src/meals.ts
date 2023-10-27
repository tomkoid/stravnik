import { getPublicMenu } from "strava-cz-sdk";

export async function getMessage(): Promise<string> {
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

  return message
}
