# 🦀 Rust microservice skeleton

![Version - 1.0.0-rc1](https://img.shields.io/static/v1?label=Version&message=1.0.0-rc1&color=blue&style=for-the-badge)
![Build - Passing](https://img.shields.io/static/v1?label=Build&message=Passing&color=brightgreen&style=for-the-badge)
![Rust version - 1.65.0](https://img.shields.io/static/v1?label=Rust+version&message=1.65.0&color=orange&style=for-the-badge)

This template repository is intended to be a base of project development for micro-services or APIs based on CRUD and simple APIs.

> **Caution:**
>
> This code skeleton is currently in a `release-candidate` version, meaning that it is not guaranteed to be 100% stable. However it is guaranteed to pass compilation.

# Features
* Based on [Rocket](https://rocket.rs/) latest version.
* Lightning fast API and console commands.
* Can run as an API as well as CLI commands.
* Automated CRUD code generation.
* Easy to use fine-tuned security.
* simple MVC-oriented design, with controllers, middleware, etc.

And a lot of features you know and love from non crab friendly languages and frameworks !

# Table of content
* [prerequisites](#prerequisites)
* [getting started](#getting-started)
* [first steps](#first-steps)
* [the skeleton](#the-skeleton)
* [generating a CRUD](#generating-a-crud)
* [creating a command](#creating-a-command)
* [deployment](#deployment)
* [future versions & roadmap](#future-versions--roadmap)
* [changelog](#changelog)
* [license](#license)

# Prerequisites
To run this app skeleton, you need to:
* **have rust installed & setup.** *You can refer to [this page](https://github.com/AdrienGras/presentation-rust-2022#les-ressources) to setup your rust environment.*
* **install PostgreSQL database driver in your system.** *As this application use source linking and PostgreSQL as a database, you must have PostgreSQL drivers installed on your system. You can use [this page](https://github.com/diesel-rs/diesel/blob/master/guide_drafts/backend_installation.md) to help you with this step.*
* **install the diesel CLI.** *This application uses Diesel as it's ORM, so you must use the diesel CLI to handle database migration. You can learn how to install diesel CLI on [this page](https://diesel.rs/guides/getting-started).*

# Getting started

> For this chapter and onwards, the documentation will assume you have a `linux` or `'nix` system, and thus give command for those systems.
>
> A dockerized version of this skeleton is planned to arrive in the `1.0.0-rc2` version, and will ease development and deployment phases.

First, clone this repository :

```bash
git clone git@github.com:owlnext-fr/rust-microservice-skeleton.git
cd rust-microservice-skeleton
```

Then launch the dev database :
```bash
docker-compose up --build -d
```

This will instanciate a PostgreSQL database for development purposes only. **Do not use this container in production environment.**

> **Note:**
>
> The future dockerization of this project will make the development database obsolete.

And now launch your IDE in the project directory :
```bash
code .
```

# First steps
This skeleton is inteded to use with two modes :
* `launch` that will launch the API server.
* `console` that will execute a given command.

This is made to allow ease of development using the same codebase for API development and console command (e.g. async runners, crons, etc.).

Before using this code skeleton, let's create a `.env.local` file to store all your application configuration.

```text
ROCKET_LOG_LEVEL=debug
DATABASE_URL=postgres://postgres:changeme@localhost/api
RUST_BACKTRACE=1

APP_DATABASE_URL="postgres://postgres:changeme@localhost/api"
APP_JWT_TTL=3600
APP_JWT_REFRESH_TTL=86400
APP_UPLOAD_DIR="rust-microservice-skeleton/upload"
APP_PACKAGE_NAME="rust-microservice-skeleton"
APP_PACKAGE_VERSION="1.0.0"
```

Now that your app is ready, let's create you your first user.

First, run the command to create an API user :

```bash
cargo run -- console app:create-account ""
```

This command will guide you through the creation of your first `Account`, `Application` and `User`.

These are organization unit to split users and applications data. Let's say for now that an `Account` can have multiple `Application`, and an `Application` can have multiple users.
All data from an `Account` is separated for another, same for the `Application`.

> **Note:**
>
> You can make your user an `administrator` by using the `app:promote-user` console command.

So, now that you have a user to play with your API, let's fire up this server shall we ?!

```bash
cargo run -- launch
```

You should see that your API is up and running. Under the hood it has populated the database, generated certificates for JWT authentication and some other configuration things.

Now let's authenticate as our new user:

```bash
curl --request POST \
  --url http://localhost:8000/api/auth/token \
  --header 'Content-Type: application/json' \
  --data '{
	"login": "<your login>",
	"password": "<your password>"
}
'
```

Yay ! you should have a JWT token. Now use it to do your first request on the API:

```bash
curl --request GET \
  --url 'http://localhost:8000/api/users?page=1&per_page=25' \
  --header 'Authorization: Bearer <access_token>'
```

# The skeleton
Now lets examine all the directory of this application skeleton:

* `command` is to store all your commands invokable by the `console` mode.
* `controllers` is to create all your application controllers.
* `core` have all the core modules to let you create applications with this skeleton. **Modify it at your own risks !**.
* `domain/dto` contains all input and output models (e.g. DTO) for your API.
* `domain/model` contains all your model representation.
* `domain/repository` contains all your data model repository (e.g. database request layers)
* `exceptions` stores all the custom logic exception of your application.
* `fixtures` stores all your data fixtures.
* `libraires` will contain all your external integrations, such as libraries, connectors, external APIs, etc.
* `middlewares` contains all your model and transitive middlewares.
* `migrations` contains all database migrations handled by `diesel`.
* `security` contains all your security policies to access your API.
* `main.rs` is the main entrypoint of this skeleton. **Modify it at your own risks !**

# Generating a CRUD

Generating a crud is a two part operation:
1. We need to generate all the code required to CRUD operations and middlewares.
2. We need to create the database migration.

To create all the code, simply run:

```bash
cargo run -- make:generate-scaffold ""
```

It will :
* Generate the model structures: model, DTOs and data repository.
* Generate the middleware.
* Generate the security policy (e.g. security voter).
* Generate the controllers.
* Update the rocket builder in the `core` to register all your new code.

Now that you have all the code, let's generate the migration:

```bash
source .env && diesel migration generate
```

This will generate a new database migration that you can complete.

Once completed, you can activate the migration by running:

```bash
source .env && diesel migration run
```

> **Note:**
>
> Due to a current bug on diesel, you will have to perform manual replace in the whole `src/domain/schema.rs` file:
>
> * `created_by -> Int4,` -> `created_by -> Nullable<Int4>,`
> * `deleted_by -> Int4,` -> `deleted_by -> Nullable<Int4>,`
> * `roles -> Array<Nullable<Text>>,` -> `roles -> Array<Text>,`
>
> This bug will be patched in the version `1.0.0-rc2`.

Now, you should have all your database and model connected, you only need to fill your model, DTO and middleware and you are good to go with your CRUD !

# Creating a command
As stated previously, this skeleton offers the possibility to create console commands like the ones you used to setup your user.

To create one, simply create file in the `command/<domain>` folder, add it to the desired `mod.rs`, and create a struct implementing `ConsoleCommand`. Your struct should at least have a field for the `CronLogMiddleware`.

Then fill the required functions, and write the body of your command in the `do_run` function.

Now go to the `src/core/rocket_factory.rs` and locate the `ConsoleCommandRegistry` instantiation, under the `command registry initialization `, and add your command as all other are added.

Now run your command with:

```bash
cargo run -- console <command name> "<command args>"
```

> **Note:**
>
> Even if your command does not handle any arguments, the `""` are required after the command name.
>
> This bug will be addressed in a future version.

> **Note:**
>
> A command generator, similar as the CRUD generator, will be implemented in the version `1.0.0-rc2`.

# Deployment

**This is a `release-candidate` version and thus should not be used in production at this time.**

Furthermore, a dockerized and deployable version will be available starting version `1.0.0-rc2`.

# Future versions & roadmap

* Version 1.0.0-rc2
    * [FEATURE] Dockerized version.
    * [FEATURE] A makefile to automate all recurrent tasks.
    * [FEATURE] Automated console command generation.
    * [PATCH] The replacement bug in the `schema.rs`.
    * [FEATURE] add a .env example.
    * [PATCH] remove the .vscode folder.

* Future versions
    * [PATCH] Remove the `""` required by clap for the console command launch.

# Changelog

You can find the changelog in the [CHANGELOG.md](./CHANGELOG.md) file.

# License

This code is under [BSD 3-Clause license](./LICENSE)