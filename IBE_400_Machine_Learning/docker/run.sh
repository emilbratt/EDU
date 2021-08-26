#!/usr/bin/env bash

_REPO_MAINTAINER='continuumio' # set docker image repository alias
_REPO_NAME='anaconda3' # set docker image name
_DOCKER_NAME='anaconda' # set docker container service name
_TAG='latest'
_PORT='8888:8888' # connect to service through port 80 (service inside runs on port 8080)
_MOUNT_PATH_HOST_notebooks="$(pwd)/notebooks"
_MOUNT_PATH_CONTAINER_notebooks='/opt/notebooks'

# create notebook dir for persistancy if not exist
if [[ ! -d $_MOUNT_PATH_HOST_notebooks ]]; then
  mkdir -p $_MOUNT_PATH_HOST_notebooks
fi

_run () {
  docker ps | grep -q continuumio/anaconda3 && docker stop $_DOCKER_NAME
  docker ps -a | grep -q continuumio/anaconda3 && docker rm $_DOCKER_NAME
  docker run -i -t -p $_PORT \
    --volume "$_MOUNT_PATH_HOST_notebooks":"$_MOUNT_PATH_CONTAINER_notebooks" \
    --name $_DOCKER_NAME \
     $_REPO_MAINTAINER/$_REPO_NAME \
     /bin/bash -c "\
      conda install jupyter -y --quiet && \
      mkdir -p /opt/notebooks && \
      jupyter notebook \
      --notebook-dir=/opt/notebooks --ip='*' --port=8888 \
      --no-browser --allow-root"
}

_delete () {
  docker rmi "$_REPO_MAINTAINER/$_REPO_NAME:$_TAG"
}

_stop () {
  docker stop $_DOCKER_NAME
}

_enter_bash () {
  docker exec -it $_DOCKER_NAME bash
}

_list_options () {
  # prints out options loaded from arguments
  # takes an arbitrary amount of args, they will all be printed on separate lines
  echo -e '\n----------------------------------------------'
  for optn in "$@"
  do
    echo "$optn"
  done
  echo -e '----------------------------------------------\n'
}
_list_options '1. run' '2. shell' '3. stop' '4. delete' '0. exit'

read _OPTN

if [[ $_OPTN == 0 ]]; then
  exit

elif [[ $_OPTN == 1 ]]; then
  _run    && echo 'ok' || echo 'error'

elif [[ $_OPTN == 2 ]]; then
  _enter_bash

elif [[ $_OPTN == 3 ]]; then
  _stop   && echo 'ok' || echo 'error'

elif [[ $_OPTN == 4 ]]; then
  _delete

fi
