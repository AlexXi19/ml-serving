# Swerve: A Fullstack ML Serving Framework

Building for rust & ml practice purposes (for now), hope I can finish this 😅

## Feature Goals

- Fast ML Inference
- Model orchestration

### Phase 1

- [ ] Inference from http endpoints
- [ ] Rust web server, coordinator, and control plane (blazingly fast)
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
- [ ] Kubernetes
  - [ ] Autoscaling
  - [ ] Model orchestration
  - [ ] Load balancing

### Secondary Goals

- [ ] MLOps
  - [ ] Training to production pipeline CI/CD
  - [ ] Model validation
- [ ] Realtime, continuous learning

## Optional CLI Random Thoughts

Should feel like `npm`

- `swerve build`
- `swerve dev`
- `swerve serve` or `swerve start` to start the inference server
- `swerve test`

## Feature thoughts

- Python model environment using `envd` + rust web server all packaged on top of docker under the hood.
- Maybe have a `swerve.config` file？
  - Used to declaratively specify model deployment
- Separating server container and model container, scale individually.
- Separate instances and scaling for CPU and GPU instances.
- Multi-model serving
- Model repository
- Non-http entrypoints
  - Online: grpc
  - Offline: kafka, dagster etc.
    - data pipeline => predict => output storage
