# Recipe parser

A simple parser for cooking recipes.

## Parts of the recipe:
- Dish name
- Dish description
- Needed ingredients
- Steps how to make the dish

## Parsing:
- **anySpace** corresponds to any amount of spaces, newlines and tabs
- **title** and **text** -esque rules correspond to simple arrays of character: simple ascii and any respectively
- **anyNumber** corresponds to a positive digit or real number that can be delimited with either a comma or a dot
- **Title**, **Description**, **Ingredients** and **Instructions** use combinations of previously created rules to 
create respective type: a simple text with spaces, a long text with any characters, a list of ingredients with their 
amounts and measuring units, a list of instruction on how to cook a dish
- **recipeFull** combines the latter to fully create a recipe