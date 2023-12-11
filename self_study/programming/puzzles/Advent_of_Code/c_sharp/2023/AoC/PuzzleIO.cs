namespace AoC;

using System.IO;

#pragma warning disable CS8604 // Possible null reference argument.

class PuzzleIO
{
    private static string? _day;
    private static string? _part;
    private static string? _path_input;
    private static string? _path_output;

    public PuzzleIO(string day, string part)
    {
        _day = day;
        _part = part;

        string? io_path = AppContext.BaseDirectory;

        // the current working directory might happen to be inside ./bin/Debug/net7.0/AoC
        while (io_path != null)
        {
            if (Directory.Exists(Path.Combine("/", io_path, "AoC")))
            {
                _path_input = Path.Combine("/", io_path, "AoC", "Input");
                _path_output = Path.Combine("/", io_path, "AoC", "Output");
                break;
            }

            io_path = Path.GetDirectoryName(io_path);

        }
        if (io_path == null)
        {
            Console.WriteLine("Could not find directory 'AoC'");
            Environment.Exit(1);
        }
    }

    public string In()
    {
        string res = String.Empty;
        string input_file = Path.Combine(_path_input, _day);

        try
        {
            res = File.ReadAllText(input_file);
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Day {_day} no input file: {ex.Message}");
        }

        return res;
    }

    public void Out(string result)
    {
        string output_file = Path.Combine(_path_output, _day + "." + _part);

        try
        {
            Directory.CreateDirectory(_path_output); // output directory might not exist

            File.WriteAllText(output_file, result);
            Console.WriteLine($"Day {_day} output: {output_file}");

        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
    }
}
