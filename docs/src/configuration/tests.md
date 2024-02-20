# Test Configuration

In addition to defining a `request` and an `expect` mapping for each test, you can also define the following settings.

## Print Response

By setting `print_response: true` on your tests, the complete response status, headers, and body will be printed to the console when your test is run. This can be useful for debugging a failing test.

```bash
== Response: (Sign up) =======

  Status: 200

  Headers:
    ▹ "x-powered-by": "Express"
    ▹ "content-type": "application/json; charset=utf-8"
    ▹ "content-length": "130"
    ▹ "etag": "W/"82-l0Mhda3RFUb75lW/cRtznG5a9jI""
    ▹ "set-cookie": "connect.sid=s%3A0D8I6wmav5gUclgFPWA9u9WvCQ4oSNo7.u7xk7r6XkMbMdwsVtwArBZ1Q0DFT0pzo72tWRuh9JA8; Path=/; HttpOnly"
    ▹ "date": "Sat, 17 Feb 2024 21:55:29 GMT"
    ▹ "connection": "keep-alive"
    ▹ "keep-alive": "timeout=5"

  Body:
    {
      "email": "testuser3@test.com",
      "displayName": "john-smith",
      "_id": "65d12b5182456857b2b9c8ce",
      "__v": 0,
      "id": "65d12b5182456857b2b9c8ce"
    }

==============================
```


## Should Fail

Setting `should_fail: true` on your test, as expected, will assert that the test should fail. In most cases, however, you should be able to acheive this functionality with the right [matchers](../matchers.md) in your `expect` definition.

This example uses the `should_fail` attribute to ensure the test does not pass with a successful status.

```yaml
  - test: "Protected route"
    description: "Attempting to access protected route without signin or signup"
    should_fail: true
    request:
      method: GET
      url: "${BASE_URL}/recipes"
    expect:
      status: 2xx
      body:
        recipes: $exists
```

However, a more declarative and idiomatic pattern would be to use matchers to assert the expected 400-level status code and absent request body information. This also enables asserting the correct error status code - in the case that the endpoint actually returns a 404 or 500-level status, the above test would pass, whereas this test would still detect the error and fail.

```yaml
  - test: "Protected route"
    description: "Attempting to access protected route without signin or signup"
    request:
      method: GET
      url: "${BASE_URL}/recipes"
    expect:
      status: 403
      body:
        recipes: $absent
```

