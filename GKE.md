# GKE deployment

- Luc is writing the yaml.

## GKE Setup

In cloud shell:

```bash
export PROJECT_ID="pdcp-cloud-009-danl"
export PROJECT_NUMBER="101744527752"
export REGION="northamerica-northeast1"

gcloud config set project ${PROJECT_ID}
gcloud config set run/region ${REGION}

gcloud artifacts repositories create epi-center-repo \
   --repository-format=docker \
   --location=${REGION} \
   --description="epi-center-repo"

gcloud artifacts repositories add-iam-policy-binding epi-center-repo \
    --location=${REGION} \
    --member=serviceAccount:${PROJECT_NUMBER}-compute@developer.gserviceaccount.com \
    --role="roles/artifactregistry.reader"

gcloud auth configure-docker ${REGION}-docker.pkg.dev

docker push ${REGION}-docker.pkg.dev/${PROJECT_ID}/epi-center-repo/hello-app:v1

# Creating a GKE cluster
gcloud config set compute/region ${REGION}
gcloud container clusters create-auto epi-cluster

# where net-ca-1 and mtl are networs/subnets I have created before
gcloud container --project ${PROJECT_ID} clusters create-auto "epi-cluster-1" --region ${REGION} --release-channel "regular" --network "projects/${PROJECT_ID}/global/networks/net-can-1" --subnetwork "projects/${PROJECT_ID}/regions/${REGION}/subnetworks/mtl" --cluster-ipv4-cidr "/17" --services-ipv4-cidr "/22"

gcloud container clusters get-credentials epi-cluster-1 --region ${REGION}
kubectl create deployment hello-app --image=${REGION}-docker.pkg.dev/${PROJECT_ID}/epi-center-repo/hello-app:v1
kubectl scale deployment hello-app --replicas=3
kubectl autoscale deployment hello-app --cpu-percent=80 --min=1 --max=5
kubectl get pods

# Now a service
kubectl expose deployment hello-app --name=hello-app-service --type=LoadBalancer --port 80 --target-port 8080
kubectl get service

kubectl delete service hello-app-service
kubectl delete horizontalpodautoscaler.autoscaling/hello-app
kubectl delete deployment hello-app

```

## Cloud Shell based demo

```bash
git clone https://github.com/GoogleCloudPlatform/kubernetes-engine-samples
cd kubernetes-engine-samples/hello-app

docker build -t ${REGION}-docker.pkg.dev/${PROJECT_ID}/epi-center-repo/hello-app:v1 .
docker run --rm -p 8080:8080 ${REGION}-docker.pkg.dev/${PROJECT_ID}/epi-center-repo/hello-app:v1
docker push ${REGION}-docker.pkg.dev/${PROJECT_ID}/epi-center-repo/hello-app:v1

```

## References

- <https://cloud.google.com/kubernetes-engine/docs/tutorials/hello-app>