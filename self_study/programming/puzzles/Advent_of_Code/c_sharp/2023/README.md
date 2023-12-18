# AoC 2023 written in .NET 7.0


### About

Written in C# with .NET sdk 7.0

Each days solve reside in AoC/Day{N} where N = the day of the puzzle.

Every day has two parts. You can find both defined as filename Part1.cs and Part2.cs

No parts share code; each part is solved by itself and is fully independent of other solves.

You will see repetitive code as a result of each part being independend and that is by choice for now.


### Run Podman Container (or Docker)
Run all the puzzles with this simple command (swap podman with docker if you have Docker installed)
```sh
podman run --privileged -it --rm --mount type=bind,source="$(pwd)",target=/App mcr.microsoft.com/dotnet/sdk:7.0 dotnet run --project App
```

Run only day 5 (swap podman with docker if you have Docker installed)
```sh
podman run --privileged -it --rm --mount type=bind,source="$(pwd)",target=/App mcr.microsoft.com/dotnet/sdk:7.0 dotnet run --project App 5
```

Run only day 5 part 2 (swap podman with docker if you have Docker installed)
```sh
podman run --privileged -it --rm --mount type=bind,source="$(pwd)",target=/App mcr.microsoft.com/dotnet/sdk:7.0 dotnet run --project App 5 2
```
