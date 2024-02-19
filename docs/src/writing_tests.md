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

This test will pass. You do not need to include the other fields to get a passing test. In fact, if your test specifies an _empty_ `expect` definition, it will _always_ pass unless the test throws an error.

> Note: In some cases, you do want to ensure a field is _absent_ from the response. For example, say you want to make sure the `password` field does not exist in the body. For this, you can use the `$absent` matcher. Review [Matchers](./matchers.md) for more information on the `$absent` matcher and other matchers.

## Matchers

You can specify exact values in the `expect` section of each test, or tests can also be configured with special matchers.

```yaml
tests:
  - test: "get hello"
    description: "hello endpoint responds with some type of greeting"
    request:
      method: GET
      url: "http://localhost:3000/hello"
    expect:
      status: 2xx # match any 200 level status code
      headers:
        Content-Type: application/json # exact match
      body:
        message: $regex /[Hh]ello/ # match based on regex
        currentTime: $exists # match anything as long as it is present
```

For more information on matchers, review the [matchers chapter](./matchers.md).

## Variables

Static variables can be defined for each test suite with the `variables` option, and then used in test definitions like `${this}`. When the test is run, each variable will be expanded in place.

```yaml
suite: "User Signup"
description: "Confirms that protected routes cannot be accessed until the user signs up."
variables:
  BASE_URL: "http://localhost:3000"
  USER_EMAIL: "testuser2@test.com"
  USER_PASSWORD: "F7%df12UU9lk"

tests:
  - test: "Sign in"
    description: "The user should be able to sign in with email and password"
    request:
      method: POST
      url: "${BASE_URL}/auth/signin"
      headers:
        Content-Type: application/json
      body:
        email: ${USER_EMAIL}
        password: ${USER_PASSWORD}
    expect:
      status: 2xx
      body:
        id: $exists
        email: ${USER_EMAIL}
        password: $absent
```

Environment variables can be referenced in the same way. If a variable is set in both your local environment and in the `variables` section, the value specified in the `variables` section will take precedence.

Variables can also be "extracted" from responses and used in subsequent tests by defining an `extract` section for a test.

```yaml
tests:
  - test: "sign in"
    description: "Sign in as the test user"
    request:
      method: POST
      url: "${BASE_URL}/auth/signup"
      headers:
        Content-Type: application/json
      body:
        email: ${USER_EMAIL} # email and password defined as variables in the test suite for easy reuse throughout tests
        password: ${USER_PASSWORD}
    expect:
      status: 2xx
    extract:
      headers:
        Authorization: Bearer ${JWT_TOKEN} # extracts the token variable from the response
      body:
        userId: ${USER_ID} # extracts the user id generated by the database

  - test: "access protected route"
    description: "After signing in, the user can get their profile data"
    request:
      method: GET
      url: "${BASE_URL}/profile/${USER_ID}" # extracted variables can be used just like any other
      headers:
        Authorization: Bearer ${JWT_TOKEN} # great for auth flows
    expect:
      status: 2xx
      body:
        firstName: $exists
        lastName: $exists
        imageUrl: $regex /.*\.png$/
```

> Note: Each suite manages its own cookies internally, enabling session authentication to work automatically within a suite. There is no need to extract cookies to carry over between requests.

For more information on variables, please review the [variables chapter](./variables.md).
