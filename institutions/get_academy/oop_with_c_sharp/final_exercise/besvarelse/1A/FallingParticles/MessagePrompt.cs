namespace FallingParticles;

internal class MessagePrompt : IComponents
{
    private List<string> _messages = [];
    private string _prompt_enter = "-Press Enter-";
    private int _positionmid_ = Console.WindowWidth / 2;

    public MessagePrompt()
    {
        // will be printed when game starts
        _messages.Add("Welcome to Falling Particles");
        _messages.Add("----------------------------");
        _messages.Add("Left Arrow = Move Left");
        _messages.Add("Right Arrow = Move Right");
        _messages.Add("P = Pause");
        _messages.Add("Q = Quit");
    }

    public void DrawComponent()
    {
        if (_messages.Any())
        {
            Console.Clear();
            int y_position = Console.WindowHeight / 3;
            foreach (string msg in _messages)
            {
                Console.SetCursorPosition(_positionmid_ - msg.Length / 2, y_position);
                Console.Write(msg);
                y_position += 2;
            }

            Console.SetCursorPosition(_positionmid_ - _prompt_enter.Length / 2, y_position + 2);
            Console.Write(_prompt_enter);

            Console.ReadLine();

            // deletes all messages and start with new list
            _messages = new List<string>();
        }
    }

    public void ResetComponent()
    {
        _messages = new List<string>();
    }

    public void Add(string msg)
    {
        _messages.Add(msg);
    }
}
