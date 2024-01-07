# AoC 2023 in C# .NET 7.0


### About

Each days solve reside in AoC/Day{N} where N = the day of the puzzle.\
Every day has two parts. You can find both defined as filename Part1.cs and Part2.cs\
No parts share code; each part for every day is solved by itself and is fully independent of the other part.\
You will see some repetitive code for input parsing/loading as a result.


### Run in Container

You can run everything in a container from this directory (swap podman with docker if you have Docker)

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

Our task is to find the steps it takes for all nodes starting with 'A' to end with 'Z', simultaneously.\
However, doing this will take forever.

Instead, we run each node at a time until we find first node ending with 'Z'.\
We end up with having calculated the steps for each node, one at a time.

Finding the lowest common multiple from all the nodes will yield\
the number that would eventually be found by doing them all simultaneously.

### Day 9 Part 1 and Part 2

[A Binomial coefficient problem](https://en.wikipedia.org/wiki/Binomial_coefficient)

My solution after building the triangle (I only use one number from each row and dont actually build it..).

Part 1 - Extrapolating forward (number that comes after the right most number)

Start with the right most number.\
Summarize this number with all the numbers below it on the left diagonal.\
The number you calculated is number that comes after it.

Part 2 - Extrapolating backwards (number that comes before the left most number)

Start with the number on the bottom right in the same diagonal as the left most number.\
Move one up on the left diagonal so you get one step closer to it.\
Subtract the number you arrive at with the previous number (the number below on the right diagonal).\
Move up again, take that number and subtract with the value you got from the last subtraction.\
Keep doing this until you arrive back at the starting number.\
The number you calculated is the one that comes before it.
