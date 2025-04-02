### zrpc Types

| Type         | Definition                               |
|:------------:|:---------------------------------------- |
| u32_id       | `32 bytes` generic id                    |
| u64_id       | `64 bytes` generic id                    |
| uname        | `32 bytes` fixed length user name        |
| uemail       | `16 bytes` hashed user email             |
| upassword    | `16 bytes` hashed password value         |
| uavatar      | `6 mb` BLOB value with 6 mb capacity     |
| ubanner      | `32 mb` BLOB value with 32 mb capacity   |
| time         | `4 bytes` uint64 timestamp               |
| lang         | `3 bytes` language code | 'Jap', 'Eng'   |
| text         | `N bytes` of human readable text         |
| bytes        | `N bytes` represented as binary. </br> Primary usage: storing files, videos etc. |
