.DEFAULT_GOAL := all

example-figma-files = \
example-figma-files/design-tokens-for-figma.json \
example-figma-files/gov-uk-design-system.json \
example-figma-files/gov-uk-design-system-button.json

example-output-files = \
src/design_tokens/example-output.json \
src/typescript_props/example-output.ts

definition-files = \
definitions.kt \
definitions.swift \
definitions.ts

example-figma-files/design-tokens-for-figma.json :
	curl -sH "X-Figma-Token: ${FIGMA_TOKEN}" \
    'https://api.figma.com/v1/files/2MQ759R5kJtzQn4qSHuqR7' \
	| jq > $@

# My unchanged copy of https://www.figma.com/community/file/946837271092540314
example-figma-files/gov-uk-design-system.json :
	curl -sH "X-Figma-Token: ${FIGMA_TOKEN}" \
    'https://api.figma.com/v1/files/x0ptBZeKChJWD4rOaOf4fs' \
	| jq > $@

example-figma-files/gov-uk-design-system-button.json : example-figma-files/gov-uk-design-system.json
	jq '.document.children[] | select(.name == "🗝️  Styles and Components").children[] | select(.name == "Button")' < $< > $@

src/design_tokens/example-output.json : example-figma-files/design-tokens-for-figma.json
	cargo run --release -- design-tokens < $< > $@

src/typescript_props/example-output.ts : example-figma-files/gov-uk-design-system.json
	cargo run --release -- typescript-props < $< > $@

definitions.kt :
	typeshare . --lang=kotlin --output-file=$@

definitions.swift :
	typeshare . --lang=swift --output-file=$@

definitions.ts :
	typeshare . --lang=typescript --output-file=$@

.PHONY : all
all : $(example-figma-files) $(example-output-files) $(definition-files)

.PHONY : clean
clean : cleanOutput cleanDownloads cleanDefinitions

.PHONY : cleanDownloads
cleanDownloads :
	rm -f $(example-output-files)

.PHONY : cleanOutput
cleanOutput :
	rm -f $(example-output-files)

.PHONY : cleanDefinitions
cleanDefinitions :
	rm -f $(definition-files)

.PHONY : cleanAll
cleanAll : clean
	git clean -fXd
