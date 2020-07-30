use reqwest;
use scraper::{Html, Selector};
use serde::Deserialize;
use serde_json;
use std::error::Error;

#[derive(Deserialize)]
pub struct InstructionSchema {
  pub name: String,
  pub text: String,
}

#[derive(Deserialize)]
pub struct RecipeSchema {
  pub name: String,
  pub description: String,
  #[serde(rename = "recipeIngredient")]
  pub recipe_ingredients: Vec<String>,
  #[serde(rename = "recipeInstructions")]
  pub recipe_instructions: Vec<InstructionSchema>,
}

impl RecipeSchema {
  pub fn parse(url: &str) -> Result<Option<RecipeSchema>, Box<dyn Error>> {
    let html = reqwest::blocking::get(url)?.text()?;
    let html = Html::parse_document(&html);
    let selector = schema_selector();
    let schema_tags = html.select(&selector);

    let schema = schema_tags
      .map(|t| serde_json::from_str(&t.inner_html()))
      .find_map(Result::ok);

    Ok(schema)
  }
}

fn schema_selector() -> Selector {
  Selector::parse("script[type=\"application/ld+json\"]").expect("Failed to parse css selector")
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_can_parse_ingredients_from_webpage() {
    match RecipeSchema::parse("https://www.skinnytaste.com/caramelized-onion-red-pepper-and/") {
      Ok(Some(recipe)) => assert_eq!(
        recipe.name,
        "Caramelized Onion, Red Pepper and Zucchini Frittata"
      ),
      Ok(None) => assert!(false, "Failed to find a recipe"),
      Err(e) => assert!(false, e.to_string()),
    }
  }
}
