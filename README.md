# ri-utilization-plotter

This AWS Lambda fucntion puts datapoinsts fetched by Cost Explorer API to CloudWatch custom metrics.

## Build and Debug with SAM CLI

Copy event.example.json to evnet.json and template.example.yml to template.yml and edit them for your development environment before follow bellow instructions.

```sh
make init # You need this command at first time building
make build
make run
```

## Build for Releasing

```sh
make init # You need this command at first time building
make release_build
```

## Customize

This function is designed to be invoked periodically by a CloudWatch Event. You can customize the parameter for Cost Explorer API and metrics' namespace and metric name by configuring input of target configuration of event rules. (See event.example.json)

## License

MIT
