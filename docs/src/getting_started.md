# Getting Started

## Prerequisites

- A REST API project with at least one HTTP endpoint to test
- Capti installed in the project or globally on your machine. See the [installation instructions](installation.md).

> Note: If you just want to see how Capti works and you have it installed globally, you can use a resource like [JSON Placeholder](https://jsonplaceholder.typicode.com/) and test fake HTTP endpoints.

## Setting up

For the examples in this setup, a NodeJS project will be referenced, but Capti is not limited to use in NodeJS projects. You can use it with Django, Ruby on Rails, Laravel, or any other framework for writing backends.

The project in this example is a simple ts-node Express backend with one endpoint - /hello. This is the entire application:

```js
// src/index.ts
import express from "express";

const app = express();

app.get("/hello", (req, res) => {
  res.send("Hello World!");
});

app.listen(3000, () => {
  console.log("Server is running on port 3000");
});
```

1. Create a directory at the root of your project that will contain all your Capti tests. In this directory, create a new file `hello.yaml`. The directory structure will look something like this:

```
.
├── src/
│   └── index.ts
├── tests/
│   └── hello.yaml
├── .gitignore
├── package-lock.json
├── package.json
└── tsconfig.json
```

2. Add Capti to the project with `npm install --save-dev capti`, and then add the following script to `package.json`:

```json
{
    "scripts": {
        "test:capti": "capti --path ./tests"
    }
}
```

> Note: If you are not using Node, you can skip this step and instead when you want to use Capti, just use the command `capti --path ./tests` instead of `npm run test:capti`.

## Writing your first test

1. Open `hello.yaml` in your favorite text editor. Start by adding the following fields:

```yaml
suite: "Hello endpoint tests"
description: "Tests the various HTTP methods of the /hello endpoint."
```

- `suite` is the title of this test suite. Every file consists of a single test suite which groups together related tests.
- `description` is an optional field that describes the test suite in more detail.

2. Next, add the `tests` array, which will consist of each of the tests we want to run.

```yaml
suite: "Hello endpoint tests"
description: "Tests the various HTTP methods of the /hello endpoint."

tests:
  - test: "Get hello"
```

> Note: If you're unfamiliar with YAML syntax, take a moment to skim over the [YAML specification](https://yaml.org/spec/1.2.2/). While Capti does not use all of the advanced features of YAML, such as tags, knowing the basic syntax around arrays (sequences), objects (mappings), and primitive values can help you better understand what to expect when writing your tests.

3. Let's write our first test. We need to add a `request` field first, which consists of the method, url, optional headers, and optional body of our HTTP request.

```yaml
tests:
  - test: "Get hello"
    request:
      method: GET
      url: http://localhost:3000/hello
```

4. To finish the first test, we need to add the `expect` field, which contains the HTTP response we expect to get back.

```yaml
tests:
  - test: "Get hello"
    request:
      method: GET
      url: http://localhost:3000/hello
    expect:
      status: 200
      body:
        message: "Hello, world!"
```

We defined a status, which represents the [HTTP Status](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status) that we expect our server to return, and we defined a body with one property `message` whose value should exactly equal "Hello, world!".

> Note: Most people are used to seeing response bodies in JSON format - writing them in YAML might take some getting used to. For reference, the above YAML syntax for `body` is equivalent to `{ "message": "Hello, world!" }` in JSON.

## Running your first test suite

Let's start up our server and try this out. Start your server with `npm start` (or whatever start command you usually use). To run our first test, we can run `npm run test:capti` (or if you installed globally, run `capti -p ./tests`).

```bash
✗ [Hello endpoint tests] Get hello... [FAILED]
→ Body does not match.
  Mismatched types
    expected:
    {
      "message": "Hello, world!"
    }

    found: "Hello world!"
```

Uh oh - we have screwed up. From the messages, we can see we were expecting a JSON object, but got a string instead.

Here's the issue in our code:

```js
// src/index.ts
app.get("/hello", (req, res) => {
  res.send("Hello World!");
});
```

This is why we write tests. Let's update it to the correct format.

```js
// src/index.ts
app.get("/hello", (req, res) => {
  res.json({ message: "Hello World!" });
});
```

```bash
✗ [Hello endpoint tests] Get hello... [FAILED]                                                                                                                                    → Body does not match.
  Assertion failed at "Hello, world!" == "Hello World!"
  Mismatch at key "message":
    expected: "Hello, world!"
    found: "Hello World!"
```

Still not quite right, but as you can see - the messages from Capti give us all the info we need to fix our endpoint. Clearly we can see that we are missing a comma and we have uppercased 'W'. Let's update the server one more time.

```bash
== Hello endpoint tests =======

✓ Get hello

Passed: 1 | Failed: 0 | Errors: 0 ▐  Total: 1

== Results Summary =======

Total Tests: 1

Total Passed: 1
Total Failed: 0
Total Errors: 0
```

Now we have a passing test. The two summaries you see are one for the "Hello endpoint tests" test suite (there will be more once you add more test suites) which shows our test "Get hello" has passed. We also have the Results Summary, which shows the test results for all tests.

## Conclusion

Hopefully, with this quick guide, you can see where to go from here. Start writing more tests - for each of your endpoints. Take some time to learn more about how to write good tests with Capti, including:

- How to use matchers to match values based on certain conditions
- How to write setup and teardown scripts and have Capti execute them for you
- How to use variables to write DRY tests
- How to extract variables from responses (such as JWT tokens) and carry them over to subsequent tests
