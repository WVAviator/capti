# $not

The `$not` matcher simply matches the opposite of the provided matcher or value.

## Examples

```yaml
  - test: Recipe not guacamole
    description: This recipe should not be named 'Guacamole'
    request:
      method: GET
      url: ${BASE_URL}/recipes/${RECIPE_ID}
    expect:
      body:
        title: $not Guacamole
```

The `$not` matcher can match other matchers as well. This example uses a [`$regex` matcher](./regex.md) to ensure that a field in the response does not contain any quotation marks. 

```yaml
  - test: No quotes in name
    description: The recipe title should not have quotes in the name
    request:
      method: GET
      url: ${BASE_URL}/recipes/${RECIPE_ID}
    expect:
      body:
        title: $not $regex /\"+/
```

In this more complex example, the `$not` matcher negates an [`$includes` matcher](./includes.md) to confirm that an object containing the specified id does not appear in the array.

```yaml
  - test: Deleted recipe gone
    description: The now-deleted recipe should no longer appear in the list of recipes
    request:
      method: GET
      url: ${BASE_URL}/recipes/all
    expect:
      status: 2xx
      body:
        data: '$not $includes { "id": "${RECIPE_ID}" }'
```

