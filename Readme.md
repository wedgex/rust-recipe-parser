# Recipe Parser

## Getting Started

Set the `SPOON_API_KEY` env variable to a [Spoonacular API key](https://spoonacular.com/food-api/).

```shell
export SPOON_API_KEY=secretkey
```

Start the server

```shell
cargo run
```

Make a request

```shell
curl -i -H "Content-Type: application/json" -X POST -d '{"url": "https://www.skinnytaste.com/jalapeno-popper-nachos/" }' http://localhost:8000/parse
```

Check out your response

```json
{
  "name": "Jalapeno Popper \"Nachos\"",
  "description": "Jalapeno Poppers meet nachos in this fun, low-carb twist of two classic appetizers, perfect for sharing!",
  "ingredients": [
    { "name": "reynolds wrap non foil", "unit": "Stick", "aisle": "Bakery/Bread", "amount": 1.0 },
    {
      "name": "olive oil spray",
      "unit": "servings",
      "aisle": "Oil, Vinegar, Salad Dressing",
      "amount": 2.0
    },
    { "name": "ground turkey", "unit": "pound", "aisle": "Meat", "amount": 0.5 },
    { "name": "garlic", "unit": "clove", "aisle": "Produce", "amount": 1.0 },
    { "name": "onion", "unit": "tablespoons", "aisle": "Produce", "amount": 2.0 },
    { "name": "cilantro", "unit": "tbsp", "aisle": "Produce;Spices and Seasonings", "amount": 1.0 },
    {
      "name": "garlic powder",
      "unit": "teaspoon",
      "aisle": "Spices and Seasonings",
      "amount": 0.5
    },
    { "name": "cumin powder", "unit": "teaspoon", "aisle": "Spices and Seasonings", "amount": 0.5 },
    { "name": "kosher salt", "unit": "teaspoon", "aisle": "Spices and Seasonings", "amount": 0.5 },
    { "name": "tomato paste", "unit": "tablespoon", "aisle": "Pasta and Rice", "amount": 0.5 },
    { "name": "water", "unit": "tablespoons", "aisle": "Beverages", "amount": 2.0 },
    {
      "name": "jalapeno peppers )",
      "unit": "",
      "aisle": "Canned and Jarred;Produce;Ethnic Foods",
      "amount": 8.0
    },
    { "name": "scallions", "unit": "large", "aisle": "Produce", "amount": 1.0 },
    { "name": "sharp cheddar cheese", "unit": "ounce", "aisle": "Cheese", "amount": 0.5 },
    { "name": "sharp cheddar cheese", "unit": "cup", "aisle": "Cheese", "amount": 0.5 },
    {
      "name": "scallions and cilantro",
      "unit": "servings",
      "aisle": "Produce;Spices and Seasonings",
      "amount": 2.0
    },
    {
      "name": "cream plus 2 teaspoons water",
      "unit": "tablespoons",
      "aisle": "Milk, Eggs, Other Dairy",
      "amount": 2.0
    },
    {
      "name": "pico de gallo",
      "unit": "cup",
      "aisle": "Canned and Jarred;Ethnic Foods",
      "amount": 0.5
    },
    { "name": "olives", "unit": "tablespoons", "aisle": "Canned and Jarred", "amount": 2.0 },
    { "name": "", "unit": null, "aisle": null, "amount": 3.0 }
  ],
  "steps": [
    "Preheat oven to 400F and line a large baking sheet with nonstick aluminum foil.",
    "Heat a medium nonstick skillet over medium heat and spray with oil. Add onion, cilantro and garlic and saute about 2 minutes, until soft. Add ground turkey, salt, garlic powder, cumin and cook meat for 4 to 5 minutes until meat is completely cooked through breaking it up with a spoon. Add the tomato paste and water, mix well and simmer on medium for about 2 to 3 minutes, remove from heat.",
    "Meanwhile, combine cream cheese, cheddar and scallions in a medium bowl. Using a small spoon or a spatula, spoon about 1 teaspoon of the cream cheese filling into the peppers.",
    "Arrange in a single layer, cut-side up close together. Bake until soft, about 12 to 15 minutes.",
    "Top with meat and cheese and bake until melted, about 3 minutes more.",
    "Remove from oven and top with pico de gallo, olives and drizzle with sour cream. Garnish with cilantro and scallions and serve immediately."
  ]
}
```
