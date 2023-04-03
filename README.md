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
* Diesel-cli

## Setup
* Clone the repository
* Create `.env` file with the following environmental variables:
    * DATABASE_URL=postgres://christopherallison:12345@localhost/people_data_api?sslmode=disable
    * SECRET_KEY=32CHARSECRETKEY
    * PASSWORD_SECRET_KEY=32CHARSECRETKEY
    * JWT_SECRET_KEY=32CHARSECRETKEY
    * ADMIN_EMAIL=some_admin@email.com 
    * ADMIN_PASSWORD=ADMINPASSWORD
    * ADMIN_NAME="Admin Name"
* Change APP_NAME const in lib.rs to your app
* `diesel migration run`
* `cargo run`


### Running on MacOS

```bash
cargo install diesel_cli --no-default-features --features postgres # if not already installed

# MacOS only clean up when done
brew install libpq
brew link --force libpq

cargo clean

docker compose down; sleep 2; docker compose up -d db; sleep 2; diesel migration run
docker compose logs -f

time docker compose build people-data-api
docker compose up
```

## TODO

- [x] Working: Dockerfile.simple again: worked (4.25GB)
- [x] Working: Dockerfile.new finally working with base rust-image (1.98GB)
- [x] Working: Dockerfile.new try debian:buster (444MB)
- [x] Working: Dockerfile.new try debian:buster-slim : (392MB)
- [x] Try again on codespaces


- [ ] replace openssl with libssl-dev in Dockerfile.simple
- [ ] Add e2e tests
- [x] progress indication with logging function
- [ ] Where shall we run diesel migrations?
