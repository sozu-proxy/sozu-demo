# Use sozu as reverse-proxy with docker-compose (using tube-cheese)

First, you have to install [docker-compose](https://docs.docker.com/compose/install/) to execute this demo

We have to add the domain names to our */etc/hosts* file
```
127.0.0.1 pikachu.local mewtwo.local nidoqueen.local
```

If you want to use Docker Machine, you can too. Just put the right ip into /etc/hosts `docker-machine ip`

First, we will start the local containers (it takes few minutes):
```
docker-compose up -d
```

Next, deploy the many pokemons containers
```
docker-compose -f docker-compose.pokemons.yml up -d
```

Open browser and try to open these urls to discover the proxying is working.
* [pikachu.local](http://pikachu.local)
* [mewtwo.local](http://mewtwo.local)
* [nidoqueen.local](http://nidoqueen.local)

Best for now is to open all files and play with it ;-)
