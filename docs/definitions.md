# Useful definitions & terms

### @scheme 

```
scheme scheme_name {
  @code_block
}
```

Besides being a keyword, the scheme is primarily used for defining [@user_defined_types]() of data, and then using them for creating typed RPC communication between server client, via [@route]().

### @route 

```
route route_name {
  request {
    @code_block
  }
  response {
    @code_block
  }
}
```

Besides being a keyword, the route is primarily used for providing establishing communication between client-server.

### @zrpc_field

`field_name: type` 

| Keyword       |Description|
|:-------------:|:----------|
|`field_name`   | Field name that valid with this regex: `/[a-zA-Z_][a-zA-Z0-9_]*/`|
|`:`            | Punctuation delimiter. <br/> Note! There must be no space between field_name and delimiter <br/> valid syntax `field_name: type` <br/> will result in error `field_name  : type` |
|`type`         | One of supported zrpc [types](https://github.com/Akzestia/zrpc/blob/main/docs/types.md) or a user defined one.

### @code_block

```
keyword keyword_name {
  inside keyword @code_block fields
}
```

Code block is the part of [scheme](), [route]() warpped inside `{}`, which describes how the [scheme](), [route]() are defined inside .zrpc schemes.
