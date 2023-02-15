# mining-pool-api

<br/>
<p align="center">
<img src="img/pool.png" >
</a>
</p>
<br/>

## postgres set-up

1. `docker run --name postgres-db -e POSTGRES_PASSWORD=docker -p 5432:5432 -d postgres`

2. `sudo apt install libpq-dev`

3. `cargo install diesel_cli --no-default-features --features postgres`

4. `export DATABASE_URL="postgres://postgres:pass@localhost/pool-db"`

5. `cargo setup`

6. `diesel migration generate mining_pool_ap`
