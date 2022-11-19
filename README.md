# Swerve: A Fullstack ML Serving Framework 
Building for rust & ml practice purposes (for now), hope I can finish this 😅

## Design Objectives 
In order of importance:
1. Simple! 
2. Robust Delivery 
3. Available 
4. Fast 

### Phase 1
- [ ] Inference from http endpoints 
- [ ] Rust web server & coordinator (blazingly fast)
- [ ] Tensorflow model 
- [ ] Runs some python script to perform inference 

### Phase 2 
- [ ] Robust message delivery from rust => python through some tbd network protocol/message queue
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
