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

## Installation

Capti is currently available as an NPM package, however Capti is framework-agnostic and can work with any REST APIs. The raw binaries are available on the [GitHub repo](https://github.com/WVAviator/capti/releases) if you prefer not to use NPM.

Note: Support for other package managers and testing frameworks is under consideration - please see the notes below about [contributing](#contributing) if you have any suggestions or requests.

To install Capti globally and use it anywhere:

```bash
$ npm install -g capti
```

### NodeJS

This will install Capti locally in a single NodeJS project. If you prefer to use the `capti` command directly, use the global install script above. To install Capti in your NodeJS project:

```bash
$ npm install --save-dev capti
```

Then edit your package.json scripts by adding a `test:capti` script:

```json
{
    ...
    "scripts": {
        ...
        "test:capti": "capti --path './tests'"
    }
    ...
}
```

You can then run your test suites like so:

```bash
$ npm run test:capti
```

Note: You can customize this however you want - the script can be called `test:e2e` instead of `test:capti`, and the path you define (`'./tests'` in the example) should point to the directory in which you plan to write all your Capti tests.

## Basic Usage

To run Capti after downloading, use the `capti` command (or `npm run test:capti` if you installed Capti in a single NodeJS project) with the `--path` or `-p` argument specifying the directory in which your tests are located.

```bash
$ capti -p ./my-tests
```

Note: If you omit the `--path` / `-p` argument, by default Capti will run from your current working directory. This could cause significant performance overhead, as Capti will search every single nested directory for YAML files and check each file to see if it's a Capti test. If you have a lot of directories in your project, this could be problematic.

### Tests

Test suites are defined in YAML format:

```yaml
suite: "Hello Endpoint Tests"
description: "This suite tests the /hello endpoint"

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

Each test consists of a `request` section that includes the request `method` (GET, POST, PATCH, etc), optional `headers`, a target `url`, and an optional `body`. Each test also includes an `expect` section which defines the shape of the expected response that should be returned from the server. Each `expect` section consists of optional `status`, `headers`, and `body` sections.

Note: Any descriptions at the suite or test level are optional.

### Matchers

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

Anything not specified in the `expect` section of a test that is present in the actual response will be ignored. An empty `expect` will match _any_ response and the test will always pass.

See [More Matchers](#more-matchers) in the section below.

Note: If you want to assert that a test should fail, you can include an optional `should_fail: true` in the test definition.

### Setup Scripts

You can define actions to take before/after your tests using the 'setup' option in a test suite. Any shell command, script, or program can be executed using the 'setup' functions 'before_all', 'before_each', 'after_each', and 'after_all'.

```yaml
setup:
  before_all:
    - description: "start the app server"
      script: "npm start"
      wait_until: output "Listening on port 3000"

    - script: echo "server started" # description and 'wait_until' are optional

  before_each:
    - description: "reset the db"
      script: "./reset_test_db.sh"
      wait_until: finished
```

For each setup script, the `wait_until` option can be in the following formats:

```yaml
  wait_until: 5 seconds # wait a specified number of seconds
  wait_until: port 3000 # waits until a service becomes available on a port
  wait_until: output "Server listening on port 3000" # waits for specific console output from the process stdout
  wait_until: finished # waits until the script finishes running to continue
```

Omitting the `wait_until` option means the script will run concurrently in the background, and the remaining scripts or test suite will immediately continue.

### Config

If you need to run 'before_all' or 'after_all' setup scripts only once for an entire collection of test suites, you can define a `capti-config.yaml` in your test directory with the same setup syntax pictured above. 

Optionally you can specify a custom config file and location by passing the path to your config as the `--config` or `-c` command line argument to Capti.

```bash
$ capti --path ./tests --config ./my-custom-config.yaml
```

Here is an example config that works with Docker Compose on Unix/Linux systems to check if the containers are already up and if not, starts them.

```yaml
setup:
  before_all:
    - description: "Start db and server"
      wait_until: output "Listening on 3000"
      script: >
        if ! docker-compose ps | grep -q " Up "; then
            docker-compose up
        else
            echo "Listening on 3000"
        fi
```

Note: Running shell scripts before or after your tests is entirely optional - it's fine to start your server and/or database manually and then run Capti, just understand that if your server is not running, your tests will fail (obviously).

### Variables

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

Note: Each suite manages its own cookies internally, enabling session authentication to work automatically within a suite. There is no need to extract cookies to carry over between requests.

### Concurrency

By default, all tests in a suite are executed sequentially in the order that they are defined. This is important if your tests rely on a user flow, especially when auth is involved and variables need to be extracted from the responses.

Under certain situations, the tests in a suite can be run concurrently. Retrieving public information from several GET endpoints would be a prime example. In these cases, all tests can be run in parallel, by specifying `parallel: true` in a suite.

```yaml
suite: "Public Endpoint Tests"
description: "This suite tests several public information endpoints simultaneously."
parallel: true

tests:
  ...
```

Note: When using `parallel: true`, you cannot `extract` variables from responses, as the order of tests cannot be guaranteed.

Suites always run concurrently. You should not design your API tests to where any of your test suites rely on each other. The idea is to design each test suite to mock the flow of a typical user, and multiple users should be able to access your API concurrently and deterministically.

### More Matchers

- `$exists` - This matcher is a catchall for verifying that a header or response body returned _something_. Literally anything will match except null, undefined, or nothing.

- `$absent` - This matcher asserts that a certain property does not exist in the response. This is useful for verifying that endpoints returning user information don't accidentally include fields like `password` or other sensitive information.

- `$empty` - This matcher is used for arrays and objects, and asserts that they are empty. Simply providing an empty field as a matcher will match _any_ object or array, so this matcher should be used instead.

- `$regex` - This matcher allows you to specify a regex that should match the returned value. The regex must be defined between forward slashes, for example: `/[Hh]ello[,\ ]+[Ww]orld!?/`. Note that unless you include the start/end-of-string matchers `^ $`, this will match _any_ part of the returned value. Example:

```yaml
  - test: Recipe description mentions guacamole
    description: "Not sure why, but the description should mention guacamole at least once"
    request:
      method: GET
      url: ${BASE_URL}/recipes/${RECIPE_ID}
    expect:
      status: 2xx
      body:
        id: ${RECIPE_ID}
        name: Guacamole
        description: $regex /([Gg]uacamole)+/
```

Note: Regex matchers only work on string values and will return false otherwise.

- `$includes` - This matcher is useful for ensuring values exist in arrays. Any value that follows this keyword is incorporated as a matcher itself and follows the same matching rules. You can match strings, booleans, and even objects (however, you will have to define them as JSON strings). Example:

```yaml
  - test: Recipe in list
    description: Recipe should be visible when listing all recipes
    request:
      method: GET
      url: ${BASE_URL}/recipes
    expect:
      status: 2xx
      body:
        data: '$includes { "id": "${RECIPE_ID}" }' # checks that the data array includes at least one object with the specified ID
```
Note: Technically you can even match values in nested arrays with `$includes $includes some-value`, since `$includes` is a valid matcher itself. `$includes $includes some-value` will check every inner array in an outer array (e.g. every cell in every row) to see if it contains a string `some-value`.

### Other Features

Here are some additional features of Capti that don't fit into any of the categories above.

- For helpful debugging, you can set the property `print_response: true` on your tests, and the full response returned from your server will be printed in the console. 

- To assert that a test _should fail_ rather than pass, you can set the property `should_fail: true` on the test. However, most of the time you should be able to get what you need using matchers and if not - make a suggestion!

- Headers are matched case-insensitive. Newer recommendations in the HTTP specifications suggest using lowercase, and most frameworks will default to using lowercase. If you specify capitalized headers in your `expect` definition, they will still match their lowercase versions.


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
3. Whatever you suggest or require for your project.

### Contributing

What would you find useful in a tool like this? Feel free to create an issue or just jump right in and fork/clone/code something up.

To run the app, ensure you already have Rust installed, and you have a REST API project you can test it on (or use the included `test-app`, a simple Express Rest API). Clone the repo locally, and run `cargo build` to create the project binary, located at `./target/debug/capti`. 

Run this binary in a project containing some tests you've written (specify your test directory as an argument to running the binary) following the guidance above. 

Note: If the above step is confusing, take a look at the "test" script in the `test_app` package.json file.
