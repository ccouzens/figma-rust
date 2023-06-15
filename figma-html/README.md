# Figma REST API to HTML and CSS toolkit

Toolkit for building automation for converting Figma files to HTML and CSS.

There are many challenges to converting Figma files to HTML and CSS in a generic
manner. For example:

- Different ways of representing the CSS in the output (inline, stylesheet,
  JavaScript solution)
- Different ways of representing the HTML in the output (split files, monolithic
  file, JavaScript component solution)
- How pseudo classes (hover, focus, etc) are represented within the Figma file
  and how they should be reprented in the output.
- Bespoke patterns in the Figma files, such as size tokens.

Rather than trying to build a single tool to cover all scenarios, I believe an
extendable toolkit to be more sensible.

Currently a Rust library, but will also be a TypeScript library.
