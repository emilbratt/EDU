class CmdSuperUpgrade : ICommand
{
    private ClickerGame _game;
    private readonly char _character;

    public CmdSuperUpgrade(ClickerGame game, char character)
    {
        _game = game;
        _character = character;
    }

    public void ShowCommand()
    {
        Console.WriteLine($"{_character}: Super Upgrade");
    }

    public void RunCommand(char character)
    {
        if (character == _character) _game.SuperUpgrade();
    }
}
