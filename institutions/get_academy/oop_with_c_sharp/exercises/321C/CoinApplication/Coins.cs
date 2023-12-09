namespace CoinApplication;

class Coins
{
    private readonly int _coinType;
    private int _coinCount;

    public Coins(int value, int count)
    {
        _coinType = value;
        _coinCount = count;
    }

    public int GetCoinType()
    {
        return _coinType;
    }

    public int GetCoinCount()
    {
        return _coinCount;
    }

    public int GetTotal()
    {
        return _coinType * _coinCount;
    }
}
