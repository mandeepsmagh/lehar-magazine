# Build the home page from metadata.json. Run: make
# The newest issue goes first. The order comes from the PDF file name, which holds the issue date.
# The Lehar source repo adds each issue with its "make publish".
index.html: metadata.json index.template.html
	jq '.issues |= (sort_by(.pdf) | reverse) | .latest = .issues[0]' metadata.json > .page.json
	pandoc -f markdown -t html --wrap=none --template index.template.html --metadata-file .page.json -o $@ /dev/null
	rm .page.json
