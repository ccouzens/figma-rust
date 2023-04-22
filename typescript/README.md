# Figma REST API TypeScript Definitions

[Figma REST API](https://www.figma.com/developers/api) JSON responses TypeScript
definitions.

These definitions are generated from Rust definitions using
[1Password's typeshare](https://github.com/1Password/typeshare). The definitions
are liable to miss fields that haven't been needed by the Rust project. But the
Rust programs will error if a field has a value outside of the declared type.
You can use the `echo` subcommand of the parent project to see the data as
constrained by the schema.

[NPM](https://www.npmjs.com/package/figma-rest-api-typescript-definitions)

`Deno`

```typescript
import * as figmaTypes from "https://raw.githubusercontent.com/ccouzens/figma-rust/typescript-0.1.0/typescript/index.d.ts";
```
