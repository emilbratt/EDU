namespace FallingParticles;

internal static class Game
{
    public static void Play()
    {
        // KEEP ORIGINAL CONSOLE WINDOW SIZE
        int orig_width  = Console.WindowWidth;
        int orig_height = Console.WindowHeight;

        // CONSOLE BOUNDARIES
        Console.WindowWidth = 80;
        Console.CursorVisible = false;

        // GAME COMPONENTS
        ScoreBoard scoreboard = new();
        Particles particles = new();
        Paddle paddle = new();
        MessagePrompt messageprompt = new();

        // COMPONENTS HAVE SHARED LOGIC
        Components components = new(scoreboard, particles, paddle, messageprompt);

        // GAME PAUSE OR QUIT
        bool pause = false;
        bool quit = false;

        // GAME LOOP STARTS HERE
        while (!quit)
        {
            // GAME LOOP SEQUENCE DELAY
            Thread.Sleep(100);

            // GET KEYSTROKE
            var key = Console.KeyAvailable
                    ? Console.ReadKey(true).Key
                    : ConsoleKey.None;

            // KEYSTROKE PAUSE GAME ..this one is a bit fancy ..read comments below for clarification
            pause = pause
                    ? !(key != ConsoleKey.None) // if paused    => un-pause (false) if any key is pressed || else keep pause (true)
                    : (key == ConsoleKey.P);    // if un-paused => pause (true) if key P is pressed       || else keep un-paused (false)
            if (pause) continue;

            // KEYSTROKE QUIT GAME
            quit = key == ConsoleKey.Q;

            // KEYSTROKE MOVE PADDLE
            (int from_x, int to_x) position = paddle.Move(key);

            // LET PARTICLES FALL ONE STEP DOWN (AND MAYBE SPWAN NEW)
            particles.Animate();

            // SCOREBOARD IS UPDATED
            scoreboard.Level = particles.Level;
            scoreboard.Score = particles.Score;

            // DRAW THE CURRENT STATE OF EACH GAME OBJECT
            Console.Clear();
            components.Draw();

            // GAMOVER IF WE DID NOT SAVE A FALLING PARTICLE
            if (particles.Dead(position))
            {
                messageprompt.Add("GameOver");
                messageprompt.Add($"You reached level: {scoreboard.Level}");
                messageprompt.Add($"Score: {scoreboard.Score}");
                Console.Clear();
                components.Draw();
                components.Reset();
            }
        }

        // RESTORE CONSOLE
        Console.Clear();
        Console.SetWindowSize(orig_width, orig_height);
        Console.CursorVisible = true;
    }
}
