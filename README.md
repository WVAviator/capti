# Capti

Capti is a lightweight end-to-end testing framework for REST APIs. Define your requests and expected response values in an intuitive YAML format, and streamline your endpoint testing.

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

## Features

- Define [test suites](https://wvaviator.github.io/capti/configuration/suites.html) to model the behavior of your users.
- Write HTTP endpoint [tests](https://wvaviator.github.io/capti/configuration/tests.html) that make HTTP requests and assert expected responses.
- Use various [matchers](https://wvaviator.github.io/capti/matchers.html) in your tests to make pattern-based assertions.
- Define and extract [variables](https://wvaviator.github.io/capti/variables.html) from responses to reuse repeated values and test authentication and stateful resource flows.
- Provide setup and teardown [scripts](https://wvaviator.github.io/capti/configuration/scripts.html) for your tests and test suites, enabling CI workflows.

## Next Steps

- View the [installation guide](https://wvaviator.github.io/capti/installation.html) to learn how to install Capti in your projects.
- Check out the ['Getting Started' guide](https://wvaviator.github.io/capti/getting_started.html) to write your first Capti test.
- Consider [contributing](https://wvaviator.github.io/capti/contributing.html) to the project with feature suggestions or bug reports.

Please visit [the documentation](https://wvaviator.github.io/capti) to learn more about Capti and how you can use it in your projects.

## Planned Development

Capti is under active development and is not production ready. If you want to contribute, feel free to reach out (or just start opening issues and PRs, whatever).

### Upcoming Features

1. More matchers - such as "$key_exists some_key" for objects, "$starts_with some_prefix", "$contains some_value", etc.
2. Testing endpoints under load, testing endpoint throttling or API limits.
3. Support for specifying a local `.env` file for loading variables.
4. Support for printing more detailed results of testing to local files, as well as setting verbose log levels for more information.

### Stretch Features
1. Support for other frameworks?
2. Coverage reports?
3. Plugin API for custom matchers?
3. Whatever you suggest or require for your project.

### Contributing

What would you find useful in a tool like this? Feel free to create an issue or just jump right in and fork/clone/code something up.

To run the app, ensure you already have Rust installed, and you have a REST API project you can test it on (or use the included `test-app`, a simple Express Rest API). Clone the repo locally, and run `cargo build` to create the project binary, located at `./target/debug/capti`. 

Run this binary in a project containing some tests you've written (specify your test directory as an argument to running the binary) following the guidance above. 

Note: If the above step is confusing, take a look at the "test" script in the `test_app` package.json file.
