# Scrum Poker
A simple to use scrum poker game built in Rust. Free and Open Source Forever!

## Local Setup
Install Dioxus CLI:
```shell
cargo install dioxus-cli
```
Install Tailwind CSS:
```shell
npm install tailwindcss@latest
```
You will also need Docker to run the databse.

## Run Local And Docker Environment
Start database:
```shell
docker run --rm --name scrumpokerdb -p 8000:8000 surrealdb/surrealdb:v1.2.1 start --auth --user root --pass root
```

### To strart locally type following:

Start CSS CLI tool:
```shell
npx tailwindcss -i ./input.css -o ./public/tailwind.css --watch
```

Start the server:
```shell
dx serve --hot-reload --platform desktop
```

### You can also build scrumpoker with docker
Build scrumpoker Dockerfile:
```shell
docker build -t scrumpoker:latest .
```

To run scrumpoker in docker check `docker-compose.yml` file. You can build the image with --build flag:

```shell
docker compose up --build
```

List of scrumpoker environment variables:

| Variable     | Default             |
|--------------|---------------------|
| HOST_ADDRESS | 127.0.0.1:3030      |
| WS_ADDRESS   | ws://127.0.0.1:3030 |
| DB_ADDRESS   | ws://localhost:8080 |
| DB_USERNAME  | root                |
| DB_PASSWORD  | root                |
| DB_NS        | scrumpokerdb        |
| DB_NAME      | Text                |

## Deploy to Fly.io

To deploy database change directory to `database` directory.
```shell
fly launch
fly volumes create data --region otp --size 1
fly secrets set SURREAL_USER=
fly secrets set SURREAL_PASS=
```

Deploy scrumpoker app fromm project root directory.
```shell
fly launch
fly secrets set DB_ADDRESS=wss://scrumpokerdb.fly.dev
fly secrets set DB_USERNAME=
fly secrets set DB_PASSWORD=
fly secrets set HOST_ADDRESS=0.0.0.0:3030 
fly secrets set WS_ADDRESS=wss://scrumpoker.fly.dev
fly deploy
```