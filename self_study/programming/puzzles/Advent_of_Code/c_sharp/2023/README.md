# AoC 2023 written in .NET 7.0


### About

Written in C# with .NET sdk 7.0

Each days solve reside in AoC/Day{N} where N = the day of the puzzle.

Every day has two parts. You can find both defined as filename Part1.cs and Part2.cs

No parts share code; each part is solved by itself and is fully independent of other solves.

You will see repetitive code as a result of each part being independend and that is by choice for now.


### Run in Container

You can run everythign in a container from this directory (swap podman with docker if you have Docker)

Run all solves
```sh
podman run --privileged -it --rm --mount type=bind,source="$(pwd)",target=/App mcr.microsoft.com/dotnet/sdk:7.0 dotnet run --project App
```

Run day 5
```sh
podman run --privileged -it --rm --mount type=bind,source="$(pwd)",target=/App mcr.microsoft.com/dotnet/sdk:7.0 dotnet run --project App 5
```

Run day 5 part 2
```sh
podman run --privileged -it --rm --mount type=bind,source="$(pwd)",target=/App mcr.microsoft.com/dotnet/sdk:7.0 dotnet run --project App 5 2
```

### Day 8 Part 2

[An LCM problem](https://en.wikipedia.org/wiki/Least_common_multiple)

Our task is to find the steps it takes for all nodes starting with 'A' to end with 'Z', simultaneously.
However, doing this will take for ever.

Instead, we run each node at a time and until we find first node ending with 'Z'.
We end up with having calculated the steps for each node, one at a time.

Finding the lowest common multiple from all the nodes will yield
the number that would eventually be found by doing them all simultaneously.
