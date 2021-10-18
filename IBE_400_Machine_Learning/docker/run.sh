#!/usr/bin/env bash
#────────────────────────────────────────────#
# Author(s):
#   Emil Bratt Børsting
#────────────────────────────────────────────#
# Description:
#   easy to get up and running with a local anacondas jupyter notebook
#────────────────────────────────────────────#

REPO_MAINTAINER=continuumio
REPO_NAME=anaconda3
DOCKER_NAME=anaconda
TAG=latest
PORT=8888
MOUNT_PATH_HOST_scripts=$(pwd)/scripts
MOUNT_PATH_CONTAINER_scripts=$HOME/scripts
MOUNT_PATH_HOST_notebooks=$(pwd)/notebooks
MOUNT_PATH_CONTAINER_notebooks=/opt/notebooks
STATUS_RUN_VALUE='\033[0;32mRunning\033[0m\n'
STATUS_STOP_VALUE='\033[0;35mStopped\033[0m\n'
STATUS_NOT_INSTALLED_VALUE='\033[0;35mNot Installed\033[0m\n'


function _confirm_docker_connection () {
  docker ps > /dev/null 2>&1 && return 0
  echo 'Make sure you have Docker installed and running'
  return 1
}

function _start () {

  if [[ "$status" == "$STATUS_NOT_INSTALLED_VALUE" ]]; then
    unset status
    _run
  fi

  mkdir -p "$MOUNT_PATH_HOST_scripts"
  mkdir -p "$MOUNT_PATH_HOST_notebooks"

  # if running, just return 0
  docker ps | grep -q "$REPO_MAINTAINER/$REPO_NAME" && return 0

  echo 'Starting..'
  docker start $DOCKER_NAME 1> /dev/null
  n=1
  while [[ true ]]; do
    # wait for container to start until returning 0
    sleep 0.5
    docker ps | grep -q "$REPO_MAINTAINER/$REPO_NAME" &&
      status=$STATUS_RUN_VALUE && return 0

    # watchdog if it never started as a safeguard
    ((n=n+1))
    if [[ $n -gt 20 ]]; then
      exit 1
    fi
  done
}


function _run () {

  # we only start and stop the container for anaconda to be installed

  mkdir -p "$MOUNT_PATH_HOST_scripts"
  mkdir -p "$MOUNT_PATH_HOST_notebooks"
  echo 'Installing Anaconda..'
  docker run -id -t -p $PORT:$PORT \
    --volume "$MOUNT_PATH_HOST_scripts":"$MOUNT_PATH_CONTAINER_scripts" \
    --volume "$MOUNT_PATH_HOST_notebooks":"$MOUNT_PATH_CONTAINER_notebooks" \
    --name $DOCKER_NAME \
     $REPO_MAINTAINER/$REPO_NAME \
     /bin/bash -c "\
      conda install jupyter -y --quiet && \
      mkdir -p '$MOUNT_PATH_CONTAINER_notebooks' && \
      mkdir -p '$MOUNT_PATH_CONTAINER_scripts' && \
      jupyter notebook \
      --notebook-dir=/opt/notebooks --ip='*' --port=$PORT \
      --no-browser --allow-root" 1> /dev/null
  _stop
  _start
}


function _stop () {
  echo "Stopping $DOCKER_NAME continaer.."
  docker stop $DOCKER_NAME 1> /dev/null

  while [[ true ]]; do
    # when we cant grep the container, is when we are sure it is stopped
    docker ps | grep -q "$REPO_MAINTAINER/$REPO_NAME" || return 0
  done
}


function _remove () {
  docker ps | grep -q "$REPO_MAINTAINER/$REPO_NAME" && _stop
  echo "Removing $DOCKER_NAME contianer.."
  docker rm $DOCKER_NAME
}


function _delete () {
  docker ps | grep -q "$REPO_MAINTAINER/$REPO_NAME" && _stop
  docker ps -a | grep -q "$REPO_MAINTAINER/$REPO_NAME" && _remove
  echo "Deleting $DOCKER_NAME Images.."
  docker rmi "$REPO_MAINTAINER/$REPO_NAME:$TAG" 1> /dev/null
}


