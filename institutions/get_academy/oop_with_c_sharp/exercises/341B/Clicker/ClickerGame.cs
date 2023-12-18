class ClickerGame
{
    public int Points { get; private set; } = 0;
    private int _pointsPerClick = 1;
    private int _pointsPerClickIncrease = 1;
    public bool Play { get; private set; } = true;

    public ClickerGame()
    {
        Console.WriteLine("ClickerGame");
    }

    public void Click()
    {
        Points += _pointsPerClick;
    }

    public void Upgrade()
    {
        if (Points >= 10)
        {
            Points -= 10;
            _pointsPerClick += _pointsPerClickIncrease;
        }
    }

    public void SuperUpgrade()
    {
        if (Points >= 100)
        {
            Points -= 100;
            _pointsPerClickIncrease++;
        }
    }

    public void ExitGame()
    {
        Play = false;
    }
}
