namespace ConsoleStars;

class Program
{      
    static void Main(string[] args)
    {
        if (args.Length == 1)
        {
            if (args[0] == "old") Old();
            if (args[0] == "new") New();
        }
        Console.WriteLine("Pass argument: 'old' | 'new'");
    }

    public static void Old()
    {
        var random = new Random();
        var stars = new object[]
        {
            new PhasesStar(random),
            new PhasesStar(random),
            new PhasesStar(random),
            new MovableStar(random),
            new MovableStar(random),
            new MovableStar(random),
        };
        while (true)
        {
            Console.Clear();     
            foreach (var star in stars)
            {
                if (star is PhasesStar)
                {
                    var phasesStar = (PhasesStar)star;
                    phasesStar.Show();
                    phasesStar.Update();
                }
                else if (star is MovableStar)
                {
                    var phasesStar = (MovableStar)star;
                    phasesStar.Show();
                    phasesStar.Update();
                }
            }
            Console.CursorLeft = 0;
            Console.CursorTop = 0;
            Thread.Sleep(200);
        }
    }

    public static void New()
    {
        var random = new Random();
        var stars = new object[]
        {
            new PhasesStar(random),
            new PhasesStar(random),
            new PhasesStar(random),
            new MovableStar(random),
            new MovableStar(random),
            new MovableStar(random),
        };
        while (true)
        {
            Console.Clear();     
            foreach (var star in stars)
            {
                // not implemented yet..
                // star.Show();
                // star.Update();
            }
            Console.CursorLeft = 0;
            Console.CursorTop = 0;
            Thread.Sleep(200);
        }
    }
}
