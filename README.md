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
