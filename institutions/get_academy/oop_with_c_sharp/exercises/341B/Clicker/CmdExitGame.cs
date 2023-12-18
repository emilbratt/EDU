class CmdExitGame : ICommand
{
    private ClickerGame _game;
    private readonly char _character;

    public CmdExitGame(ClickerGame game, char character)
    {
        _game = game;
        _character = character;
    }

    public void ShowCommand()
    {
        Console.WriteLine($"{_character}: Exit");
    }

    public void RunCommand(char character)
    {
        if (character == _character) _game.ExitGame();
    }
}
