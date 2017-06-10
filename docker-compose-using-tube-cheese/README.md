# Use sozu as reverse-proxy with docker-compose (using tube-cheese)

First, you have to install [docker-compose](https://docs.docker.com/compose/install/) to execute this demo

On OSX, you can use Docker for Mac with success.

We have to add the domain names to our host file. We need to know the machine IP and add it to the /etc/hosts file
```
# then open your hosts file and add the line, with the ip you just get
127.0.0.1 pokemons.local pikachu.local mewtwo.local nidoqueen.local i-like-pikachu.local i-like-mewtwo.local i-like-nidoqueen.local
```

If you want to use Docker Machine, you can too. Just put the right ip into /etc/hosts
`docker-machine ip`

First, we will start the k8s cluster (it takes few minutes):
```
docker-compose -f docker-compose.pokemons.yml up -d
```

Next, deploy the many pokemons containers
```
docker-compose up -d
```

Open browser and try to open these urls to discover the proxying is working.
* [pikachu.local](http://pikachu.local) 
* [mewtwo.local](http://mewtwo.local) 
* [nidoqueen.local](http://nidoqueen.local)
But the domain i-like-pikachu.local is not working and display 404. If we apply a new ingress with this domaine name.

Best for now is to open all files and play with it ;-)
