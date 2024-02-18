# $absent

The `$absent` matcher is the inverse of [`$exists`](./exists.md) - where `$exists` matches any non-null value, `$absent` only matches null or missing values. 

More specifically, it will match:

- `null`
- Values where the key/value pair is completely missing

## Example

In the example below, we are calling an endpoint to get user information from the server. In our `expect` defintion, we assert that the `password` field should be null or missing, and we didn't accidentally include the password hash with the response after fetching user data from the database.

```yaml
tests:
  - test: "Get user info"
    request:
      method: GET
      url: http://localhost:3000/user/${USER_ID}
    expect:
      email: test@test.com
      password: $absent
```
