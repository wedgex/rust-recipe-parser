use reqwest;
use scraper::{Html, Selector};
use serde::Deserialize;
use serde_json;
use std::error::Error;

#[derive(Deserialize)]
struct AuthorSchema {
  pub name: String,
}

#[derive(Deserialize)]
struct InstructionSchema {
  pub name: String,
  pub text: String,
}

#[derive(Deserialize)]
struct RecipeSchema {
  pub name: String,
  pub description: String,
  pub image: Vec<String>,
  pub author: AuthorSchema,
  #[serde(rename = "datePublished")]
  pub date_published: String,
  #[serde(rename = "recipeIngredient")]
  pub recipe_ingredient: Vec<String>,
  #[serde(rename = "recipeInstructions")]
  pub recipe_instructions: Vec<InstructionSchema>,
}

impl RecipeSchema {
  pub async fn parse(url: &str) -> Result<Option<RecipeSchema>, Box<dyn Error>> {
    let html = reqwest::get(url).await?.text().await?;
    let html = Html::parse_document(&html);
    let selector =
      Selector::parse("script[type=\"application/ld+json\"]").expect("Failed to parse css");
    let tags = html.select(&selector);

    let schema = tags.map(|t| parse_schema(&t)).find_map(Result::ok);

    Ok(schema)
  }
}

fn parse_schema(tag: &scraper::ElementRef) -> Result<RecipeSchema, serde_json::Error> {
  serde_json::from_str(&tag.inner_html())
}

#[cfg(test)]
mod tests {
  use super::*;
  use tokio;

  #[tokio::test]
  async fn test_can_parse_ingredients_from_webpage() {
    match RecipeSchema::parse("https://www.skinnytaste.com/caramelized-onion-red-pepper-and/").await
    {
      Ok(Some(recipe)) => assert_eq!(
        recipe.name,
        "Caramelized Onion, Red Pepper and Zucchini Frittata"
      ),
      Ok(None) => assert!(false, "Failed to find a recipe"),
      Err(e) => assert!(false, e.to_string()),
    }
  }
}
