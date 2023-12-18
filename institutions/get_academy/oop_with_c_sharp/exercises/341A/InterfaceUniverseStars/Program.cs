namespace ConsoleStars;

class Program
{      
    static void Main(string[] args)
    {
        // NotUsingInterface();
        UsingInterface();
    }

    public static void NotUsingInterface()
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

    public static void UsingInterface()
    {
        var random = new Random();

        // MovableStar and PhasesStar both inherit the interface IStars so we can make an array of these
        var stars = new IStars[]
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
                star.Show();
                star.Update();
            }
            Console.CursorLeft = 0;
            Console.CursorTop = 0;
            Thread.Sleep(200);
        }
    }
}
