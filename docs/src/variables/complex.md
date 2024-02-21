# Complex Variables

Beyond simple values, variables can be used to define entire mappings or sequences, and even matchers. Variables can reference other variables, and can be used in place of entire `request` mappings or response `body` mappings. Additionally, some matchers that accept embedded mappings or sequences (such as [$includes](../matchers/includes.md)), it can be more ergonomic to predefine these mappings as variables.

## Nesting Variables

You can reference, or nest, variables within other variables. Nested variables are recursively resolved and expanded.

```yaml
suite: "Create Recipe"
description: "This suite involves creating a new recipe and fetching its information."
variables:
  BASE_URL: http://localhost:3000
  RECIPE_URL: ${BASE_URL}/recipes
```

## Mappings or Sequences

You can define entire mappings (objects) or sequences (arrays) as variables and use them in your tests. In the example below, and entire `RECIPE` object is defined as a variable and then used as the request body for creating the recipe, and later used as an `expect` definition when verifying the correct recipe is returned.

```yaml
suite: 'Create Recipe'
description: 'This suite involves creating a new recipe and fetching its information.'
variables:
  RECIPE_URL: http://localhost:3000/recipes
  USER_EMAIL: recipe1@tests.com
  USER_PASSWORD: hG98s4%%phG
  RECIPE:
    name: Guacamole
    description: >
      A delicious classic guacamole recipe.
    time: 10
    servings: 6
    ingredients:
      - 3 avocados
      - 1/2 red onion
      - 1 lime
      - 1 green bell pepper
      - 1 jalapeno pepper
      - 1/2 tsp cumin
      - 1/2 tsp salt
      - 1/2 tsp red pepper
    instructions:
      [
        'Roughly chop onion, green pepper, and jalapeno and add to food processor. Pulse 2-3 times.',
        'Cut and remove seeds from avocados, use spoon to scoop and mash in a bowl.',
        'Mix in the vegetables, seasonings, and lime juice squeezed from a fresh lime.',
      ]

tests:
  - test: Create new recipe
    description: Create a new recipe
    request:
      method: POST
      url: ${RECIPE_URL}
      headers:
        Content-Type: application/json
      body: ${RECIPE}
    expect:
      status: 2xx
      body: ${RECIPE}
    extract:
      body:
        id: ${RECIPE_ID}

  - test: Get recipe
    description: 'Should be able to get recipe information'
    request:
      method: GET
      url: ${RECIPE_URL}/${RECIPE_ID}
    expect:
      status: 2xx
      body: ${RECIPE}
```

## Embedded Matcher Arguments

When using a matcher like [$includes](../matchers/includes.md) that expects any value, mapping, or sequence as an argument - defining something complex like an object requires using JSON strings (because of the limitations of YAML nesting in strings). Instead of defining a complex JSON string and wearing out your `"` key, you can define the object you expect to find in the array as a variable.

For this example, reference the previous example's use of the `RECIPE` variable.

```yaml
  - test: Recipe in list
    description: Recipe should be visible when listing all recipes
    request:
      method: GET
      url: ${RECIPE_URL}/all
    expect:
      status: 2xx
      body: 
        recipes: $includes ${RECIPE}
```

At runtime, the `${RECIPE}` will be expanded to the full mapping as defined above, and will then proceed to be correctly parsed as an object to be matched.