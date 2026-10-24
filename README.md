# lehar-magazine

The published issues of Lehar, a Punjabi magazine. This repository holds only finished files. The source of the magazine is in a private repository.

- `issues/` holds one PDF and one cover image for each issue.
- `metadata.json` lists the issues.
- `index.html` is the home page. It is made from `metadata.json` and `index.template.html` with `make` (needs `jq` and `pandoc`).
- A push to `main` deploys `index.html`, `assets/` and `issues/` to GitHub Pages. Nothing else is published.

To add an issue, run `make publish` in the source repository. It copies the PDF and the cover here and updates `metadata.json` and `index.html`. Then check the page, commit and push.
