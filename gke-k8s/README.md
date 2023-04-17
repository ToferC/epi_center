# GKE deployment

Luc is also writing the yaml manifests...
I could not get those to run, so I wrote these. We can reconcile all that later.

The service seems to be functioning correctly. The database seed still takes about 10 minutes to complete, be the server is up after that.

I am still pulling the `api` (`people-data-api`) container that luc pushed to dockerhub (`belliveaul/epicenter:latest`), until our own pipeline is ready. Although I have scripts for cloudbuild we could integrate for CI, and the commands to provision the artifact registry are in the *GKE Setup* section below.

I haven't yet managed to provision a persistent disk, so just as in Luc's manifests, the database is ephemeral.
(i.e. it will be lost when the database pod is deleted, or re-created)

In these manifests, I omitted the namespace, so we can deploy them into different namespaces, without modification

## Deploying the manifests

```bash
export PROJECT_ID="pdcp-cloud-009-danl"
# Get the PROJECT_NUMBER from the PROJECT_ID
export PROJECT_NUMBER=$(gcloud projects describe ${PROJECT_ID} --format="value(projectNumber)")
export REGION="northamerica-northeast1"
export CLUSTER_NAME="epi-cluster-1"

gcloud container --project ${PROJECT_ID} clusters list

# install the 'gke-gcloud-auth-plugin' for kubectl: only needed once on a host
gcloud components install gke-gcloud-auth-plugin

gcloud container clusters get-credentials "${CLUSTER_NAME}" --region ${REGION}

kubectl create namespace epi

# For the rest of the commands we assume the namespace epi is set
kubens epi # set the namespace to epi - or add it to *all commands* as "-n epi"

# Bring everything up at once, inspecting things along the way
kubectl apply -f .

## Or bring up each component individually

# Claim the storage
kubectl apply -f postgres-volume.yaml
# Bring up the database
kubectl apply -f postgres.yaml
# Connect to the database (exec) and run a psql command
kubectl exec -it $(kubectl get pods -l app=postgres -o jsonpath='{.items[0].metadata.name}') -- psql -U christopherallison people_data_api
# Get a shell in the database pod
kubectl exec -it $(kubectl get pods -l app=postgres -o jsonpath='{.items[0].metadata.name}') -- bash

# if you want to expose postgres to your local machine
kubectl port-forward svc/postgres-service 5432:5432

kubectl apply -f api-config.yaml
kubectl apply -f api.yaml

# get the logs from the api pod - as it seeds for a long time (like 10-15 minutes)
kubectl logs -f --all-containers -l app=api
# or another way - more explicit; this is how you find the pod
kubectl logs -f $(kubectl get pods -l app=api -o jsonpath='{.items[0].metadata.name}')


# Finally the ingress
kubectl apply -f ingress.yaml
# show the endpoint
kubectl get ingress
# Output will look like this:
# $ kubectl get ingress
# NAME          CLASS    HOSTS   ADDRESS          PORTS   AGE
# api-ingress   <none>   *       34.111.127.169   80      12m
# That address is the external IP (http only for now) of the ingress controller
```

## GKE Setup / Provisioning

In cloud shell, to provision a GKE cluster; you will need to substitute your own values for the PROJECT_ID, PROJECT_NUMBER, and REGION variables,..

```bash
export PROJECT_ID="pdcp-cloud-009-danl"
# Get the PROJECT_NUMBER from the PROJECT_ID
export PROJECT_NUMBER=$(gcloud projects describe ${PROJECT_ID} --format="value(projectNumber)")
export REGION="northamerica-northeast1"
export CLUSTER_NAME="epi-cluster-1"


gcloud config set project ${PROJECT_ID}
gcloud config set run/region ${REGION}
gcloud config set compute/region ${REGION}

# Create an artifact registry
gcloud artifacts repositories create epi-center-repo \
   --repository-format=docker \
   --location=${REGION} \
   --description="epi-center-repo"

# Allow our service account to read from the registry
gcloud artifacts repositories add-iam-policy-binding epi-center-repo \
    --location=${REGION} \
    --member=serviceAccount:${PROJECT_NUMBER}-compute@developer.gserviceaccount.com \
    --role="roles/artifactregistry.reader"

gcloud auth configure-docker ${REGION}-docker.pkg.dev

# docker push ${REGION}-docker.pkg.dev/${PROJECT_ID}/epi-center-repo/hello-app:v1

# Creating a GKE cluster
gcloud config set compute/region ${REGION}

# List the available GKE clusters
gcloud container --project ${PROJECT_ID} clusters list

# Creation of a network and subnet first.
#  - Because of the constraints inside our GCP environment
# The network:
#  list existing networks
gcloud compute networks list
# create a new network - epi-net-can-1
gcloud compute networks create epi-net-can-1 \
  --project ${PROJECT_ID} \
  --subnet-mode custom
#  list existing networks - see if it worked
gcloud compute networks list

# now the subnetwork
export SUBNETWORK_IP_RANGE="10.1.0.0/20"
# This will give us a subnetwork with IP addresses ranging from 10.1.0.1 to 10.1.15.254.
# Since your cluster and services CIDRs are /17 and /22, they won't overlap with this range, assuming they use different base addresses.

# list existing subnets
gcloud compute networks subnets list

gcloud compute networks subnets create epi-subnet-mtl \
  --project ${PROJECT_ID} \
  --region ${REGION} \
  --network epi-net-can-1 \
  --range ${SUBNETWORK_IP_RANGE}
# list existing subnets - see if it worked
gcloud compute networks subnets list

# Provision the GKS CLuster itself
# where net-ca-1 and mtl are networs/subnets I have created before
gcloud container --project ${PROJECT_ID} clusters create-auto "${CLUSTER_NAME}" --region ${REGION} --release-channel "regular" --network "projects/${PROJECT_ID}/global/networks/epi-net-can-1" --subnetwork "projects/${PROJECT_ID}/regions/${REGION}/subnetworks/epi-subnet-mtl" --cluster-ipv4-cidr "/17" --services-ipv4-cidr "/22"

# Get you kubeconfig credentials to enable kubectl to work with your cluster
gcloud container clusters get-credentials "${CLUSTER_NAME}" --region ${REGION}
```

