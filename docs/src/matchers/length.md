# $length

The `$length` matcher can be used to assert the length of arrays, objects, and strings. You can assert using exact values, or a custom _value matcher_.

## Usage

```
$length <arg>
```

Valid arguments include:

```yaml
$length 5 # length is exactly 5
$length > 5 # length is greater than 5
$length >= 5 # length is greater than or equal to 5
$length < 5 # length is less than 5
$length <= 5 # length is less than or equal to 5
$length == 5 # length is exactly 5, same as first example
```

## Example

```yaml
tests:
  - test: At least two comments
    description: The comments field should have at least two comments
    request:
      method: GET
      url: http://localhost:3000/post/${POST_ID}
    expect:
      post:
        comments: $length >= 2
```
