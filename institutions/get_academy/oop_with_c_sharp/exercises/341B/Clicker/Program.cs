var game = new ClickerGame();
var commands = new CommandSet(game);

while (game.Play)
{
    Console.Clear();

    commands.ShowCommand();

    Console.WriteLine($"Points: {game.Points}");

    char cmd = Console.ReadKey().KeyChar;

    commands.RunCommand(char.ToUpper(cmd));
}

Console.Clear();
