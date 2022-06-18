## requirements

install `diesel_cli`:

```
$ cargo install diesel_cli
```

start mysql:

```
$ docker compose up --build -d
```

## exaple of mysql access with Diesel

```
echo DATABASE_URL=mysql://root:password@0.0.0.0/diesel_demo > .env
$ diesel setup
$ diesel migration run
$ cargo run --bin example_mysql
```
