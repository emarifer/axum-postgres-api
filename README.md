# Axum Postgres REST API

### Server/REST API built using Rust's Axum framework and Postgres database running in a Docker container.

###### Besides the obvious prerequisite of having Rust and Docker installed, we need to do the following to create the database (2 Docker containers: one for the DB itself, and one for the DB administrator UI [pgAdmin]).

```
$ docker compose up -d  # run at the root of the project.
```

This will create 2 Docker containers that we will start with the command:

```
$ docker start postgres pgAdmin  # "docker stop postgres pgAdmin" to stop them.
```

About the use of pgAdmin (postgres graphical administrator) you can get more information [here][1] and [here][2].

---

> **NOTE**: Creating a new server, after having logged in with the credentials listed in the .env file, requires us to click "Add a new server" which will open a new window. In the "General" section we can give it the name we want. But the important thing is in the "Connection" tab. In the “Host name/address” section it is crucial that we add the same service name created in the docker-compose.yml file. Alternatively, we can use an address that we can obtain through the command:
>
> ```
> $ docker inspect postgres
> ```
>
> The value corresponding to “ipAddress” is the one that we can enter in this section, although this value could vary on each restart of the "postgres" container. So the best solution would be the one we have discussed in the first place.

---

###### To create the table inside the database that we added earlier using docker compose, we need to run database migrations with SQLx-CLI, which will need to be installed on our system.

###### Assuming that we already have the CLI installed, we execute the following command in the root of the project:

```

$ sqlx migrate add -r init

```

###### Now in the "migrations" folder we have two files. In the "up" file we will paste the following SQL command to create the "notes" table:

```

$ -- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE
IF NOT EXISTS notes (
id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
title VARCHAR(255) NOT NULL UNIQUE,
content TEXT NOT NULL,
category VARCHAR(100),
published BOOLEAN DEFAULT FALSE,
created_at TIMESTAMP
WITH
TIME ZONE DEFAULT NOW(),
updated_at TIMESTAMP
WITH
TIME ZONE DEFAULT NOW()
);

```

###### It is recommended to always include a script that can undo the changes made by the "up" script. To do this, navigate to the corresponding "down" script and add the following SQL code:

```

$ -- Add down migration script here

DROP TABLE IF EXISTS notes;

```

---

###### Now we can start our server, provided we have correctly set the environment variable of the .env file:

```

$ POSTGRES_HOST=127.0.0.1
POSTGRES_PORT=6500
POSTGRES_USER=admin
POSTGRES_PASSWORD=password123
POSTGRES_DB=rust_sqlx

DATABASE_URL=postgresql://admin:password123@localhost:6500/rust_sqlx?schema=public

PGADMIN_DEFAULT_EMAIL=admin@admin.com
PGADMIN_DEFAULT_PASSWORD=password123

```

To run the server (in development mode):

```bash
$ cargo run # or cargo watch -q -c -w src/ -x run (cargo-watch must be installed on the system)
```

The page will reload if you make edits (with watch mode) 😀.

To build the project for production and minimize its size:

```bash
$ cargo build --release
```

Runs the app in the development mode.<br>
Open [http://localhost:8080/api/healthchecker](http://localhost:8080/api/healthchecker) to view it in the browser.

---

###### To avoid having to manually set the request URL and HTTP verbs, you can use the file found in the project root "Note_App.postman_collection.json" and import it into [Postman](https://www.postman.com/) or [Insomnia](https://insomnia.rest/).

###### Alternatively, you can use the cURL command to test the REST API. Running the following example commands, obviously with your own data, is a lightweight alternative to the 2 applications mentioned above:

```bash
$ curl -v -X POST http://localhost:8080/api/notes -d '{"title": "Creando nota con cURL", "content": "Funcionará ahora?", "category": "FastAPI"}' -H "content-type: application/json" | json_pp
```

```bash
$ curl -v http://localhost:8080/api/notes | json_pp
```

```bash
$ curl -v http://localhost:8080/api/notes?page=1\&limit=5 | json_pp
```

```bash
$ curl -v http://localhost:8080/api/notes/043d31a6-6b27-4814-9c64-00988f8e28af | json_pp # replace the id with one that is actually in your DB.
```

```bash
$ curl -v -X PATCH http://localhost:8080/api/notes/043d31a6-6b27-4814-9c64-00988f8e28af -d '{"title": "Editando nota con cURL", "content": "Todo está funcionando hasta el momento 😀😀", "category": "Custom FastAPI", "published" : true}' -H "content-type: application/json" | json_pp
```

```bash
$ curl -v -X DELETE http://localhost:8080/api/notes/1dc89584-6868-4c17-a21e-3f592617f917
```

[1]: https://anasdidi.dev/articles/200713-docker-compose-postgres/
[2]: https://www.youtube.com/watch?v=uKlRp6CqpDg
[//]: # "https://stackoverflow.com/questions/19985235/break-long-lines-in-markdown-code"
[//]: # "https://ajaxhispano.com/ask/comentarios-en-markdown-13816/"
