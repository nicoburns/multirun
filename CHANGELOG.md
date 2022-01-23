# Changelog

# 0.3.0
- Support for relative paths in services directories (paths are resolved relative to the config file's directory)
- Support for `paths` section of config file (also support relative path resolution)
- Support for interpolating varariables from the paths section into environment variables (syntax is `${paths.VAR_NAME}`
- Upgrade dependencies (incl. tokio to 1.x)
- Upgrade to Rust 2021 edition

# 0.2.1
- Print version number when called with `--version`

# 0.2.0
- Basic support for environment variables in the config file

# 0.1.0
- Initial version
- Basic support for running multiple commands in different directories and multiplexing the log output