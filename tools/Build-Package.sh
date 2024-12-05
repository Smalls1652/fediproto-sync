#/bin/bash

PACKAGE_NAME=""
PROFILE_NAME=""
while getopts ":n:p:" opt; do
    case "${opt}" in
        n) PACKAGE_NAME=${OPTARG};;
        p) PROFILE_NAME=${OPTARG};;
    esac
done

if [ -z "${PACKAGE_NAME}" ]; then
    echo "package is required"
    exit 1
fi

if [ -z "${PROFILE_NAME}" ]; then
    PROFILE_NAME="Debug"
fi

if ! [[ "${PROFILE_NAME}" =~ ^(Debug|Release)$ ]]; then
    echo "profile must be debug or release"
    exit 1
fi

if [[ "${PROFILE_NAME}" == "release" ]]; then
    cargo build --package ${PACKAGE_NAME} --release
else
    cargo build --package ${PACKAGE_NAME}
fi
