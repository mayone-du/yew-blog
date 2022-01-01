#!/bin/bash
# GitHub Pagesでデプロイしているため、ビルドされたHTMLから読み込むファイルのパスを修正
cat ./dist/index.html | sed -e 's/"\//".\//g' -e "s/'\//'.\//g" >> ./dist/fixed.html && mv ./dist/fixed.html ./dist/index.html