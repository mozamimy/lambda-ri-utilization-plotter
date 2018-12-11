# ri-utilization-plotter

This AWS Lambda fucntion puts datapoinsts fetched by Cost Explorer API to CloudWatch custom metrics.

## Build and Debug with SAM CLI

```sh
make build-docker-image # You need this command at first time building
make build
make zip
make run
```

## Build for Releasing

```sh
make build-docker-image # You need this command at first time building
make build BUILD=release
make zip BUILD=release
```

## Customize

This function is designed to be invoked periodically by a CloudWatch Event. You can customize the parameter for Cost Explorer API and metrics' namespace and metric name by configuring input of target configuration of event rules. (See event.example.json)

## License

MIT
