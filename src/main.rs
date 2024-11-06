use pest::Parser;
use recipe_parser::*;

fn main() -> anyhow::Result< () > {
    let text = "1.5 cups";
    let got = Grammar::parse(Rule::amountAndUnit, text)?;
    println!("{:#?}", got);
    Ok(())
}
