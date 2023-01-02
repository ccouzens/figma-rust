# Figma API tooling

An experiment to see what can be achieved with the
[Figma REST API](https://www.figma.com/developers/api).

All commands require an API file to be provided on `stdin`.

```bash
cargo run -- design-tokens < example-figma-files/design-tokens-for-figma.json
# or
curl -sH "X-Figma-Token: ${FIGMA_TOKEN}" https://api.figma.com/v1/files/2MQ759R5kJtzQn4qSHuqR7 | cargo run -- design-tokens
```

## Design tokens

A `design-token` subcommand inspired by the
[design-tokens Figma plugin](https://github.com/lukasoppermann/design-tokens).

```bash
cargo run --release -- design-tokens < example-file.json
```

Due to limitations with the Figma API, I do not recommend this subcommand. Some
basic information cannot be obtained through the API such as colour token values
or font information. I do recommend the plugin.

[Read more](src/design_tokens/README.md)
