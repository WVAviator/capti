# $empty

The `$empty` matcher checks that an object, array, or string has length 0.

Because of Capti's [implicit matching](../writing_tests.md#implicit-matching), you cannot simply write something like this:

```yaml
  expect:
    body:
      comments: []
```

The goal here is to assert that the comments array is empty, however because of implicit matching, this will match _any number_ of comments.

To properly assert that there are no comments in this array, you can use the `$empty` matcher instead.

```yaml
  expect:
    body:
      comments: $empty
```

You can also use `$empty` to match empty objects or strings. Using `$empty` is identical to writing [`$length 0`](./length.md).
