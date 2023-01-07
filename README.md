# Figma API tooling

An experiment to see what can be achieved with the
[Figma REST API](https://www.figma.com/developers/api).

All commands require an API file to be provided on `stdin`.

```bash
cargo run -- design-tokens < example-figma-files/design-tokens-for-figma.json
# or
curl -sH "X-Figma-Token: ${FIGMA_TOKEN}" https://api.figma.com/v1/files/2MQ759R5kJtzQn4qSHuqR7 | cargo run -- design-tokens
```

## Attribution

I've included Figma files for testing and demonstrations. All the files are
fetched from the API and the only change is pretty formatting the JSON. Where
required by Figma I have made my own copy.

https://www.figma.com/file/2MQ759R5kJtzQn4qSHuqR7/Design-Tokens-for-Figma by
Lukas Oppermann and
[used with permission](https://github.com/lukasoppermann/design-tokens/issues/238).

https://www.figma.com/community/file/946837271092540314 by Joe Horton and
licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/).

## Component interfaces

Generate TypeScript props for components.

```bash
cargo run --release -- component-interfaces < example-figma-files/gov-uk-design-system.json > gov-uk-design-system-interfaces.ts
```

## Design tokens

A `design-token` subcommand inspired by the
[design-tokens Figma plugin](https://github.com/lukasoppermann/design-tokens).

```bash
cargo run --release -- design-tokens < example-figma-files/design-tokens-for-figma.json
```

Due to limitations with the Figma API, I do not recommend this subcommand. Some
basic information cannot be obtained through the API such as colour token values
or font information. I do recommend the plugin.

[Read more](src/design_tokens/README.md)
