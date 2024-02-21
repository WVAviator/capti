# Suite Configuration

You have a few options to work with when configuring your test suites. You can define [setup scripts](./scripts.md) that execute before or after your tests, specify whether tests should run in parallel or sequentially, and specify static variables to be used throughout your test suite.

## Setup Scripts

See [setup scripts](./scripts.md) for more information on how to create setup scripts. These scripts execute command line programs or utilities before and after your tests, and can be useful if you want to do some specific configuration, like resetting a database, before testing.

## Parallel Testing

In general, you should prefer sequential testing over parallel testing. Capti test suites are each meant to simulate individual users interacting with your application, and a user would not typically be visiting multiple endpoints concurrently.

> Note: When you have multiple test suites defined, the test _suites_ will always run concurrently. Each suite should be designed to simulate a user, and multiple users should be able to interact with your API concurrently and deterministically. Your suites should never rely on the state of other test suites. The individual _tests_ in a suite should, in the majority of cases, run sequentially.

There are some cases in which the tests within a suite should run in parallel. One example would be if you are grouping together multiple tests of several different _public_ and _stateless_ endpoints. In these cases, you can specify in your test suites that all tests should run in parallel with `parallel: true`.

```yaml
suite: "Published recipes"
description: "This suite tests multiple sequential accesses to the public endpoints returning published recipe information."
parallel: true
```

> Note: You cannot _extract_ variables when specifying `parallel: true`. Referencing an extracted variable in a later request is not possible when all requests run concurrently.

## Variables

You can define static variables to be used throughout the tests in your suites with the `variables:` mapping. These variables will expand to the specified value, sequence, or mapping when they are used. You can learn more in the [variables chapter](../variables.md).

```yaml
suite: "Create Recipe"
description: "This suite involves creating a new recipe and fetching its information."
variables:
  BASE_URL: http://localhost:3000
  USER_EMAIL: recipe1@tests.com
  USER_PASSWORD: abc123!
```