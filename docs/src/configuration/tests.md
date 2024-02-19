# Test Configuration

In addition to defining a `request` and an `expect` mapping for each test, you can also define the following settings.

## Print Response

By setting `print_response: true` on your tests, the complete response status, headers, and body will be printed to the console when your test is run. This can be useful for debugging a failing test.

## Should Fail

Setting `should_fail: true` on your test, as expected, will assert that the test should fail. In most cases, however, you should be able to acheive this functionality with the right [matchers](../matchers.md) in your `expect` definition.