# The book - Server

## Setup:

```sh
docker compose up -d
docker cp ./seeds the_book_db:/home/seeds
docker exec -it the_book_db /usr/bin/mysql -u <DB_USER> -p -e 'USE <DB_NAME>;SOURCE /home/seeds/<any_file>.sql'
```
