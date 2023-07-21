# feature-flags
Core derived from [crazcalm](https://github.com/crazcalm/feature-flags/tree/main).

Adds support for keys as well as improved API return descriptions and consistency.
Set the port in `.cargo/config.toml`, defaults to `3030`.

### Routes

1. `GET /flags` (Retrieve All Flags)
2. `POST /flags` (Add New Flag)
3. `PUT /flags/<id>` (Update Flag Value)
4. `DELETE /flags/<id>` (Delete Flag)

For routes `2+`, a security key is required. This is set in `.cargo/config.toml` as `SEC_KEY`, encoded into the binary upon build and when deploying the binary is not accesable by the file system, reccord it somewhere safe.

### Request Body
`POST /flags`
```json
{
    "name": "NewFlagName",
    "value": false
    "key": "security-key-here"
}
```

`PUT /flags/<id>`
```json
{
    "value": true,
    "key": "security-key-here"
}
```

`DELETE /flags/<id>`
```json
{
    "key": "security-key-here"
}
```

### Example Response
`GET /flags`
```json
[
    {
        "id": 1,
        "name": "FlagOne",
        "value": true
    },
    {
        "id": 2,
        "name": "FlagTwo",
        "value": false
    }
]
```