## GKE Teardown

```bash
# Delete the cluster
gcloud container clusters delete "${CLUSTER_NAME}" --region ${REGION}

# delete the subnet
gcloud compute networks subnets list
gcloud compute networks subnets delete epi-subnet-mtl

# delete the network
gcloud compute networks list
gcloud compute networks delete epi-net-can-1

# delete the artifact registry
gcloud artifacts repositories list
gcloud artifacts repositories delete epi-center-repo --location=${REGION}
```

## Confirm the API is working

```bash
export ENDPOINT="http://$(kubectl get ingress api-ingress -o jsonpath='{.status.loadBalancer.ingress[0].ip}')"
# Fetch the home page
echo ${ENDPOINT}
curl ${ENDPOINT}
# Query the API and count the number of people
curl -s "${ENDPOINT}/graphql" -H 'Accept-Encoding: gzip, deflate, br' -H 'Content-Type: application/json' -H 'Accept: application/json' -H 'Connection: keep-alive' -H 'DNT: 1' -H "Origin: ${ENDPOINT}" --data-binary '{"query":"# Write your query or mutation here\nquery {\n                allPeople {\n                  email\n                  workAddress\n                }\n              }\n"}' --compressed |  jq '.data.allPeople | length'
```

## Load Testing

With [rakyll/hey](https://github.com/rakyll/hey#installation):

```bash
# Install hey if not already installed in cloud shell
wget https://hey-release.s3.us-east-2.amazonaws.com/hey_linux_amd64
chmod +x hey_linux_amd64 && sudo mv hey_linux_amd64 /usr/local/bin/hey

export ENDPOINT="http://$(kubectl get ingress api-ingress -o jsonpath='{.status.loadBalancer.ingress[0].ip}')"

# optionally watch the logs in another shell:
# kubectl logs -f --all-containers -l app=api

hey -n 100 -c 10 -m POST -H "Accept-Encoding: gzip, deflate, br" -H "Content-Type: application/json" -H "Accept: application/json" -H "Connection: keep-alive" -H "DNT: 1" -H "Origin: ${ENDPOINT}" -D payload.json.notyaml "${ENDPOINT}/graphql"

Summary:
  Total:    5.2010 secs
  Slowest:  1.0886 secs
  Fastest:  0.1059 secs
  Average:  0.4952 secs
  Requests/sec: 19.2272
  
  Total data: 38659800 bytes
  Size/request: 386598 bytes

Response time histogram:
  0.106 [1]  |■
  0.204 [4]  |■■■■■■
  0.302 [6]  |■■■■■■■■■
  0.401 [20] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.499 [20] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.597 [27] |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.696 [10] |■■■■■■■■■■■■■■■
  0.794 [4]  |■■■■■■
  0.892 [2]  |■■■
  0.990 [3]  |■■■■
  1.089 [3]  |■■■■


Latency distribution:
  10% in 0.3004 secs
  25% in 0.3671 secs
  50% in 0.4982 secs
  75% in 0.5449 secs
  90% in 0.7234 secs
  95% in 0.9547 secs
  99% in 1.0886 secs

Details (average, fastest, slowest):
  DNS+dialup: 0.0007 secs, 0.1059 secs, 1.0886 secs
  DNS-lookup: 0.0000 secs, 0.0000 secs, 0.0000 secs
  req write:  0.0001 secs, 0.0000 secs, 0.0003 secs
  resp wait:  0.3329 secs, 0.0392 secs, 0.5401 secs
  resp read:  0.1614 secs, 0.0028 secs, 0.7079 secs

Status code distribution:
  [200] 100 responses

```

Just for fun, let's try with hyperfine:

```bash
hyperfine "curl -s '${ENDPOINT}/graphql' -H 'Accept-Encoding: gzip, deflate, br' -H 'Content-Type: application/json' -H 'Accept: application/json' -H 'Connection: keep-alive' -H 'DNT: 1' -H 'Origin: ${ENDPOINT}' --data-binary '{\"query\":\"query {\n  allPeople {\n    email\n    workAddress\n  }\n}\n\"}' --compressed | jq '.data.allPeople | length'"

Benchmark 1: curl -s ... | jq '.data.allPeople | length'
  Time (mean ± σ):      79.5 ms ±   2.6 ms    [User: 24.9 ms, System: 6.4 ms]
  Range (min … max):    74.0 ms …  89.6 ms    36 runs
```

## References

- <https://cloud.google.com/kubernetes-engine/docs/tutorials/hello-app>