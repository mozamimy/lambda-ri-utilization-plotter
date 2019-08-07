BUILD_IMAGE := ri-utilization-plotter-build
APP_NAME := ri-utilization-plotter
LOGICAL_FUNCTION_NAME := RIUtilizationPlotter

init:
	docker build -t ${BUILD_IMAGE} .

build:
	docker run --rm -v ${PWD}:/workspace:cached -v ${PWD}/tmp/cargo-registry:/usr/local/cargo/registry:cached ${BUILD_IMAGE} tools/build.sh

release_build:
	docker run --rm -v ${PWD}:/workspace:cached -v ${PWD}/tmp/cargo-registry:/usr/local/cargo/registry:cached -e RELEASE=1 ${BUILD_IMAGE} tools/build.sh

run:
	sam local invoke -e event.json -t template.yml ${LOGICAL_FUNCTION_NAME}

