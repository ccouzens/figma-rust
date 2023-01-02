# Design tokens subcommand

```bash
cargo run -- design-tokens < example-figma-files/design-tokens-for-figma.json
```

A design tokens generator inspired by the legacy output of the
[design-tokens Figma plugin](https://github.com/lukasoppermann/design-tokens).

The sub command is not recommended for use as it is missing large amounts of
functionality when compared to the plugin.

Compare the goal output with the sub command's output:

```bash
# vscode
code --diff src/design_tokens/{design-tokens-goal.tokens.json,example-output.json}

# jq and diff
diff -y --color=always <(jq --sort-keys < src/design_tokens/design-tokens-goal.tokens.json) <(jq --sort-keys < src/design_tokens/example-output.json) | less -R

# jq and diff focussing on a particular type
diff -y --color=always <(jq --sort-keys .motion < src/design_tokens/design-tokens-goal.tokens.json) <(jq --sort-keys .motion < src/design_tokens/example-output.json) | less -R
```

<table>
    <thead>
        <tr><th>Feature</th><th>Problems</th></tr>
    </thead>
    <tbody>
        <tr>
            <td>General</td>
            <td><ul>
            <li>Numbers are often expressed with decimals. Not a problem but shows in the diff.</li>
            <li>Sort order of properties are sometimes different. Not a problem but shows in the diff unless <code>--sort-keys</code> is used.</li>
            </ul></td>
        </tr>
        <tr>
            <td>Sizes</td>
            <td>No known issues</td>
        </tr>
        <tr>
            <td>Breakpoints</td>
            <td>No known issues</td>
        </tr>
        <tr>
            <td>Spacing</td>
            <td>No known issues</td>
        </tr>
        <tr>
            <td>Borders</td>
            <td><ul>
            <li>Dash pattern isn't outputted (not present in the Figma API).</li>
            <li>Ignores <code>unsupported</code> example (but is this a problem?)</li>
            </ul></td>
        </tr>
        <tr>
            <td>Radii</td>
            <td>Smoothing isn't outputted (not present in the Figma API).</td>
        </tr>
        <tr>
            <td>Motions</td>
            <td><ul>
            <li>Direction properties aren't outputted.</li>
            <li><code>.type.value</code> is hardcoded and often incorrect.</li>
            <li><code>custom-spring</code> and <code>custom-cubic-bezier</code> values are hardcoded and often incorrect (not present in the Figma API).</li>
            <li>Ignores <code>instant</code> example (but is this a problem?)</li>
            </ul></td>
        </tr>
        <tr>
            <td>Opacities</td>
            <td>No known issues</td>
        </tr>
        <tr>
            <td>Gradients</td>
            <td>Tokens are categorised as colors instead of gradients.</td>
        </tr>
        <tr>
            <td>Colors</td>
            <td><ul>
            <li>Incorrectly contains gradient tokens.</li>
            <li>Color values aren't outputted (not present in the Figma API)!</li>
            </ul></td>
        </tr>
        <tr>
            <td>Grid</td>
            <td><ul>
            <li>Doesn't ignore invalid no-grid example.</li>
            <li>Grid values aren't outputted (not present in the Figma API)!</li>
            </ul></td>
        </tr>
        <tr>
            <td>Font</td>
            <td>Grid values aren't outputted (not present in the Figma API)!</td>
        </tr>
        <tr>
            <td>Effect</td>
            <td><ul>
            <li>Doesn't ignore invalid none example.</li>
            <li>Effect values aren't outputted (not present in the Figma API)!</li>
            </ul></td>
        </tr>
    </tbody>
</table>
