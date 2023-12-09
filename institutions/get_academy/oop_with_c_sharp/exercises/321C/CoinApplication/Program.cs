namespace CoinApplication;

class Program
{
    static void Main()
    {
        // lets make an array of different coins
        var myCoins = new Coins[]
        {
            new Coins(1, 8),
            new Coins(5, 7),
            new Coins(10, 4),
            new Coins(20, 2),
            new Coins(5, 10),
        };

        // inserting our array of coins into the bank and get a bank object
        var bank = new Bank(myCoins);

        // we can use the bank object to get som information
        int total = bank.TotalFunds();
        int count = bank.TotalCoinCount();

        Console.WriteLine($"The total value of all our coins: {total}");
        Console.WriteLine($"The total count of all our coins: {count}");


        // lets get info about our twenties
        int countTwenties = bank.CountOfOneType(20);
        int totalTwenties = bank.TotalOfOneType(20);

        Console.WriteLine($"The total count of our twenties: {countTwenties}");
        Console.WriteLine($"The total value of our twenties: {totalTwenties}");


        // we can also add extra coins, lets say we add 3 extra twenties
        bank.AddCoins(new Coins(20, 3));

        // ..and then get updated information after
        int countTwentiesAfter = bank.CountOfOneType(20);
        int totalTwentiesAfter = bank.TotalOfOneType(20);

        Console.WriteLine($"The total count of our twenties after adding more: {countTwentiesAfter}");
        Console.WriteLine($"The total value of our twenties after adding more: {totalTwentiesAfter}");


        // or maybe we remove all twenties
        bank.RemoveCoins(20);

        // ..and then get updated information
        int countTwentiesAfterRemoval = bank.CountOfOneType(20);
        int totalTwentiesAfterRemoval = bank.TotalOfOneType(20);

        Console.WriteLine($"The total count of our twenties after removing: {countTwentiesAfterRemoval}");
        Console.WriteLine($"The total value of our twenties after removing: {totalTwentiesAfterRemoval}");
    }
}
