#![doc = include_str!("../doc.md")]

use ::pest_derive::Parser;
use anyhow::Result;
use pest::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

#[derive(Debug, Error)]
pub enum RecipeError {
    #[error("Couldn't pass recipe: {0}")]
    Error(String),
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Recipe {
    title: String,
    description: String,
    ingredients: Vec<(String, (f32, String))>, // (ingredient, (amount, measurement))
    steps: Vec<String>,
}
#[derive(Debug)]
pub enum Commands {
    HELP,
    MENU,
    LEAVE,
    PARSE,
    CREDITS,
    ERROR,
}

#[derive(Debug)]
pub struct Command {
    pub command: Commands,
    pub input: Option<String>,
}

impl Recipe {
    pub fn new(recipe_string: &str) -> Result<Self, RecipeError> {
        let mut rule_list = Grammar::parse(Rule::recipeFull, recipe_string)
            .map_err(|e| RecipeError::Error(e.to_string()))?
            .next()
            .unwrap()
            .into_inner();

        let title = rule_list
            .next()
            .unwrap()
            .into_inner()
            .next()
            .unwrap()
            .as_str()
            .to_string();
        let description = rule_list
            .next()
            .unwrap()
            .into_inner()
            .next()
            .unwrap()
            .as_str()
            .to_string();

        let mut ingredients = Vec::new();
        for ingredient_row in rule_list.next().unwrap().into_inner() {
            let mut row_inner_list = ingredient_row.into_inner();
            let ingredient_name = row_inner_list.next().unwrap().as_str();
            let ingredient_amount: f32 = row_inner_list
                .next()
                .unwrap()
                .as_str()
                .replace(",", ".")
                .parse()
                .map_err(|_| RecipeError::Error("Couldn't map ingredients".to_string()))?;
            let ingredient_measure = row_inner_list.next().unwrap().as_str();
            ingredients.push((
                ingredient_name.to_string(),
                (ingredient_amount, ingredient_measure.to_string()),
            ));
        }

        let mut steps = Vec::new();
        for step in rule_list.next().unwrap().into_inner() {
            let step_name = step.as_str();
            steps.push(step_name.to_string());
        }

        Ok(Recipe {
            title,
            description,
            ingredients,
            steps,
        })
    }
}

impl Command {
    pub fn new(input_command: &str) -> Self {
        let parsed = Grammar::parse(Rule::listOfCommands, input_command);
        if parsed.is_err() {
            return Command {
                command: Commands::ERROR,
                input: None,
            };
        }

        let rule_list = parsed.unwrap().next().unwrap().into_inner();
        let mut command: Commands = Commands::ERROR;
        let mut input = None;

        for sub in rule_list {
            match sub.as_rule() {
                Rule::HELP => command = Commands::HELP,
                Rule::MENU => command = Commands::MENU,
                Rule::LEAVE => command = Commands::LEAVE,
                Rule::CREDITS => command = Commands::CREDITS,
                Rule::PARSE => {
                    command = Commands::PARSE;
                    let mut inner = sub.into_inner();
                    let next = inner.next();
                    if next.is_some() {
                        input = Some(next.unwrap().as_str().to_string());
                    }
                }
                _ => command = Commands::ERROR,
            }
        }

        Command { command, input }
    }
}
