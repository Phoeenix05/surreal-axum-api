#//bin/zsh

redocly build-docs openapi.json -o docs/index.html

git add .
git commit -m $1
git push origin/master
