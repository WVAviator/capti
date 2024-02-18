# $regex

The `$regex` matcher uses a provided regular expression to determine whether a match is found in the specified response field.

## Usage

The `$regex` matcher takes one argument, a regular expression wrapped in forward slash characters.

```
$regex /<regular expression>/
```

> Note: Unless you include start/end matchers `^` and `$` in your regular expression, `$regex` will match _any_ contained instances of the value. The `$regex` matcher also only matches strings - any attempt to match another value type will simply result in the test failing.

## Example

This example matches the `description` fields returned by the response body and matches any description which contains one or more matches of the word "guacamole" with a case-insensitive 'G'.

```yaml
  - test: Recipe description mentions guacamole
    description: "Not sure why, but the description should mention guacamole at least once"
    request:
      method: GET
      url: ${BASE_URL}/recipes/${RECIPE_ID}
    expect:
      status: 2xx
      body:
        id: ${RECIPE_ID}
        name: Guacamole
        description: $regex /([Gg]uacamole)+/
```
