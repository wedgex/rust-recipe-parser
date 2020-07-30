mod ingredient;
mod schema;

use ingredient::Ingredient;
use schema::RecipeSchema;
use serde::Serialize;
use std::error::Error;

#[derive(Serialize)]
pub struct Recipe {
  pub name: String,
  pub description: String,
  pub ingredients: Vec<Ingredient>,
  pub steps: Vec<String>,
}

impl Recipe {
  pub fn from(url: String) -> Result<Option<Self>, Box<dyn Error>> {
    if let Some(schema) = RecipeSchema::parse(&url)? {
      let ingredients = Ingredient::parse(schema.recipe_ingredients)?;
      return Ok(Some(Recipe {
        name: schema.name,
        description: schema.description,
        ingredients,
        steps: schema
          .recipe_instructions
          .iter()
          .map(|s| s.text.clone())
          .collect(),
      }));
    }

    Ok(None)
  }
}
