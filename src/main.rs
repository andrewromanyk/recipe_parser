use pest::Parser;
use recipe_parser::*;

fn main() -> anyhow::Result< () > {
    let text = "Страва: Тестова страва
Опис: тестовий опис
Інгредієнти:
 огірок - 2 шт
 вода - 0.5 л
Рецепт:
- огірок порізати
- воду налити";
    let got = Grammar::parse(Rule::recipeFull, text)?;
    println!("{:#?}", got);
    Ok(())
}
