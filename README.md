# Rust GNAP

This is a learning sanbox to explore building full stack services in Rust, while
maybe actually doing someting useful.  The goal is to support the GNAP protocol.

This service is developed with Warp, Serde, and MongoDB.

The data persistence is managed via cached (Redis) MongoDB.  The `services` crate
handles ensuring data is cached and provides an abstraction level between the
REST handlers and the database.  The handlers only understand the data models as
defined in the `models` crate.

## Build

### Env Settings
Copy the following in a `.env` file in the crate root (where your Cargo.toml lives):

````
MONGODB_URI=mongodb://127.0.0.1:27017
MONGODB_DATABASE=gnap
MONGODB_USER=me
MONGODB_PASSWORD=password
MONGODB_APP_NAME=gnap
````

## Run

- Start Mongo:

````bash
> docker-compose up -d
````

- Start the server

````bash
> cargo +nightly run
````

The service will run on localhost:8000

Access the monodb shell with
````bash
> docker exec -it mongodb mongo
