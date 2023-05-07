.DEFAULT_GOAL := all

# My unchanged copy of https://www.figma.com/community/file/946837271092540314
gov-uk-figma-file = WwTlihidmAqp0bky272vd9

example-figma-files = \
example-figma-files/design-tokens-for-figma.json \
example-figma-files/gov-uk-design-system.json \
example-figma-files/gov-uk-design-system-components/get-started-page.svg \
example-figma-files/gov-uk-design-system-components/button.svg \
example-figma-files/gov-uk-design-system-components/cookie-banner.svg \
example-figma-files/gov-uk-design-system-components/footer.svg \
example-figma-files/gov-uk-design-system-components/header.svg \
example-figma-files/gov-uk-design-system-components/tag.svg

example-output-files = \
src/design_tokens/example-output.json \
src/typescript_props/example-output.ts \
example-figma-files/gov-uk-design-system-components/get-started-page.json \
example-figma-files/gov-uk-design-system-components/get-started-page.html \
example-figma-files/gov-uk-design-system-components/button.json \
example-figma-files/gov-uk-design-system-components/button.html \
example-figma-files/gov-uk-design-system-components/cookie-banner.json \
example-figma-files/gov-uk-design-system-components/cookie-banner.html \
example-figma-files/gov-uk-design-system-components/footer.json \
example-figma-files/gov-uk-design-system-components/footer.html \
example-figma-files/gov-uk-design-system-components/header.json \
example-figma-files/gov-uk-design-system-components/header.html \
example-figma-files/gov-uk-design-system-components/tag.json \
example-figma-files/gov-uk-design-system-components/tag.html

definition-files = \
definitions.kt \
definitions.swift \
typescript/index.d.ts

example-figma-files/design-tokens-for-figma.json :
	curl -sH "X-Figma-Token: ${FIGMA_TOKEN}" \
    'https://api.figma.com/v1/files/2MQ759R5kJtzQn4qSHuqR7' \
	| jq > $@

example-figma-files/gov-uk-design-system.json :
	curl -sH "X-Figma-Token: ${FIGMA_TOKEN}" \
	'https://api.figma.com/v1/files/$(gov-uk-figma-file)' \
        | jq > $@

example-figma-files/gov-uk-design-system-components/get-started-page.json : example-figma-files/gov-uk-design-system.json
	jq '.document.children[] | select(.name == "👋  Get Started")' < $< > $@

example-figma-files/gov-uk-design-system-components/button.json : example-figma-files/gov-uk-design-system.json
	jq '.document.children[] | select(.name == "🗝️  Styles and Components").children[] | select(.name == "Button")' < $< > $@

example-figma-files/gov-uk-design-system-components/cookie-banner.json : example-figma-files/gov-uk-design-system.json
	jq '.document.children[] | select(.name == "🗝️  Styles and Components").children[] | select(.name == "Cookie banner")' < $< > $@

example-figma-files/gov-uk-design-system-components/footer.json : example-figma-files/gov-uk-design-system.json
	jq '.document.children[] | select(.name == "🗝️  Styles and Components").children[] | select(.name == "Footer")' < $< > $@

example-figma-files/gov-uk-design-system-components/header.json : example-figma-files/gov-uk-design-system.json
	jq '.document.children[] | select(.name == "🗝️  Styles and Components").children[] | select(.name == "Header")' < $< > $@

example-figma-files/gov-uk-design-system-components/tag.json : example-figma-files/gov-uk-design-system.json
	jq '.document.children[] | select(.name == "🗝️  Styles and Components").children[] | select(.id == "147:17")' < $< > $@

example-figma-files/gov-uk-design-system-components/get-started-page.svg :
	curl -s $$(curl -sH "X-Figma-Token: ${FIGMA_TOKEN}" \
	'https://api.figma.com/v1/images/$(gov-uk-figma-file)?ids=756:127&format=svg&svg_include_id=true' \
        | jq '.images["756:127"]' -r) > $@

example-figma-files/gov-uk-design-system-components/button.svg :
	curl -s $$(curl -sH "X-Figma-Token: ${FIGMA_TOKEN}" \
	'https://api.figma.com/v1/images/$(gov-uk-figma-file)?ids=213:6&format=svg&svg_include_id=true' \
        | jq '.images["213:6"]' -r) > $@

