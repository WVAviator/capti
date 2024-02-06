# Surf

Surf is a lightweight end-to-end testing framework for REST APIs. 

## Usage

Test suites are defined in YAML format:

```yaml
suite: "Hello Endpoint Tests"
description: "This suite tests the /hello endpoint"

setup:
  before_all:
    - description: "start the app server"
      script: "npm start"
      wait_until: 5

tests:
  - test: "get hello"
    description: "hello endpoint responds with 'Hello, world!'"
    request:
      method: GET
      url: "http://localhost:3000/hello"
    expect:
      status: 200
      body:
        message: "Hello, world!"
```

Tests can also be configured with matchers.

```yaml
tests:
  - test: "get hello"
    description: "hello endpoint responds with some type of greeting"
    request:
      method: GET
      url: "http://localhost:3000/hello"
    expect:
      status: 2xx # match any 200 level status code
      body:
        message: $regex /[Hh]ello/ # match based on regex
        currentTime: $exists # match anything
```

Any command, script, or program can be executed using the 'setup' functions 'before_all', 'before_each', 'after_each', and 'after_all'.

```yaml
setup:
  before_all:
    - description: "start the app server"
      script: "npm start"
      wait_until: 5 # wait five seconds before continuing to the next command/script
    - script: echo "server started" # description and wait_until fields are optional
  before_each:
    - description: "reset the db"
      script: "./reset_test_db.sh"
      wait_until: finished # waits until the command/script finishes to continue
```

## Planned Features

Surf is under active development and is not production ready. If you want to contribute, feel free to reach out (or just start opening issues and PRs, whatever).
The following features are planned:

1. Support for the remaining HTTP methods
2. Environment variable usage in tests - can sub in API keys and such using curly braces
3. Local or environment variable extraction from responses - useful for authentication flows where something like a JWT token should be carried over to subsequent requests
4. More matchers - such as "$key_exists some_key" for objects, "$item_exists some_item" for arrays, "$starts_with some_prefix", "$contains some_value", etc.
5. Support for adding cookies and expecting cookies to be set
6. An NPM package wrapper for installing in projects and globally

And I'm sure more things wil come up. What would you find useful in a tool like this? Feel free to create an issue.
