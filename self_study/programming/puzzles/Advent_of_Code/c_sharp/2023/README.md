# AoC - 2023
Solved with .NET 7.0

### Run Podman Container (or Docker)
Run all the puzzles with this simple command (swap podman with docker if you have Docker installed)
```sh
podman run --privileged -it --rm --mount type=bind,source="$(pwd)",target=/App mcr.microsoft.com/dotnet/sdk:7.0 dotnet run --project App
```