example-figma-files/gov-uk-design-system-components/cookie-banner.svg :
	curl -s $$(curl -sH "X-Figma-Token: ${FIGMA_TOKEN}" \
	'https://api.figma.com/v1/images/$(gov-uk-figma-file)?ids=18330:13859&format=svg&svg_include_id=true' \
        | jq '.images["18330:13859"]' -r) > $@

example-figma-files/gov-uk-design-system-components/footer.svg :
	curl -s $$(curl -sH "X-Figma-Token: ${FIGMA_TOKEN}" \
	'https://api.figma.com/v1/images/$(gov-uk-figma-file)?ids=19792:14489&format=svg&svg_include_id=true' \
        | jq '.images["19792:14489"]' -r) > $@

example-figma-files/gov-uk-design-system-components/header.svg :
	curl -s $$(curl -sH "X-Figma-Token: ${FIGMA_TOKEN}" \
	'https://api.figma.com/v1/images/$(gov-uk-figma-file)?ids=20226:12488&format=svg&svg_include_id=true' \
        | jq '.images["20226:12488"]' -r) > $@

example-figma-files/gov-uk-design-system-components/tag.svg :
	curl -s $$(curl -sH "X-Figma-Token: ${FIGMA_TOKEN}" \
	'https://api.figma.com/v1/images/$(gov-uk-figma-file)?ids=147:17&format=svg&svg_include_id=true' \
        | jq '.images["147:17"]' -r) > $@

src/design_tokens/example-output.json : example-figma-files/design-tokens-for-figma.json
	cargo run --release -- design-tokens < $< > $@

src/typescript_props/example-output.ts : example-figma-files/gov-uk-design-system.json
	cargo run --release -- typescript-props < $< > $@

example-figma-files/gov-uk-design-system-components/get-started-page.html : example-figma-files/gov-uk-design-system.json
	cargo run --release -- to-html 756:127 < $< \
		| sed 's/px GDS Transport Website/px GDS Transport Website,arial,sans-serif/g' \
		| npx prettier@2.8.4 --parser html > $@

example-figma-files/gov-uk-design-system-components/button.html : example-figma-files/gov-uk-design-system.json
	cargo run --release -- to-html 213:6 < $< \
		| sed 's/px GDS Transport Website/px GDS Transport Website,arial,sans-serif/g' \
		| npx prettier@2.8.4 --parser html > $@

example-figma-files/gov-uk-design-system-components/cookie-banner.html : example-figma-files/gov-uk-design-system.json
	cargo run --release -- to-html 18330:13859 < $< \
		| sed 's/px GDS Transport Website/px GDS Transport Website,arial,sans-serif/g' \
		| npx prettier@2.8.4 --parser html > $@

example-figma-files/gov-uk-design-system-components/footer.html : example-figma-files/gov-uk-design-system.json
	cargo run --release -- to-html 19792:14489 < $< \
		| sed 's/px GDS Transport Website/px GDS Transport Website,arial,sans-serif/g' \
		| npx prettier@2.8.4 --parser html > $@

example-figma-files/gov-uk-design-system-components/header.html : example-figma-files/gov-uk-design-system.json
	cargo run --release -- to-html 20226:12488 < $< \
		| sed 's/px GDS Transport Website/px GDS Transport Website,arial,sans-serif/g' \
		| npx prettier@2.8.4 --parser html > $@

example-figma-files/gov-uk-design-system-components/tag.html : example-figma-files/gov-uk-design-system.json
	cargo run --release -- to-html 147:17 < $< \
		| sed 's/px GDS Transport Website/px GDS Transport Website,arial,sans-serif/g' \
		| npx prettier@2.8.4 --parser html > $@

definitions.kt :
	typeshare . --lang=kotlin --output-file=$@

definitions.swift :
	typeshare . --lang=swift --output-file=$@

typescript/index.d.ts :
	typeshare . --lang=typescript --output-file=$@

.PHONY : all
all : $(example-figma-files) $(example-output-files) $(definition-files)

.PHONY : clean
clean : cleanOutput cleanDownloads cleanDefinitions

.PHONY : cleanDownloads
cleanDownloads :
	rm -f $(example-figma-files)

.PHONY : cleanOutput
cleanOutput :
	rm -f $(example-output-files)

.PHONY : cleanDefinitions
cleanDefinitions :
	rm -f $(definition-files)

.PHONY : cleanAll
cleanAll : clean
	git clean -fXd
