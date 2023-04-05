# People Data Analytics

This app is a learning project and attempt to create a data-centric model and Graphql API of employee skills, capabilities, certifications and work over time.

- [ ] Model people and their roles on teams
- [ ] Model people's skills and validate them based on their work
- [ ] Model how teams fit into an org hierarchy
- [ ] Model organizational capacity and work in progress
- [ ] Time-series modelling of changes to the organization over time as people change roles, learn and evolve.

It also includes :

- [x] User models
- [x] Automated Admin Generation
- [x] Authentication and sign-in

## Dependencies

- Diesel-cli

## Setup

- Clone the repository
- Create `.env` file with the following environmental variables:
  - DATABASE_URL=postgres://christopherallison:12345@localhost/people_data_api?sslmode=disable
  - SECRET_KEY=32CHARSECRETKEY
  - PASSWORD_SECRET_KEY=32CHARSECRETKEY
  - JWT_SECRET_KEY=32CHARSECRETKEY
  - ADMIN_EMAIL=some_admin@email.com 
  - ADMIN_PASSWORD=ADMINPASSWORD
  - ADMIN_NAME="Admin Name"
- Change APP_NAME const in lib.rs to your app
- `diesel migration run`
- `cargo run`

## Dan's notes

### Running on MacOS

```bash
cargo install diesel_cli --no-default-features --features postgres # if not already installed

# MacOS only clean up when done
brew install libpq
brew link --force libpq

cargo clean

docker compose down; sleep 2; docker compose up -d db; sleep 10; diesel migration run
docker compose exec -it db psql -U christopherallison -W people_data_api
docker compose logs -f

time docker compose build people-data-api
docker images | grep epi
docker compose up
```

### WiP/TODO

- [x] Working: Dockerfile.simple again: worked (arm64:4.25GB)
- [x] Working: Dockerfile.slim finally working with base rust-image (arm64:1.98GB)
- [x] Working: Dockerfile.slim try debian:buster (arm64:444MB)
- [x] Working: Dockerfile.slim try debian:buster-slim : (arm64:392MB amd64:447MB )
- [x] Try again on codespaces (amd64)
  - [x] Rename Dockerfile.slim to Dockerfile.slim
- [x] Docker as non root user (rusty)

- [ ] Can I just copy src and Cargo.(toml|lock) into the image?
  - [ ] If so, Fix Dockerfile.simple as well
  - [ ] If so, remove the .dockerignore file
- [ ] Accelerate build with rust crate cache?
- [ ] Re-validate all dependencies in Dockerfile.slim
- [ ] alpine base image (musl)
  - [ ] [LogRocket Blog article](https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/)
    - [ ] [Associated Repo](https://github.com/zupzup/rust-docker-web/blob/main/debian/Dockerfile)
  - [ ] Working: Dockerfile.alpine (arm64: 1.98GB)
  - [ ] Working: Dockerfile.alpine (arm64: 1.98GB)
- [x] http response type for index.html Content-Type: text/html; charset=UTF-8
- [x] replace openssl with libssl-dev in Dockerfile.simple
- [ ] Add e2e tests
- [x] progress indication with logging function
- [ ] Where shall we run diesel migrations?
  - Can use docker-compose to wait for db startup, and run migrations to completion
  - Create a diesel container, with migrations folders mounted, and run migrations
  - https://stackoverflow.com/questions/35069027/docker-wait-for-postgresql-to-be-running
  - https://docs.docker.com/compose/startup-order/

### Container image sizes

You can control the image you build by selecting the Dockerfile.XX in the docker-compose.yml file.
| Image              | arm64  | amd64                    | description                                |
|--------------------|--------|--------------------------|--------------------------------------------|
| rust:1.68          | 4.25GB |                          | single stage (Dockerfile.simple)           |
| rust:1.68          | 1.98GB |                          | multi-stage                                |
| debian:buster      | 444MB  |                          | multi-stage                                |
| debian:buster-slim | 392MB  | 447MB (15 minutes build) | multi-stage (Dockerfile.slim)              |
| alpine:3.14        |        |                          | multi-stage musl based (Dockerfile.alpine) |

### Caching the build

To enable the caching of the compiling of the rust dependencies, you can modify the start of the Dockerfile.slim to add this:

```dockerfile
SNIPPET
```

*measured on M2 Mac Mini:*

| Image             | first | subsequent with only code change | description  |
|-------------------|-------|----------------------------------|--------------|
| Dockerfile.simple | 238s  |                                  | single stage |
| Dockerfile.slim   | 216s  |                                  | multi-stage  |
| Dockerfile.fast   |       |                                  | multi-stage  |
