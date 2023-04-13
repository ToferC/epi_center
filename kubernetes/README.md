# Kubernetes configuration

This directory contains a sample configuration than can be used
to deploy this application to kubernetes.

## Google Cloud Platform

Here are some instructions to get started on GCP.

### Enable GKE APIs

```
gcloud services enable container.googleapis.com
```

### Create a GKE cluster.

```
gcloud container clusters create-auto epicenter-cluster \
    --region northamerica-northeast1 \
    --project=PROJECT_ID
```

### Connect to created cluster.

```
gcloud container clusters get-credentials epicenter-cluster \
    --region northamerica-northeast1 \
    --project=PROJECT_ID
```

### Apply the kubernetes configurtation.

Edit the values in the [secrets.yaml](secrets.yaml) file.  Each value must be base64 encoded.  **Make sure not to commit your changes to git!**

```
kubectl apply -f kubernetes
```