function _exec () {
  printf 'Type: '; read CMD
  docker exec -it $DOCKER_NAME $CMD
}


function _enter_bash () {
  if [[ $status != $STATUS_RUN_VALUE ]]; then
    _start
  fi
  docker exec -it $DOCKER_NAME bash
}


function _run_python_script () {
  if [[ $status != $STATUS_RUN_VALUE ]]; then
    _start
  fi
  echo 'Place your python scripts here:'
  printf "\033[0;32m$MOUNT_PATH_HOST_notebooks\033[0m\n"
  _list_options \
    "1. Open python shell" \
    "2. Open python file" \
    '0. Exit'

  printf 'Type: '; read _OPTN

  if [[ $_OPTN == 1 ]]; then
    docker exec -it $DOCKER_NAME python
  elif [[ $_OPTN == 2 ]]; then
    echo 'Type in the name of the python script you want to run and press enter'
    printf 'Type: '; read  name
    docker exec -it $DOCKER_NAME python $MOUNT_PATH_CONTAINER_scripts/$name
  elif [[ $_OPTN == 3 ]]; then
    exit
  fi
}


function _list_options () {
  # prints out options loaded from arguments
  # takes an arbitrary amount of args, they will all be printed on separate lines
  echo -e '----------------------------------------------'
  for optn in "$@"
  do
    echo "$optn"
  done
  echo -e '----------------------------------------------'
}


function _mainloop () {

  _confirm_docker_connection || exit 1

  declare status


  docker images | grep -q "$REPO_MAINTAINER/$REPO_NAME" && status=$STATUS_NOT_INSTALLED_VALUE
  docker ps -a | grep -q "$REPO_MAINTAINER/$REPO_NAME" && status=$STATUS_STOP_VALUE
  docker ps | grep -q "$REPO_MAINTAINER/$REPO_NAME" && status=$STATUS_RUN_VALUE

  echo ''
  if [[ -z ${status+x} ]]; then
    echo '--Anaconda Options--'
    _list_options \
      "1. Download and start $DOCKER_NAME" \
      '0. Exit'

    printf 'Type: '; read _OPTN

    if [[ $_OPTN == 1 ]]; then
      _run
    else
      exit
    fi
  fi

  if [[ "$status" == "$STATUS_RUN_VALUE" ]]; then
    local URL=''
    echo 'Initializing..'
    while [[ $URL == '' ]]; do
      sleep 1
      _url=$(docker exec -t $DOCKER_NAME jupyter notebook list | grep 'http' | cut -d ':' -f1-3)
      echo $_url | grep -q 'http' && URL=$_url
    done

  fi

  echo '--Anaconda Options--'
  _list_options \
    "1. Start $DOCKER_NAME" \
    "2. Open BASH shell inside $DOCKER_NAME" \
    "3. Stop $DOCKER_NAME" \
    "4. Remove $DOCKER_NAME container" \
    "5. Delete image $REPO_MAINTAINER/$REPO_NAME" \
    "6. Open python shell inside $DOCKER_NAME" \
    '0. Exit'
  printf "Status: $status"
  printf 'Put your jupyter notebooks here: '
  printf "\033[0;32m$MOUNT_PATH_HOST_notebooks\033[0m\n"
  printf 'Put anything else here: '
  printf "\033[0;32m$MOUNT_PATH_HOST_scripts\033[0m\n"
  echo $URL | grep -q 'http' &&
    printf 'Visit Jupyter server at: ' &&
    printf "\033[0;32m$URL\033[0m\n"

  # force sleep to let echo | grep finish to avoid suppressing next output
  sleep 0.2
  printf 'Type: '; read _OPTN

  if [[ $_OPTN == 0 ]]; then
    exit

  elif [[ $_OPTN == '1' ]]; then
    _start

  elif [[ $_OPTN == 2 ]]; then
    _enter_bash

  elif [[ $_OPTN == 3 ]]; then
    _stop

  elif [[ $_OPTN == 4 ]]; then
    _remove

  elif [[ $_OPTN == 5 ]]; then
    _delete

  elif [[ $_OPTN == 6 ]]; then
    _run_python_script

  fi
}


### entrypoint ###
while [[ true ]]; do _mainloop; done
