namespace CoinApplication;

class Bank
{
    private Coins[] _coins;

    public Bank(Coins[] coins)
    {
        _coins = coins;
    }

    private IEnumerable<int> ICoinCount(int coinType)
    {
        foreach (Coins coin in _coins)
        {
            if (coin.GetCoinType() == coinType) yield return coin.GetCoinCount();
        }
    }

    public void AddCoins(Coins coins)
    {
        var _coinsList = _coins.ToList();
        _coinsList.Add(coins);
        _coins = _coinsList.ToArray();
    }

    public void RemoveCoins(int coinType)
    {
        var newCoinsList = new List<Coins>();

        foreach (Coins coin in _coins)
        {
            if (coin.GetCoinType() != coinType) newCoinsList.Add(coin);
        }

        _coins = newCoinsList.ToArray();
    }

    public int TotalFunds()
    {
        int total = 0;
        foreach (Coins coin in _coins)
        {
            total += coin.GetTotal();
        }

        return total;
    }

    public int TotalCoinCount()
    {
        int count = 0;
        foreach (Coins c in _coins)
        {
            count += c.GetCoinCount();
        }

        return count;
    }

    public int TotalOfOneType(int coinType)
    {
        int coinCount = 0;
        foreach (int count in ICoinCount(coinType))
        {
            coinCount += count;
        }

        return coinCount * coinType;
    }

    public int CountOfOneType(int coinType)
    {
        int coinCount = 0;
        foreach (int count in ICoinCount(coinType))
        {
            coinCount += count;
        }

        return coinCount;
    }
}
