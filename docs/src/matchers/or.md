# $or

The `$or` matcher is a logical matcher that compares the response value to an array or sequence of possible values or matchers, allowing the test to pass if _any_ of the provided values or matchers match the response. The matchers to compare to the response value must either be in JSON format as an array, or they can be defined as [variables](../variables.md) using YAML sequence syntax.

## Examples


This example uses an `$or` matcher along with [`$includes`](./includes.md) and [`$regex`](./regex.md) matchers to confirm that the instructions array contains either the word 'mash' or 'stir'. This uses JSON string syntax to feed the additional matchers as an argument to `$and`.

```yaml
  - test: Mash or stir
    description: Guacamole recipe should instruct you to either mash or stir the avocados.
    request:
      method: GET
      url: ${BASE_URL}/recipes/${RECIPE_ID}
    expect:
      status: 2xx
      body:
        instructions: '$or ["$includes $regex /[Mm]ash/", "$includes $regex /[Ss]tir/"]'
```

Alternatively, and for clarity, you can provide the matchers as a variable - avoiding the need for JSON syntax and additional quotation marks in your tests.

```yaml

tests:
  - test: Mash or stir
    description: Guacamole recipe should instruct you to either mash or stir the avocados.
    request:
      method: GET
      url: ${BASE_URL}/recipes/${RECIPE_ID}
    define:
      GUAC_REQUIRED_INSTRUCTIONS:
        - $includes $regex /[Mm]ash/
        - $includes $regex /[Ss]tir/
    expect:
      status: 2xx
      body:
        instructions: $or ${GUAC_REQUIRED_INSTRUCTIONS}
```

You are not limited to just two arguments, you may specify as many as you would like and `$or` will retrun true as long as at least one is true.

```yaml
tests:
  - test: Required kitchen tools
    description: Guacamole recipe should instruct you to use a fork, spoon, or a molcajete to mix the guacamole.
    request:
      method: GET
      url: ${BASE_URL}/recipes/${RECIPE_ID}
    define:
      GUAC_REQUIRED_TOOLS:
        - $includes $regex /[Ff]ork/
        - $includes $regex /[Ss]poon/
        - $includes $regex /[Mm]olcajete/
    expect:
      status: 2xx
      body:
        instructions: $or ${GUAC_REQUIRED_TOOLS}
```

