# $exists

The `$exists` matcher checks that an object or value exists in the response.

More specifically, it will match everything except:

- `null`
- Values where the key/value pair is completely missing

## Example

This is example is calling a POST endpoint to create a recipe. When the recipe is returned, it will have been given an `id` property by the database. We do not know what this `id` will be, and therefore we cannot assert that it should be any particular value. We do, however, want to make sure it exists in the response. This is a perfect use case for `$exists`.

```yaml
  - test: Create new recipe
    description: Create a new recipe
    request:
      method: POST
      url: http://localhost:3000/recipes 
      headers:
        Content-Type: application/json
      body: ${RECIPE}
    expect:
      status: 2xx
      body:
        id: $exists
```
