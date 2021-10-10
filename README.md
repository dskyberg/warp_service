# Rust Web Service with Warp and MongoDB

This is a learning sanbox to explore building full stack services in Rust, while
maybe actually doing someting useful.

This service is developed with Warp, Serde, and MongoDB.

The data persistence is managed via MongoDB.  The `services` crate provides an
abstraction level between the REST handlers and the database.  The handlers only
understand the data models as defined in the `models` crate.

## Extending the Service

### Step 1: Update the Model
The service leverages `serde_json` for all request and response data.  Updating
and extending the model starts with defining a `struct` for each request body
and each response in [src/models](./src/models).  If an API accepts data via query params (which I strongly
suggest avoiding - use post!!), the query params can be modeled via a `struct`
as well.

### Step 2: Add the Routes
The routes are defined in [src/routes](./src/routes). You can bundle the routes in
separate modules by adding additional files in this folder.  Or just add more
routes in [routes/request.rs](./src/routes/request.rs)

### Step 3: Add the Handlers
The handlers are defined in [src/handlers].  Each handler sbould be defined to
accept the `service` parameter (for access to the db client), and whatever body,
path params, and query params managed by the route.

### Step 4: Add a Macro for the Route + Handler
The macros are defined in [src/apis](./src/apis).  The macros are just sugar to
enable easily composing the routes in [main.rs](./src/main.rs).

### Step 5: Compose the Route in Main
The routes are served by the warp service defined in [main.rs](./src/main.rs). Just
add another `.or(<your macro>)` to the collection of routes.

## Setting up MongoDB
This service leverages MongoDB, using the new 100% Rust mongodb crate. Connection
pooling is built into the crate.  So, no need to fool with `r2d2` any more.

### Launch MongoDB

To launch mongo via docker, run:

````
> docker-compose up -d mongodb
````

This will create and run the mongo container, and link a local folder as a
volume.  Review the [docker-compose.yml](./docker-compose.yml) file for info.
Personally, I prefer using the Robo3T GUI for directly managing the Mongo instance
to create the database, and initializing data.

The MongoDB instance is initialized with the script in [mongodb_init](./mongodb_init/init.js).

Access the monodb shell with:

````bash
> docker exec -it mongodb mongo
````

### Manage the Env Settings
Copy the following in a `.env` file in the crate root (where your Cargo.toml lives):

````
MONGODB_URI=mongodb://127.0.0.1:27017
MONGODB_DATABASE=gnap
MONGODB_USER=me
MONGODB_PASSWORD=password
MONGODB_APP_NAME=gnap
REDIS_URI=redis://localhost
API_ADDRESS=127.0.0.1:8000
````

## Run

- Start Mongo:

````bash
> docker-compose up -d
````

- Start the server

````bash
> cargo run
````

The service will run on localhost:8000
