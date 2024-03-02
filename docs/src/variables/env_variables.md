# Environment Variables

By default, if you specify a variable in your tests but the declaration for that value cannot be found, Capti will then default to searching your local environment for that variable.

For example, if you have a `SERVER_URL` variable defined in your local environment, you can use that like any other variable:

```yaml
  - test: Get a user
    request:
      method: GET
      url: ${SERVER_URL}/users
    expect:
      status: 200
```

However, since variables defined in your test suite take precendence, if you were to define the SERVER_URL in your suite configuration, that value will be used instead.

```yaml
suite: User endpoint tests
variables:
  SERVER_URL: http://localhost:4000

tests:
  - test: Get a user
    request:
      method: GET
      url: ${SERVER_URL}/users
    expect:
      status: 200
```

## Env File

If you want to load variables into your environment from a `.env` file in your project, you can specify the path to your `.env` file in your [global config](../configuration/config.md). These variables are not loaded by default.

Just as with environment variables that already exist in your terminal environment, variables loaded from `.env` files will never overwrite variables defined in your test suites.

> Note: Currently, variables loaded from `.env` are not available when declaring variables in your test suites, so you cannot compose static variables from `.env` variables. This is expected to change in the future.
