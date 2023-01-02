.DEFAULT_GOAL := all

example-figma-files = \
example-figma-files/design-tokens-for-figma.json

example-output-files = \
src/design_tokens/example-output.json

# https://www.figma.com/file/2MQ759R5kJtzQn4qSHuqR7/Design-Tokens-for-Figma
example-figma-files/design-tokens-for-figma.json :
	curl -sH "X-Figma-Token: ${FIGMA_TOKEN}" \
    'https://api.figma.com/v1/files/2MQ759R5kJtzQn4qSHuqR7' \
	| jq > $@

src/design_tokens/example-output.json : example-figma-files/design-tokens-for-figma.json
	cargo run -- design-tokens < $< > $@

.PHONY : all
all : $(example-figma-files) $(example-output-files)

.PHONY : clean
clean :
	rm -f $(example-figma-files) $(example-output-files)

.PHONY : cleanAll
cleanAll : clean
	git clean -fXd