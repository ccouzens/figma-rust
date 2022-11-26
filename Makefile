# https://www.figma.com/file/2MQ759R5kJtzQn4qSHuqR7/Design-Tokens-for-Figma
example-file.json :
	curl -sH "X-Figma-Token: ${FIGMA_TOKEN}" \
    'https://api.figma.com/v1/files/2MQ759R5kJtzQn4qSHuqR7' \
	| jq > $@

.PHONY : all
all : example-file.json

.PHONY : clean
clean :
	rm -f example-file.json

.PHONY : cleanAll
cleanAll : clean
	git clean -fXd