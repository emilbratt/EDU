class CommandSet : ICommand
{
    private readonly ICommand[] _commands;
    private char _commandClick = 'C';
    private char _commandUpgrade = 'K';
    private char _commandSuperUpgrade = 'S';
    private char _commandExitGame = 'X';

    public CommandSet(ClickerGame game)
    {
        _commands = new ICommand[]
        {
            new CmdClick(game, _commandClick),
            new CmdUpgrade(game, _commandUpgrade),
            new CmdSuperUpgrade(game, _commandSuperUpgrade),
            new CmdExitGame(game, _commandExitGame),
        };
    }

    public void ShowCommand()
    {
        foreach (var command in _commands)
        {
            command.ShowCommand();
        }
    }

    public void RunCommand(char character)
    {
        foreach (var command in _commands)
        {
            command.RunCommand(character);
        }
    }
}
