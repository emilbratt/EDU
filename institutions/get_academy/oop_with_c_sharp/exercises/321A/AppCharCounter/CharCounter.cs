class CharCounter
{
    private string _text;
    private int[] _counts;
    private readonly int _range = 250;
    public CharCounter(string text)  {
        _text = text;
        _counts = new int[_range];
    }

    public string GetText()
    {
        return _text;
    }
        
    public void AddText()
    {
        Console.Write("Type in something: ");
        _text = Console.ReadLine()?? "";
        foreach (var character in _text ?? string.Empty)
        {
            _counts[(int)character]++;
        }
    }

    public void ShowCounts()
    {
        for (var i = 0; i < _range; i++)
        {
            if (_counts[i] == 0) continue;

            char character = (char)i;
            Console.WriteLine(character + " - " + _counts[i]);
        }
    }
}
