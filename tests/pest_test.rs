use pest::iterators::Pairs;
use pest::Parser;
use recipe_parser::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn my_any_test() -> anyhow::Result<()> {
        let text = "a";
        let pair: Pairs<Rule> = Grammar::parse(Rule::myANY, text)?;

        assert_eq!(pair.as_str(), text);

        Ok(())
    }

    #[test]
    fn full_text_test() -> anyhow::Result<()> {
        let text = "рор ол ро jh hhghj ahjgGjhg hj";
        let pair: Pairs<Rule> = Grammar::parse(Rule::fullText, text)?;

        assert_eq!(pair.as_str(), text);

        Ok(())
    }

    #[test]
    fn any_number_test() -> anyhow::Result<()> {
        let text = "12.23";
        let pair: Pairs<Rule> = Grammar::parse(Rule::anyNumber, text)?;

        assert_eq!(pair.as_str().parse::<f32>()?, 12.23);

        Ok(())
    }

    #[test]
    fn measurements_test() -> anyhow::Result<()> {
        let mut pair: Pairs<Rule> = Grammar::parse(Rule::amountAndUnit, "1.5 cups")?;

        println!("{:?}", pair);

        assert_eq!(pair.as_str(), "1.5 cups");
        assert_eq!(pair.next().unwrap().as_str(), "1.5");

        Ok(())
    }

    #[test]
    fn recipe_title_full_test() -> anyhow::Result<()> {
        let text = "  \t Dish: тест";
        let pair: Pairs<Rule> = Grammar::parse(Rule::recipeTitleFull, text)?;

        assert_eq!(pair.as_str(), text);

        Ok(())
    }

    #[test]
    fn recipe_description_full_test() -> anyhow::Result<()> {
        let text = "  Опис:Опистест";
        let pair: Pairs<Rule> = Grammar::parse(Rule::recipeDescriptionFull, text)?;

        assert_eq!(pair.as_str(), text);

        Ok(())
    }

    #[test]
    fn ingredient_row_test() -> anyhow::Result<()> {
        let text = "тест - 1 шт";
        let pair: Pairs<Rule> = Grammar::parse(Rule::ingredientRow, text)?;

        assert_eq!(pair.as_str(), text);

        Ok(())
    }

    #[test]
    fn recipe_ingredients_full_test() -> anyhow::Result<()> {
        let text = "Інгредієнти: тест - 1 шт \tінший тест - 1 шт";
        let pair: Pairs<Rule> = Grammar::parse(Rule::recipeIngredientsFull, text)?;

        assert_eq!(pair.as_str(), text);

        Ok(())
    }

    #[test]
    fn recipe_instructions_full_test() -> anyhow::Result<()> {
        let text = "\nРецепт: - зробити то - зробити сьо";
        let pair: Pairs<Rule> = Grammar::parse(Rule::recipeInstructionsFull, text)?;

        assert_eq!(pair.as_str(), text);

        Ok(())
    }

    #[test]
    fn file_name_test() -> anyhow::Result<()> {
        let text = "file.txt";
        let pair: Pairs<Rule> = Grammar::parse(Rule::filename, text)?;

        assert_eq!(pair.as_str(), text);

        Ok(())
    }

    #[test]
    fn command_test() -> anyhow::Result<()> {
        let text = "PARSE thisfile.txt";
        let pair: Pairs<Rule> = Grammar::parse(Rule::listOfCommands, text)?;

        assert_eq!(pair.as_str(), text);

        Ok(())
    }
}
