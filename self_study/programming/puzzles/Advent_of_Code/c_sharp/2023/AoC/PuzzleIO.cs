namespace AoC;

using System.IO;

class PuzzleIO
{
    private static string _path_input = String.Empty;
    private static string _path_output = String.Empty;

    public PuzzleIO()
    {
        string? path = AppContext.BaseDirectory;

        // find directory named AoC by jumping one directory up every time
        while (path != null)
        {
            // try to load AoC directory
            if (Directory.Exists(Path.Combine("/", path, "AoC")))
            {
                _path_input = Path.Combine("/", path, "AoC", "Input");
                _path_output = Path.Combine("/", path, "AoC", "Output");
                // _day = day;
                // _part = part;
                return;
            }

            // jump one directory up and try again
            path = Path.GetDirectoryName(path);
        }

        Console.WriteLine("PuzzleIO Error: Failed to find directory 'AoC'");
        Environment.Exit(1);
    }

    public string Input(string day)
    {
        string input_file = Path.Combine(_path_input, day);

        try
        {
            return File.ReadAllText(input_file);
        }
        catch (Exception)
        {
            return String.Empty;
        }
    }

    public string Output(string output, string day, string part)
    {
        if (output == String.Empty) return "No output data";

        string output_file = Path.Combine(_path_output, day + "." + part);

        try
        {
            Directory.CreateDirectory(_path_output); // output directory might not exist
            File.WriteAllText(output_file, output + '\n'); // we add a "newline"-character for easy concat/read
            return output_file;
        }
        catch (Exception ex)
        {
            return ex.Message;
        }
    }
}
