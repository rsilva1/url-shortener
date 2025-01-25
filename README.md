Validating urls has some intricacies such as choosing a regex (there are many),
choosing which schemes to consider (in our case only http/https).
So, using a simplified validation with url crate feels good enough.

Inside `url-shortener` folder, run:
```
cornucopia live $(cat ../.env | grep DATABASE_URL | sed 's/DATABASE_URL=//' | sed 's/"//g')
```
