use reqwest;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::env;

pub struct SpoonClient {
  api_key: String,
}

impl SpoonClient {
  pub fn new() -> Self {
    SpoonClient {
      api_key: api_key().expect("Spoonacular API key not found. Make sure to set $SPOON_API_KEY"),
    }
  }

  pub async fn parse_ingredients<T: DeserializeOwned>(
    &self,
    request: ParseIngredientRequest,
  ) -> Result<Vec<T>, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
      .post("https://api.spoonacular.com/recipes/parseIngredients")
      .query(&[("apiKey", &self.api_key)])
      .form(&request)
      .send()
      .await?;
    response.json().await
  }
}

fn api_key() -> Result<String, env::VarError> {
  env::var("SPOON_API_KEY")
}

#[derive(Serialize)]
pub struct ParseIngredientRequest {
  #[serde(rename = "ingredientList")]
  ingredient_list: String,
}

impl ParseIngredientRequest {
  pub fn from_vec(raw_ingredients: Vec<String>) -> Self {
    let ingredient_list = raw_ingredients.join("\n");

    ParseIngredientRequest { ingredient_list }
  }
}
