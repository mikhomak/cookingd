# Requirements
* Rust (cargo)
* Docker
* sqlx cli (optional)

# Installation
1. Make sure that the docker with postgres is working (`. ./scripts/init-db.sh`)
2. Copy `.env.example` to `.env`
3. `cargo run`

# For deployment
* If there is a new query, run `cargo sqlx prepare` if you add any new `query!` form **sqlx**

# For development
* To add new migration for DB add `sqlx 2migrate add "name"`