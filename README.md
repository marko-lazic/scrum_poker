# Scrum Poker
A simple to use scrum poker game built in Rust. Free and Open Source Forever!

## Local setup
Install Dioxus CLI
```shell
cargo install dioxus-cli
```
Install Tailwind CSS
```shell
npm install tailwindcss@latest
```
You will also need Docker to run the databse.

## Run
Start database
```shell
docker run --rm --pull always -p 8000:8000 surrealdb/surrealdb:v1.0.2 start --auth --user root --pass root
```

Start CSS CLI tool
```shell
npx tailwindcss@experimental -i ./input.css -o ./public/tailwind.css --watch
```

Start the server
```shell
dx serve --hot-reload --platform desktop
```