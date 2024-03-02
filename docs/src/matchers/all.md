# $all

The `$all` matcher is used to assert that every value in an array returned in the response matches a certain condition. This can be useful for asserting that every object in a list of data has an `id` property, or verify that a user can only see data they are authorized to view.

## Usage

The `$all` matcher takes one argument, a matcher that is compared to every item in the corresponding array. If any of those items fail to match, the test fails.

```yaml
$all <matcher>
```

## Example

This test asserts that every recipe returned from the '/recipes' endpoint contains a `userId` value that matches the current user (which is defined by the [variable](../variables.md) `${USER_ID}`, presumed to have been [extracted](../variables/extracting.md) from an earlier test).

```yaml
  - test: All recipes belong to user
    description: Ensure that the endpoint only returns recipes that belong to the current user
    request:
      method: GET
      url: ${BASE_URL}/recipes
    expect:
      status: 2xx
      body: 
        data: '$all { "userId": "${USER_ID}" }' 
```

Here is another similar test that asserts that every recipe in the list has at least three ingredients and three instructions. This test defines a [local variable](../variables/local.md) to represent the expected match for each item in the list, which in turn uses a [`$length`](./length.md) matcher to verify the associated arrays contain at least three items.

```yaml
  - test: Recipe constraints upheld
    description: Verify that all recipes have at least three ingredients and three instructions.
    request:
      method: GET
      url: ${BASE_URL}/recipes
    define:
      EXPECTED_AMOUNTS:
        ingredients: $length >= 3
        instructions: $length >= 3
    expect:
      status: 2xx
      body: 
        data: $all ${EXPECTED_AMOUNTS}
```
