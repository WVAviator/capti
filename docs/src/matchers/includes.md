# $includes

The `$includes` matcher is used to verify that a specific value exists in an array.

## Usage

```
$includes <value or matcher>
```

The `$includes` matcher takes one argument, which is a value or matcher that should be found in the array. It can be used to match primitive values:

```
$includes 5
$includes "Hello, world!"
$includes true
```

It can be used to match objects or other arrays, following the same [implicit matching](../writing_tests.md#implicit-matching) rules used in normal `expect` definitions. However, because of limitations of YAML, these objects or arrays must be defined as JSON wrapped in quotes, or separately as [variables](../variables.md).

```
$includes "{ "id": "1A2B3C" }"
$includes "[1, 2, 3]"
```

It can also be used to match other matchers. For example, checking if a string that matches a regex using the [`$regex`](./regex.md) matcher:

```
$includes $regex /[Hh]ello/
# would match ["A", "B", "hello"]
```

Or matching another `$includes` to search nested arrays for a value:

```
$includes $includes 5
# would match [[1, 2, 3], [4, 5, 6]]
```

## Examples

Here is a simple example that checks if an object with the specified "id" property is included in a returned data array.

```yaml
tests:
  - test: Recipe included in list
    request:
      method: GET
      url: http://localhost:3000/recipes
    expect:
      body:
        data: $includes "{ "id": "${RECIPE_ID}" }"
```

For a more specific check, the expected item can first be defined as a variable in the suite. The expected value can still be defined as YAML and later will be expanded to the full value when the test is run.

```yaml
variables:
  RECIPE:
    name: Guacamole
    description: A delicious classic guacamole recipe.
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
    instructions: [
      "Roughly chop onion, green pepper, and jalapeno and add to food processor. Pulse 2-3 times.",
      "Cut and remove seeds from avocados, use spoon to scoop and mash in a bowl.",
      "Mix in the vegetables, seasonings, and lime juice squeezed from a fresh lime." ]

tests:
  - test: Recipe included in list
    request:
      method: GET
      url: http://localhost:3000/recipes
    expect:
      body:
        data: $includes ${RECIPE} 
```
