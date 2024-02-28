# $and

The `$and` matcher is a logical matcher that accepts an array of possible values or matchers and confirms that _all_ of the provided items match the result. The matchers to compare to the response value must either be in JSON format as an array, or they can be defined as [variables](../variables.md) using YAML sequence syntax.

## Examples

This example uses an `$and` matcher along with [`$includes`](./includes.md) and [`$regex`](./regex.md) matchers to confirm that the ingredients array contains at least avocado and lime. This uses JSON string syntax to feed the additional matchers as an argument to `$and`.

```yaml
  - test: Proper guacamole
    description: Guacamole must have at least avocados and lime ingredients
    request:
      method: GET
      url: ${BASE_URL}/recipes/${RECIPE_ID}
    expect:
      status: 2xx
      body:
        ingredients: '$and ["$includes $regex /avocado/", "$includes $regex /lime/"]' 

```

Alternatively, and for clarity, you can provide the matchers as a variable - avoiding the need for JSON syntax and additional quotation marks in your tests.

```yaml
variables:
  MIN_GUAC_INGREDIENTS:
    - $includes $regex /avocado/
    - $includes $regex /lime/

tests:
  - test: Proper guacamole
    description: Guacamole must have at least avocados and lime ingredients
    request:
      method: GET
      url: ${BASE_URL}/recipes/${RECIPE_ID}
    expect:
      status: 2xx
      body:
        ingredients: $and ${MIN_GUAC_INGREDIENTS}

```

You are not limited to just two arguments when using the `$and` matcher. You can provide as many as you would like.

```yaml
variables:
  MIN_GUAC_INGREDIENTS:
    - $includes $regex /avocado/
    - $includes $regex /lime/
    - $includes $regex /onion/
    - $includes $regex /jalapeno/
    - $includes $regex /cilantro/

tests:
  - test: The best guacamole
    description: Guacamole must have avocados, lime, onion, jalapeno, and cilantro at least
    request:
      method: GET
      url: ${BASE_URL}/recipes/${RECIPE_ID}
    expect:
      status: 2xx
      body:
        ingredients: $and ${MIN_GUAC_INGREDIENTS}

```

