### Defining RPC with zrpc

```
route GetUser {
    request {
        user_name: uname
    },
    response {
        user: User
    }
}
```
```
User {
  user_name: uname,
  user_avatar: uavatar,
  ...
}
```
