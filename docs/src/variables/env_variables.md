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