# Variables

Variables are one component of Capti test suites that provide a lot of power. They enable features such as testing authorization flows, embedding complex mappings or sequences as matcher arguments, or simply reducing boilerplate and making your tests more DRY.

## Variable Types

Variables can be defined statically as part of the suite or test configuration, or dynamically with `extract` definitions. They can also be pulled from the environment or from a `.env` file.

- Global Variables - Defined at the suite level under the field `variables`. They are available to all tests within the suite. They are most useful for defining values that you intend to repeat multiple times throughout your tests.
- Extracted Variables - These are extracted from your tests an can be used in any subsequent tests. They are useful for comparing dynamic values such as unique identifiers for resources or authentication tokens.
- Local Variables - These are defined at the test level under the field `define`. They are only valid for that single test. They can be useful for specifying complex structures to be used as matcher arguments, or they can be used to override suite variables for a single test.
- Environment Variables - These are pulled from your shell environment or from a [env file](./configuration/config.md#environment-variables). They are useful for synchronizing values in your tests that are also available to your server or other services.

## Variable Precendence

Variables can be defined as any or all of the above types, and certain types take precendence over others. In order from highest precendence:

> Local -> Extracted / Global -> Environment (.env) -> Environment (Shell)

Local variables will be applied first. Next, global or extracted variables are used (extracting a variable with the same name as a global variable permanently overrides the global variable). Lastly, if no variable is found in the local or global space, the env file and shell environment are searched - with env file variables taking precedence over shell environment as well.

It's generally advised to avoid clashing variable names anyway.

## Simple Values

The basic usage of variables is to reduce repetitive values, such as a `BASE_URL` for your endpoints.

```yaml
suite: "Create Recipe"
description: "This suite involves creating a new recipe and fetching its information."
variables:
  BASE_URL: http://localhost:3000

tests:
  - test: Fetch recipes
  description: List of public recipes contains at least one recipe
  request:
    method: GET
    url: ${BASE_URL}/recipes
  expect:
    status: 2xx
    body: 
      data: $length >= 1
```
