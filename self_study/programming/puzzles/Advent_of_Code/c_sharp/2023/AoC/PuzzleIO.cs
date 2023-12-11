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

        string? base_path = AppContext.BaseDirectory;
        while (base_path != null)
        {
            if (Directory.Exists(Path.Combine("/", base_path, "AoC")))
            {
                _path_input = Path.Combine("/", base_path, "AoC", "Input");
                _path_output = Path.Combine("/", base_path, "AoC", "Output");
                break;
            }

            base_path = Path.GetDirectoryName(base_path);
        }

        if (base_path == null)
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

    public string Out(string result)
    {
        string output_file = Path.Combine(_path_output, _day + "." + _part);

        try
        {
            Directory.CreateDirectory(_path_output); // output directory might not exist

            File.WriteAllText(output_file, result);

        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }

        return output_file;
    }
}
