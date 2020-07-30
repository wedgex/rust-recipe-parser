use crate::spoon::{ParseIngredientRequest, SpoonClient};
use reqwest;
use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize, PartialEq, Debug)]
struct Ingredient {
  name: String,
  unit: String,
  aisle: String,
  amount: f32,
}

impl Ingredient {
  pub async fn parse(raw_ingredients: Vec<String>) -> Result<Vec<Self>, reqwest::Error> {
    let request = ParseIngredientRequest::from_vec(raw_ingredients);
    let client = SpoonClient::new();
    Ok(client.parse_ingredients(request).await?)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tokio;

  #[tokio::test]
  async fn test_parses_ingredient_list() {
    let ingredient_list = vec![
      "1 1/4 pounds of ground beef".to_string(),
      "3 jalapenos, diced".to_string(),
      "1 medium yellow onion".to_string(),
    ];

    let expected_ingredients = vec![
      Ingredient {
        name: "ground beef".to_string(),
        unit: "pounds".to_string(),
        aisle: "Meat".to_string(),
        amount: 1.25,
      },
      Ingredient {
        name: "jalapenos".to_string(),
        unit: "".to_string(),
        aisle: "Canned and Jarred;Produce;Ethnic Foods".to_string(),
        amount: 3.0,
      },
      Ingredient {
        name: "onion".to_string(),
        unit: "medium".to_string(),
        aisle: "Produce".to_string(),
        amount: 1.0,
      },
    ];

    let ingredients = Ingredient::parse(ingredient_list).await;

    match ingredients {
      Ok(ingredients) => assert_eq!(ingredients, expected_ingredients),
      Err(e) => assert_eq!("", e.to_string()),
    }
  }
}