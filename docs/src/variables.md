# Variables

Variables are one component of Capti test suites that provide a lot of power. They enable features such as testing authorization flows, embedding complex mappings or sequences as matcher arguments, or simply reducing boilerplate and making your tests more DRY.

Variables can be defined statically as part of the suite configuration, or dynamically with `extract` definitions.

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
