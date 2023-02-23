# Design

## Design Diagram

![multi-model-serving](https://tbsnhkewuwyfxowgazvr.supabase.co/storage/v1/object/public/public/ml/multi-model-serving.png)

## Component Explanation

Control Plane:

- Model Orchestration by watching and applying the CRD for models
- Maximize system resource via intelligent management of in-memory model data based on model usage
- Load balancing on models
- Routes requests and dispatches tasks to pre/post-processing and inference

GPU Pods

- Hosts the machine learning server (e.g. Nvidia Triton) that can perform inference and load/unload models

CPU Pods

- Hosts the pre/post-processing server that performs input processing

CRD

- Specified model configs (e.g. total # instances, # GPU allocated, etc.)
