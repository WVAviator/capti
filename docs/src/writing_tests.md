# Writing Tests

## Example

```yaml
  - test: Get recipe
    description: "Should be able to get recipe information"
    request:
      method: GET
      url: ${BASE_URL}/recipes/${RECIPE_ID}
    expect:
      status: 2xx
      body:
        id: ${RECIPE_ID}
        name: Guacamole
        ingredients: $exists
```

Let's review each component of this example.

- `test` - This is the name of the test as it will appear in your terminal. Keep this short.
- `description` - This is an optional description that describes your test in more detail.
> Note: As of version 0.1.0, `description` does not actually do anything and is akin to a comment. However, in future updates it will hopefully be integrated with any kind of test report output.
- `request` - The HTTP request that you want Capti to make.
  - `method` - The HTTP method - one of "GET", "POST", "PATCH", "PUT", or "DELETE"
  - `url` - The URL to which the request should be made. In the example, _variables_ are used to substitute in parts of the URL. See the section on [variables](variables.md) for more information.
- `expect` - The HTTP response that you expect to get back.
    - `status` - The [HTTP Status Code](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status) you expect your endpoint to return.
    - `body` - The response body you expect to get back - in this case the body has three fields `id`, `name`, and `ingredients`. `id` is being matched to a variable, `name` must exactly match the word "Guacamole", and `ingredients` is using the `$exists` matcher to verify that the ingredients field is non-null. Please see the [matcher section](./matchers.md) for more information on matchers.

## Implicit Matching

Omitted fields in your `expect` definition are matched implicitly with the response. This means that you can focus on testing individual components of the response body by only defining a subset of the response. For example, if the response body returned from your server looks like this:

```bash
{
  "__v": 0,
  "_id": "65d204a107b727ac2667e82f",
  "displayName": "john-smith",
  "email": "testuser3@test.com",
  "id": "65d204a107b727ac2667e82f"
}
```

And all you care about is making sure the `displayName` field is correct, you can simply define your `expect` definition like so:

```yaml
  expect:
    body:
      displayName: john-smith
```

This test will pass. You do not need to include the other fields to get a passing test. 

> Note: In some cases, you do want to ensure a field is _absent_ from the response. For example, say you want to make sure the `password` field does not exist in the body. For this, you can use the `$absent` matcher. Review [Matchers](./matchers.md) for more information on the `$absent` matcher and other matchers.

