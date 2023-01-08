.DEFAULT_GOAL := all

example-figma-files = \
example-figma-files/design-tokens-for-figma.json \
example-figma-files/gov-uk-design-system.json

example-output-files = \
src/design_tokens/example-output.json \
src/component_interfaces/example-output.ts

example-figma-files/design-tokens-for-figma.json :
	curl -sH "X-Figma-Token: ${FIGMA_TOKEN}" \
    'https://api.figma.com/v1/files/2MQ759R5kJtzQn4qSHuqR7' \
	| jq > $@

# My unchanged copy of https://www.figma.com/community/file/946837271092540314
example-figma-files/gov-uk-design-system.json :
	curl -sH "X-Figma-Token: ${FIGMA_TOKEN}" \
    'https://api.figma.com/v1/files/VU1aDcxBJCKLw1e1vbFmur' \
	| jq > $@

src/design_tokens/example-output.json : example-figma-files/design-tokens-for-figma.json
	cargo run --release -- design-tokens < $< > $@

src/component_interfaces/example-output.ts : example-figma-files/gov-uk-design-system.json
	cargo run --release -- component-interfaces < $< > $@

.PHONY : all
all : $(example-figma-files) $(example-output-files)

.PHONY : clean
clean :
	rm -f $(example-figma-files) $(example-output-files)

.PHONY : cleanAll
cleanAll : clean
	git clean -fXd