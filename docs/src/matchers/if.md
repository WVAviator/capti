# $if

The `$if` matcher, as expected, evaluates a secondary match based on the result of a previous match. The argument to the `$if` matcher is an array/sequence of either two or three items.

If two items are provided, the statement is evaluated as an `if/then` match. If the first item evaluates to `true`, then the second item must also evaluate to `true`. If the first item evaluates to `false`, then the second item is never evaluated and the whole statement returns `true`, passing the test.

|if|then|result|
|-|-|-|
|true|true|true|
|true|false|false|
|false|true|true|
|false|false|true|

If three items are provided, the statement is evaluated as an `if/then/else` match. The first and second items are evaluated the same way as an `if/then` match, except that if the first item returns `false`, then the third item must evalaute to `true` for the test to pass.

|if|then|else|result|
|-|-|-|-|
|true|true|true|true|
|true|true|false|true|
|true|false|true|false|
|true|false|false|false|
|false|true|true|true|
|false|true|false|false|
|false|false|true|true|
|false|false|false|false|

## Example

This example uses the `$if` matcher, along with the [`$regex`](./regex.md) and [`$includes`](./includes.md) matchers, to assert that if any recipes in the list contain the word "guacamole" in the name, that they also contain "avocado" somewhere in the list of ingredients.

The example uses a [local variable](../variables/local.md) to provide the `$if` arguments in a YAML sequence. Each item in the sequence is an object with either the property `name` or `ingredients`. 

```yaml
  - test: Guac recipes have avocado
    description: Any guacamole recipes in the list should contain avocado as an ingredient
    request:
      method: GET
      url: ${BASE_URL}/recipes
    define:
      RECIPE_IF:
        - name: $regex /[Gg]uacamole/
        - ingredients: $includes $regex /[Aa]vocado/
    expect:
      status: 2xx
      body: 
        data: $includes $if ${RECIPE_IF}
```

