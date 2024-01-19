# Feedback app

## Requirements

 - Docker

## Lauch

- Launch the database with `docker compose up --build`
- Open [locahost:8080](http://localhost:8080)

## Reset

- `Ctrl+C` to close docker
- `docker compose down`
- Find the name of the volume:  `docker volume ls` 
- Remove volume `docker volume rm [name]`
- Then relaunch