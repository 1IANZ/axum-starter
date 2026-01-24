# axum-starter

1. clone the repository

```bash
git clone https://github.com/1IANZ/axum-starter.git

```

2. install sea-orm-cli (optional)

```bash
cargo install sea-orm-cli
```

3. create `.env` file

```bash
DATABASE_URL=postgres://user:password@localhost:5432/db_name
```

4. use `sea-orm-cli` to generate entity

```bash
   sea-orm-cli generate entity -s demo --with-serde both --model-extra-attributes 'serde(rename_all ="camelCase")' --date-time-crate chrono  -o ./src/entity
```

5. run the server

```bash
   cargo run
```
