# recipe_parser

A parser for food recipes. Divides recipes into: name, description, ingredients and instructions.

## Parts of the recipe:
- Dish name 
<br> 
_Starts with the keyword followed by the name_
- Dish description 
<br>
_Starts with the keyword followed by the description_
- Needed ingredients
<br>
_A list of ingredients in the "_ingredient_ - *amount* _unit_" form_
- Steps how to make the dish
<br>
_A list of steps in the <br> 
\- step1
<br>\- step2
<br>\- step3
<br> form_

## Process
A recipe is parsed by being divided into 4 aforementioned parts, each having its own form.
Many rules are "hidden" for easier parsing, e.g. spaces, separate letters, delimiters, etc.

## Recipe example
```
Страва: Бутерброд з ковбасою

Опис: простий бутерброд

Інгредієнти:
Ковбаса - 1 шт
Сир - 1 уп
Хліб - 1 скибка

Кроки:
- Нарізати ковбасу і покласти на хліб
- Покласти сир на ковабсу
```
### Corresponds to

```
reicpeTitle = "Бутерброд з ковбасою"
recipeDescription = "простий бутерброд"
recipeIngredientsFull = [
    ("Ковбаса", (1, "шт")),
    ("Сир", (1, "уп")),
    ("Хліб", (1, "скибка"))
]
recipeInstructionsFull = [
    "Нарізати ковбасу і покласти на хліб",
    "Покласти сир на ковабсу"
]
```