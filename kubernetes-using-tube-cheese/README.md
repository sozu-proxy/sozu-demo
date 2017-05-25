# Use sozu as ingress on a kubernetes cluster (using tube-cheese)

First, you have to install [minikube](https://github.com/kubernetes/minikube) to execute this demo

On OSX, minikube is avaible on brew

```
brew install minikube
```

First, we will start the k8s cluster (it takes few minutes):
```
minikube start
```

We have to add the domain names to our host file. We need to know the minikube IP and add it to the /etc/hosts file
```
minikube ip
# then open your hosts file and add the line, with the ip you just get
$MINIKUBEIP pokemons.local pikachu.local mewtwo.local nidoqueen.local i-like-pikachu.local i-like-mewtwo.local i-like-nidoqueen.local
```

Next, deploy the many pokemons services
```
# First the containers for pokemons
kubectl apply -f pokemon-deployments.yaml

# Then the services for pokemons
kubectl apply -f pokemon-services.yaml

# Then the ingress for pokemon
kubectl apply -f pokemon-ingress.yaml

# And deploy sozu as ingress manager
kubectl apply -f k8s-sozu-ingress.yaml

# Watch progression of deployment in the k8s dashboard
minikube dashboard
```

Open browser and try pikachu.local mewtwo.local nidoqueen.local to discover the proxying is working. But the domain i-like-pikachu.local is not working and display 404. If we apply a new ingress with this domaine name.
```
# Then the ingress for pokemon
kubectl apply -f i-like-pokemon-ingress.yaml
```

Now we can use i-like-pikachu.local and even http://i-like-pikachu.local/nidoqueen-to ;-)

Best for now is to open all files and play with it ;-)
