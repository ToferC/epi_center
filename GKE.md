# GKE deployment

- Luc is writing the yaml.

## GKE Setup / Provisioning

In cloud shell, to provision a GKE cluster; you will need to substitute your own values for the PROJECT_ID, PROJECT_NUMBER, and REGION variables,..

```bash
export PROJECT_ID="pdcp-cloud-009-danl"
export PROJECT_NUMBER="101744527752"
export REGION="northamerica-northeast1"

gcloud config set project ${PROJECT_ID}
gcloud config set run/region ${REGION}

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

docker push ${REGION}-docker.pkg.dev/${PROJECT_ID}/epi-center-repo/hello-app:v1

# Creating a GKE cluster
gcloud config set compute/region ${REGION}
gcloud container clusters create-auto epi-cluster

# TODO: I need to document the creation of a network and subnet first.
#   This is required because of the constraints inside our GCP environment
# Provision the GKS CLuster itself
# where net-ca-1 and mtl are networs/subnets I have created before
gcloud container --project ${PROJECT_ID} clusters create-auto "epi-cluster-1" --region ${REGION} --release-channel "regular" --network "projects/${PROJECT_ID}/global/networks/net-can-1" --subnetwork "projects/${PROJECT_ID}/regions/${REGION}/subnetworks/mtl" --cluster-ipv4-cidr "/17" --services-ipv4-cidr "/22"

# Get you kubeconfig credentials to enable kubectl to work with your cluster
gcloud container clusters get-credentials epi-cluster-1 --region ${REGION}

# Delete the cluster
gcloud container clusters delete epi-cluster-1 --region ${REGION}
```

## References

- <https://cloud.google.com/kubernetes-engine/docs/tutorials/hello-app>