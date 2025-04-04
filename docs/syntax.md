# zrpc syntax rules

### Keywords

| KeyWord | Description |
|:---------:|:-------------|
|`scheme`     | Defines a zrpc scheme | 
|`route`      | Defines a zrpc route |
|`request`    | Only used inside the `route` block, for defining request scheme |
|`response`   | Only used inside the `route` block, for defining response scheme |


### Brackets & Staff

`Scheme` @code_block
```
scheme @scheme_name {
  @zrpc_field
  @zrpc_field
  @zrpc_field
  ...
}
```

`Route` @code_block
```
route @scheme_name {
  request {
    @zrpc_field
    @zrpc_field
    ...
  }
  response {
    @user_defined_type
    @zrpc_field
    ...
  }
}
```

> [!TIP]
> [@scheme](https://github.com/Akzestia/zrpc/blob/main/docs/definitions.md#scheme) <br/>
> [@scheme](https://github.com/Akzestia/zrpc/blob/main/docs/definitions.md#route) <br/>
> [@zrpc_field](https://github.com/Akzestia/zrpc/blob/main/docs/definitions.md#zrpc_field) <br/>
> [@code_block](https://github.com/Akzestia/zrpc/blob/main/docs/definitions.md#code_block) <br/>

### Usefull tips

