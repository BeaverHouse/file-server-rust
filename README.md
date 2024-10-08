<p align="center">
  <a href="https://github.com/BeaverHouse/file-server-rust">
    <img src="logo.png" alt="Logo">
  </a>

  <p align="center">
    Personal file Server made by Rust
    <br>
    <br>
    <a href="https://github.com/BeaverHouse/file-server-rust/issues">Bug Report</a>
    |
    <a href="https://github.com/BeaverHouse/file-server-rust/issues">Request to HU-Lee</a>
  </p>

  <p align="center">
    <a href="https://www.rust-lang.org">
      <img src="https://img.shields.io/badge/Rust-000?logo=rust&logoColor=fff&style=flat" alt="Rust">
    </a>
    <a href="https://actix.rs">
      <img src="https://img.shields.io/badge/Actix-000?logo=actix&logoColor=fff&style=flat" alt="Actix Web">
    </a>
    <a href="./LICENSE">
      <img src="https://img.shields.io/github/license/BeaverHouse/file-server-rust" alt="License">
    </a>
  </p>
</p>

<!-- Content -->

<br>

## Description

Personal file Server made by [Rust].  
Used [Actix Web] & [utoipa] for web framework and Swagger UI.

[Rust]: https://www.rust-lang.org
[Actix Web]: https://actix.rs
[utoipa]: https://github.com/juhaku/utoipa

<br>

## On local development

First, set up the environment variables.  
You need to change the variables if you do not use pre-authenticated request or use other storages like AWS S3,

```
PG__URL=<Your PostgreSQL connection string>

ORACLE_FAMILY_RW_ENDPOINT=<Your Oracle Object Storage pre-authenticated request URL>
ORACLE_AECHECK_W_ENDPOINT=<Your Oracle Object Storage pre-authenticated request URL>

API_KEY=RSF202410@$
```

Then run this command in the terminal:

```bash
cargo run
```

Or, set up [auto-reloading] and run:

```bash
cargo watch -x run
```

[auto-reloading]: https://actix.rs/docs/autoreload

<br>

## References

|      **Description**       |               **References**                |
| :------------------------: | :-----------------------------------------: |
|  Actix Web official docs   |            https://actix.rs/docs            |
| Schema import for `utoipa` | https://github.com/juhaku/utoipa/issues/894 |

<br>

## Contributing

See the [CONTRIBUTING.md][contributing].

[contributing]: ./CONTRIBUTING.md
