# Useful definitions & terms


### @zrpc_field

`field_name: type` 

| Keyword       |Description|
|:-------------:|:----------|
|`field_name`   | Field name that valid with this regex: `/[a-zA-Z_][a-zA-Z0-9_]*/`|
|`:`            | Punctuation delimiter. <br/> Note! There must be no space between field_name and delimiter <br/> valid syntax `field_name: type` <br/> will result in error `field_name  : type` |
|`type`         | One of supported zrpc [types](https://github.com/Akzestia/zrpc/blob/main/docs/types.md) or a user defined one.
