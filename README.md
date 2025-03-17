## Description

URL Shortening Service

Usage:

| Description | Endpoint |
| ----------- | ----------- |
| Health | GET `/health` |
| Shorten a long URL | POST `/shorten` `{ url: <Long URL>}` |
| Access a shortened URL | GET `/<code>` |
| Get info about a shortened URL | GET `/shorten/<code>` |
| Get stats for a shortened URL | GET `/shorten/<code>/stats` |
| Shorten a long URL | PUT `/shorten/<code>` `{ url: <New Long URL>}` |
| Delete a shortened URL | DELETE `/shorten/<code>` |

Examples:
```bash
curl -i -X POST localhost:3000/shorten
-H "Content-Type: application/json"
-d
{
  "url": "https://www.typescriptlang.org/docs/handbook/decorators.html#method-decorators"
}

curl -i -X GET localhost:3000/shorten/wouftw
curl -i -X GET localhost:3000/shorten/wouftw/stats

curl -i -X DELETE localhost:3000/shorten/wouftw
```

## Project Setup
Start the Postgres container:

```bash
docker-compose -f docker-compose.yml up
```

Run the migrations (using [dbmate](https://github.com/amacneil/dbmate))

```bash
dbmate up
```

Start the app:

```bash
cargo run
```

## Tests

```bash
cargo test
```

## Misc / development
Validating urls has some intricacies such as choosing a regex (there are many),
choosing which schemes to consider (in our case only http/https).
So, using a simplified validation with url crate feels good enough.

Inside `url-shortener` folder, run:
```
cornucopia live $(cat ../.env | grep DATABASE_URL | sed 's/DATABASE_URL=//' | sed 's/"//g')
```
