use pest::Parser;
use recipe_parser::*;
use pest::iterators::Pairs;

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_measurements() -> anyhow::Result<()> {
        let mut pair:Pairs<Rule> = Grammar::parse(Rule::amountAndUnit, "1.5 cups")?;

        assert_eq!(pair.as_str(), "1.5 cups");
        assert_eq!(pair.next().unwrap().into_inner().peek().unwrap().as_str(), "1.5");

        Ok(())
    }

    // #[test]
    // fn test_parser_record() -> anyhow::Result<()> {
    //     let rule = Rule::field;
    //     let toParse = "-2345.54,-15";
    //     let pair = Grammar::parse(rule, toParse)?.next().ok_or_else( || anyhow!("no needed pair"))?;
    //     let parsed_result = Grammar::parse(rule, "*");
    //
    //     assert!(parsed_result.is_err());
    //
    //     assert_eq!(pair.as_str(), "-2345.54");
    //     assert_eq!(pair.as_span().start(), 0);
    //
    //     Ok(())
    // }
}