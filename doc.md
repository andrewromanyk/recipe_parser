# Recipe parser

In this documentation I will explain the process of parsing a recipe.

## Grammar rules

Many of the rules under _Basic_ and recipe-ish rules are self-explanatory.

## myAny

Corresponds to any cyryllic or latin letter

```pest
    myANY = { CYRILLIC | ASCII_ALPHA }
```

## spacedDelimiter

Corresponds to a delimiter surrounded by 0 to 3 spaces

```pest
    spacedDelimiter = {minimumSpaces ~ delimiter ~ minimumSpaces}
```

## fullText

Corresponds to the text starting and ending in any myAny letter with spaces in between

```pest
fullText = {myANY ~ (space* ~ myANY)*}
```

## anyNumber

Corresponds to any positive integer or real number

```pest
anyNumber = {ASCII_DIGIT+ ~ (("." | ",") ~ ASCII_DIGIT+)?}
```

## recipeTitleFull

Corresponds to the name of the dish with the prefix.
<br>
_E.g. "Dish:test" "Страва: тест"_

```pest
recipeTitleFull = {anySpaces ~ recipeTitle ~ textAfterSpace}
```

#### Other recipe-ish rules show the same behaviour 

## amountAndUnit

Corresponds to a pair of a number and its unit.
<br>
_E.g. "1.5 cups" "3 l" "400 g"_

```pest
amountAndUnit = _{anyNumber ~ space ~ fullText}
```

## ingredientRow

Corresponds to a line with the ingredients name and its pair of measurement and unit.
<br>
_E.g. "Apple - 2 pcs" "Water -  3l" "Sugar-2 tsp"_

```pest
ingredientRow = {fullText ~ spacedDelimiter ~ amountAndUnit}
```

## recipeFull

Corresponds to a full recipe, a combination of many previous rules.

```pest
recipeFull = {recipeTitleFull ~  recipeDescriptionFull ~ recipeIngredientsFull ~ recipeInstructionsFull}
```

## filename

Corresponds to a file ending with .txt

```pest
filename = { anyPlusSpecial+ ~ ".txt" }
```

## PARSE

Corresponds to parsing command with the filename 

```pest
PARSE = {"PARSE " ~ filename }
```

#### Other commands are one-word:
- HELP
- MENU
- LEAVE
- CREDITS
