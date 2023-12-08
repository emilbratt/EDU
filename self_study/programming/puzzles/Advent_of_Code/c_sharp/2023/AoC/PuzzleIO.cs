namespace AoC;

using System.IO;

class PuzzleIO
{
    #pragma warning disable CS8604 // Possible null reference argument.
    private static string? _day;
    private static string? _part;

    public PuzzleIO(string day, string part)
    {
        _day = day;
        _part = part;
    }

    public string[] ReadInput()
    {
        string[] res = Array.Empty<string>();
        string path = Path.Combine(Path.Combine("AoC", "Input"), _day);

        try
        {
            res =  File.ReadAllLines(path);
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
            Environment.Exit(1);
        }

        return res;
    }

    public string In()
    {
        string res = String.Empty;
        string path = Path.Combine(Path.Combine("AoC", "Input"), _day);

        try
        {
            res = File.ReadAllText(path);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Day {_day} no input file: {ex.Message}");
        }

        return res;
    }

    public void Out(string result)
    {
        string path_root = Path.Combine("AoC", "Output");
        string path = Path.Combine(Path.Combine("AoC", "Output"), _day + "." + _part);

        try
        {
            Directory.CreateDirectory(path_root); // output directory might not exist

            File.WriteAllText(path, result);
            Console.WriteLine($"Day {_day} output: {path}");

        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
    }
}
