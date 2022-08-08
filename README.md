# Multirun

CLI tool for running multiple commands at once and multiplexing the output

## Installation

First install the Rust compiler via https://rustup.rs, then run `cargo install multirun`

## Usage

Create a `multirun.json` file then run `multirun`.

Sample `multirun.json`:

```json
{
    "paths": {
        "DEV_ENV_FILE": "./admin/.dev.env",
    },
    "services": {
        "accounts": {
            "directory": "./accounts",
            "command": "npm run start",
            "environment": {
              "ENV_FILE": "${paths.DEV_ENV_FILE}",
              "FORCE_COLOR": "1"
            }
        },
        "compliance": {
            "directory": "./compliance",
            "command": "npm run start",
            "environment": {
              "ENV_FILE": "${paths.DEV_ENV_FILE}",
              "FORCE_COLOR": "1"
            }
        },
        "hasura": {
            "directory": ".",
            "command": "graphql-engine serve",
            "environment": {
                "HASURA_GRAPHQL_DATABASE_URL": "postgres://cw:cw@localhost/cw",
                "HASURA_GRAPHQL_AUTH_HOOK": "http://localhost:3401/hasura/auth-webhook",
                "HASURA_GRAPHQL_SERVER_PORT": "3399",
                "HASURA_GRAPHQL_ADMIN_SECRET": "password123",
                "HASURA_GRAPHQL_ENABLE_CONSOLE": "true",
                "HASURA_GRAPHQL_V1_BOOLEAN_NULL_COLLAPSE": "true"
            }
        }
    }
}
```

Values of variables in the `paths` section and the `directory` field of services are resolved as paths relative to the location of the `multirun.json` files itself.