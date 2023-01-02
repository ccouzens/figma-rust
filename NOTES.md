# Notes

## Document structure

To get the structure of the document with most properties hidden. Useful to
quickly explore the structure.

```bash
jq '.document | walk(
    if type == "object" then
        {name, type} + if has("children") then
            {children}
        else
            {}
        end + if has("characters") then
            {characters}
        else
            {}
        end
    else
        .
    end
)' -C < example-figma-files/design-tokens-for-figma.json | less -R
```
