# AoC 2023 in C# .NET 8.0

[Website](https://adventofcode.com/2023)

### Solves

Each days solve reside in AoC/Day{N} where N is the day of the puzzle.\
Every day has two parts; both defined as filename Part1.cs and Part2.cs.

### Why aren't you 'DRY'-ving?

Yes, there is repetitive code in every part and I don't usually write code like this.\
The only shared (non-repetive code) is loading the input from a file into a string type.\
That code is found here: /AoC/PuzzleIO.cs

This might seem dumb especially for days where part 2 differs only by a couple of line changes from part 1.\
So why do each part contain all the code needed for the solve, making them fully independent?\
Sharing code between parts forces me to jump around the source code to understand everything.\
I.e. creating abstractions for common tasks can make it more time consuming to read through the code later.\
I want the ability to just jump right into any part and read the whole solution.\
Bottom line; I am hoping that future me saves time by not "dryving". :)

### Run in Container

You can run everything in a container from this directory (swap podman with docker if you have Docker)

Run all solves
```sh
podman run -it --rm --mount type=bind,source="$(pwd)",target=/App,U=true --workdir /App mcr.microsoft.com/dotnet/sdk:8.0 dotnet run
```

Run day 5
```sh
podman run -it --rm --mount type=bind,source="$(pwd)",target=/App,U=true --workdir /App mcr.microsoft.com/dotnet/sdk:8.0 dotnet run 5
```

Run day 5 part 2
```sh
podman run -it --rm --mount type=bind,source="$(pwd)",target=/App,U=true --workdir /App mcr.microsoft.com/dotnet/sdk:8.0 dotnet run 5 2
```

### Day 8 Part 2

[An LCM problem](https://en.wikipedia.org/wiki/Least_common_multiple)

Our task is to find the steps it takes for all nodes starting with 'A' to end with 'Z', simultaneously.\
However, doing this will take forever.

Instead, we run each node at a time until it ends with 'Z'.\
Ultimately we calculate the total steps it would have taken for every node by using [this](https://en.wikipedia.org/wiki/Least_common_multiple#Gears_problem) method.

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

### Day 10 Part 2

[A point-in-polygon problem](https://en.wikipedia.org/wiki/Point_in_polygon)

This one was hard for me. Spent some hours making a mental model of how to translate this problem into code.\
A way of counting all the tiles that are enclosed, is by knowing wheter or not we are inside or not.\
At first glance it seems easy, at second glance it becomes hard.\
Having watched alot of youtube math content (disclosure: I am not good at math..),\
I came to remember something about crossing lines and poligons.

So it began.\
Re-searching this, I learned the main principle.\
Finding whether a point is inside or outside a polygon, is to test how many times a ray\
starting from the point and going in any fixed direction (in our case only the right direction)\
intersects the edges of the polygon which in our case is the enclosed (marked) pipemaze.\
If the point is on the inside of the polygon then it will intersect the edge an odd number of times.\
If the point is on the outside of the polygon the ray will intersect its edge an even number of times.\
So basically it is every other time it intersects, which we can represent using true and false.

I finally succeeded and I am feeling great about it.

### Day 12

[A Dynamic Programming problem](https://en.wikipedia.org/wiki/Dynamic_programming)

The hardest AoC puzzle I have encountered with good chunks of the AoC community seeming to agree.

There was simply no way where I would be capable of solving both parts on my own.\
In essence, we are trying different configurations to see what "final result" satisfies all the defined rules.\
Kinda like a [nonogram logic puzzle](https://en.wikipedia.org/wiki/Nonogram#Example).\
Also, part 2 is 100% unsolvable for any brute-force attempted solutions.

My solution is fully based on [this code](https://github.com/jonathanpaulson/AdventOfCode/blob/master/2023/12.py) by competitive programmer Jonathan Paulson.\
He uses dynamic programming and memoization (both new concepts to me) but are usually implemented to solve nonograms-like problems.\
I think a good way to approach these concepts is by first looking at the [Fibonacci sequence](https://en.wikipedia.org/wiki/Dynamic_programming#Fibonacci_sequence) example.

If I understand this correctly, it is all about defining sub-problems and attacking them first.\
Solving the sub-problems according to the set of rules and then build solutions keeping only the ones that still follow the set of rules.\
But, ..I am still wrapping my head around this..

### Day 13

[Change in requirements problem](https://en.wikipedia.org/wiki/Scrum_(software_development)#Sprint_planning)

Well, not exactly the theme in particular for this puzzle (it happens for every day after solving part 1).\
This day was just a good example because of how it impacted my solution.\
What i did for part 1 was converting every line in the input to one single number.\
A hashtag '#' becomes '1' and a period '.' becomes '0' which I can represent as a binary number.\
I then converted the binary to an int32.\

Following the above, these lines.

```
.#...##..
#.#######
....####.
..##.##.#
..##.##.#
....####.
#.#######
```

become this array of integers

```
140, 383, 30, 109, 109, 30, 383
```

I wanted to be clever and compare numbers instead of comparing strings.\
However, for part 2 we are tasked with finding a reflection which has exactly one mismatch (smudge on the mirror).\
My implementation for part 1 was not well suited for the change in requirements for part 2.\
In part 2 this will not work because we have to find the one irregular character, '.' or '#'.\
I guess I could convert the int32 back to base 2 and then compare each '1' and '0',\
but that would defeat the whole purpose of creating the integers in the first place.\
I decided to leave it as is for part 1 and then do a new implementation for part 2.


### Day 18 Part 2

[Area of a Polygon problem](https://en.wikipedia.org/wiki/Shoelace_formula)

The bruteforce attempt from part 1 will not suffice for part 2 as we expect a 15 digit answer.\
I did read about other programmers using coordinate compression (allows to solve with bruteforce).\
While there are multiple ways of going about this problem, I decided to use the [shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula#Example).\
A very cool math problem.


### Day 19 Part 2

[A range problem](https://advent-of-code.xavd.id/writeups/2023/day/19/#part-2)

Bruteforcing all values 4000^4 would never work.\
We compute the ranges instead.\
The link above seemed to have a better explanation (and understanding) than me.\
Anyway, I am very proud of the code I wrote and the use of recursion to solve both part 1 and part 2.
