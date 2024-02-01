# Scrum Poker Online

## Local setup

Install Dioxus CLI
```shell
cargo install dioxus-cli

npm install tailwindcss@latest
```

Run
```shell
npx tailwindcss@experimental -i ./input.css -o ./public/tailwind.css --watch
docker run --rm --pull always -p 8000:8000 surrealdb/surrealdb:v1.0.2 start --auth --user root --pass root
dx serve --hot-reload --platform desktop
```