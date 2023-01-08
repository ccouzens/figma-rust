# Component types

Generate TypeScript types to use as props for your React (or other) components.
Generate `const` values for looping through the possible values for
[Storybook](https://storybook.js.org/).

## Example output discussed

[example](./example-output.ts)

A single nested type is generated. You need to use
[Indexed Access](https://www.typescriptlang.org/docs/handbook/2/indexed-access-types.html)
to get the component you need. For example:

```typescript
import React from 'react';
import type { GOVUKDesignSystemCommunityTypes } from './gov-uk-design-system-interfaces.ts';

type TagProps = GOVUKDesignSystemCommunityTypes["🗝️  Styles and Components"]["Tag"];

export const Tag: React.FC<TagProps> = ({
  children,
  colour,
}) => (
};
```

Figma doesn't define behaviour, so additional props may be required for
interactive elements. Additional props may also be required to support
accessibility. For example:

```typescript
import React from 'react';
import type { GOVUKDesignSystemCommunityTypes } from './gov-uk-design-system-interfaces.ts';

interface ButtonProps extends GOVUKDesignSystemCommunityTypes["🗝️  Styles and Components"]["Button"] {
    onClick: () => void;
};

export const Button: React.FC<TagProps> = (props) => (
};
```

Some props may be better handled in CSS rather than JavaScript, so may not be
wanted in your TypeScript interface. For example `Hover` and `Focus` from
`Button`. Further extending the button example:

```typescript
import React from 'react';
import type { GOVUKDesignSystemCommunityTypes } from './gov-uk-design-system-interfaces.ts';

interface ButtonProps extends Omit<
    GOVUKDesignSystemCommunityTypes["🗝️  Styles and Components"]["Button"],
    'hover' | 'focus'
> {
    onClick: () => void;
};

export const Button: React.FC<TagProps> = (props) => (
};
```

## Formatting to your project's styleguide

The output can be piped through a formatter before saving to disk. For example

```bash
cargo run -- component-interfaces < example-figma-files/gov-uk-design-system.json | npx prettier --parser typescript > src/component_interfaces/gov-uk-design-system-interfaces.ts
```
