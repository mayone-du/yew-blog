# Yew Blog

## Build

```
npm run build:talwindcss
```

```
trunk serve
```

download the schema.docs.graphql from github.
https://docs.github.com/ja/graphql/overview/public-schema

## TODO

- GitHub Pages で初回にルートページ以外にアクセスすると 404 になるのを修正（ファイルシステムが適用されているため仕方ない。404.html などを作成するか、ルーティングまわりをみなおす。）
- URL ダサいから GitHub Pages やめてサーバー借りるのあり
- いいねボタン的なのつけて、GitHub API つかってシンプルにカウント増やしてく

## DONE!

- ルーティング
- レイアウトの共通化
- Tailwind CSS の導入
- GitHub Actions で GitHub Pages へ自動デプロイ
