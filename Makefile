# Build the home page from metadata.json. Run: make
# The newest issue goes first. The order comes from the PDF file name, which holds the issue date.
# Issues are grouped by year, newest year first. The year also comes from the PDF file name.
# The newest year is open and the older years are closed.
# A card shows the title without the year, because the year is the heading above it.
# The Lehar source repo adds each issue with its "make publish".
index.html: metadata.json index.template.html
	jq '.issues |= (sort_by(.pdf) | reverse | map(.card = (.title | sub(" [0-9]{4}$$"; "")))) | .latest = .issues[0] | .years = (.issues | group_by(.pdf | capture("lehar-(?<y>[0-9]{4})").y) | reverse | map({year: (.[0].pdf | capture("lehar-(?<y>[0-9]{4})").y), count: length, issues: .}) | to_entries | map(.value + {open: (.key == 0)}))' metadata.json > .page.json
	pandoc -f markdown -t html --wrap=none --template index.template.html --metadata-file .page.json -o $@ /dev/null
	rm .page.json
