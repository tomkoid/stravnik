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
    const searchParams = new URLSearchParams({
      "q": meal["nazev"].normalize("NFD").replace(/[\u0300-\u036f]/g, ""),
      "tbm": "isch" // image search
    })

    return `<p>${meal["id"] + 1}. ${meal["nazev"]} - <a href="https://www.google.com/search?${searchParams}">obrázek</a></p>`
  })

  if (meals.length == 0) {
    return "empty"
  }

  const mealsMessage = meals.length !== 0 ? messageMeals.join('\n') : "Žádné obědy nenalezeny!";

  const introMessage = `Obědy pro ${stravaDate}:`
  const message = [introMessage, `<p>${"#".repeat(introMessage.length)}</p>`, mealsMessage].join('\n');

  return message
}
