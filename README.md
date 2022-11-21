# Swerve: A Fullstack ML Serving Framework 
Building for rust & ml practice purposes (for now), hope I can finish this 😅

## Design Objectives 
In order of importance:
1. Simple! 
2. Robust 
3. Available 
4. Fast 

### Phase 1
- [ ] Inference from http endpoints 
- [ ] Rust web server & coordinator (blazingly fast)
- [ ] Tensorflow model 
- [ ] Runs some python script to perform inference 

### Phase 2 
- [ ] Robust message delivery from rust => python through some tbd network protocol/message queue/stream
- [ ] Simple python interface to configure model serving (instead of script)
- [ ] Deployment 
    - [ ] Containerized 

### Phase 3 
- [ ] Batch Processing 
- [ ] Observability 
- [ ] Availability 
- [ ] Speed
- [ ] Kubernetes

### Phase very far in the future maybe never 
- [ ] MLOps 
    - [ ] Training to production pipeline CI/CD
    - [ ] Model validation 
    - [ ] Feature store 
- [ ] Realtime, continuous learning 
- [ ] Smart orchestration, load balancing and scaling 

## CLI Random Thoughts
Should feel like `npm` 
- `swerve build` 
- `swerve dev` 
- `swerve serve` or `swerve start` to start the inference server
- `swerve test` 

## Feature thoughts 
- Python model environment using `envd` + rust web server all packaged on top of docker under the hood. 
- Maybe have a `swerve.config` file？
- Separating server container and model container, scale individually. 
- Multi-model serving 
- Model from model store 
- Non-http entrypoints 
    - Online: grpc
    - Offline: kafka, dagster etc.
        - data pipeline => predict => output storage 

## What `swerve` is trying to do 
- Provides simple interface to start and deploy an ml inference server. 
- United all aspects of machine learning into one platform (training, serving and more) 
- Easy to use + Lightweight

## Questions and Concerns 
- 
