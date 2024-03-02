
![Capti](./images/capti_logo.png)

Capti is a lightweight end-to-end testing framework for testing REST APIs. You define your tests in YAML format as an HTTP request and expected response. Leverage the flexible features of Capti, such as matchers and variables, to write high-quality, customized test suites for your applications. Configure automation and scripting to build CI flows to test your endpoints or contract dependencies in staging or production. 

```yaml
  - test: Get recipe
    description: Should be able to get recipe information
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

## Features

- Define [test suites](./configuration/suites.md) to model the behavior of your users.
- Write HTTP endpoint [tests](./configuration/tests.md) that make HTTP requests and assert expected responses.
- Use various [matchers](./matchers.md) in your tests to make pattern-based assertions.
- Define and extract [variables](./variables.md) from responses to reuse repeated values and test authentication and stateful resource flows.
- Provide setup and teardown [scripts](./configuration/scripts.md) for your tests and test suites, enabling CI workflows.

## Next Steps

- View the [installation guide](./installation.md) to learn how to install Capti in your projects.
- Check out the ['Getting Started' guide](./getting_started.md) to write your first Capti test.
- Consider [contributing](./contributing.md) to the project with feature suggestions or bug reports.

