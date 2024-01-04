# .NET


## Setup for Cross Platform Developement

[Install .NET core](https://learn.microsoft.com/en-us/dotnet/core/install/)

[Install VS-Code](https://code.visualstudio.com/download)

[Setting up VS-Code for .NET](https://code.visualstudio.com/docs/languages/dotnet#_setting-up-vs-code-for-net-development)

[Official .NET Documentation](https://learn.microsoft.com/en-us/dotnet/)


### Get started with a HelloWorld console project

create directory "HelloWorld"
```sh
mkdir HelloWorld
```

cd into the project root directory
```sh
cd HelloWorld
```

create the console project (change sdk version for your use case)
* ..the project will inherit the name of the parent directory
```sh
dotnet new console --framework net8.0 --use-program-main
```

add standard gitignore for .NET projects
```sh
dotnet new gitignore
```

run the application (will automatically build first)
```sh
dotnet run
```

remove output of the previous build
```sh
dotnet clean
```


## Opening a project in your editor

Make sure to navigate to the directory containing the Program.cs file.
Open this directory in your editor of choice
The dev-tools in yor IDE depend on having the file and folder structure intact.


## Run in a Container (Podman)
Open a container with a bind-mount for the current working directory which mounts to /App inside the container

Change sdk version for your use case
```sh
podman run --privileged -it --rm --mount type=bind,source="$(pwd)",target=/App mcr.microsoft.com/dotnet/sdk:8.0
```
