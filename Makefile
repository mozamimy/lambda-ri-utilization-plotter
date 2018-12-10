BUILD := debug
APP_NAME := lambda-ri-utilization-plotter
JOBS := 4
LOGICAL_FUNCTION_NAME := RIUtilizationPlotter

build-docker-image:
	docker build -t aws-lambda-rust .
build:
	if [ ${BUILD} == "release" ]; then \
		docker-compose run builder /bin/bash -c "cargo build --jobs ${JOBS} --release"; \
	else \
		docker-compose run builder /bin/bash -c "cargo build --jobs ${JOBS}"; \
	fi
check-fmt:
	docker-compose run builder /bin/bash -c "cargo fmt --all -- --check"
zip:
	docker-compose run builder /bin/bash -c "cp /tmp/target/${BUILD}/bootstrap /workspace/package"; \
	cd package && zip ${APP_NAME}.zip bootstrap
run:
	sam local invoke -e event.example.json -t template.example.json ${LOGICAL_FUNCTION_NAME}
clean:
	rm -rf target/debug target/release package/${APP_NAME}.zip
