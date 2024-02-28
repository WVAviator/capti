# Local Variables

Local variables are defined for a single test. They are primarily useful when using complex matcher arguments and you need to define nested structures to be used in your assertions.

In this example, an array of matchers is defined as a local variable so that it can easily be used with the [`$and`](../matchers/and.md) matcher, which accepts an array of matchers as an argument.

```yaml
tests:
  - test: Required kitchen tools
    description: Guacamole recipe should instruct you to use a fork, spoon, or a molcajete to mix the guacamole.
    request:
      method: GET
      url: ${BASE_URL}/recipes/${RECIPE_ID}
    define:
      GUAC_REQUIRED_TOOLS:
        - $includes $regex /[Ff]ork/
        - $includes $regex /[Ss]poon/
        - $includes $regex /[Mm]olcajete/
    expect:
      status: 2xx
      body:
        instructions: $or ${GUAC_REQUIRED_TOOLS}
```

Local variables have precedence over suite variables, so if desired you can override suite variables for a single test by specifying a variable with the same name in the `define` field. Alternatively, you could just use a different variable.
