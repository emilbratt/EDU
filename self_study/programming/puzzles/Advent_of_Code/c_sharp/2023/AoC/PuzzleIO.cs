
namespace AoC;

using System.IO;

class PuzzleIO
{
    private readonly string _path_input_root = Path.Combine("AoC", "Input");
    private readonly string _path_output_root = Path.Combine("AoC", "Output");
    
    private static string? _path_input_file;
    private static string? _path_output_file;

    public PuzzleIO(string day, string part)
    {
        string file_name_in = day;
        _path_input_file = Path.Combine(_path_input_root, file_name_in);

        string file_name_out = day + "." + part;
        _path_output_file = Path.Combine(_path_output_root, file_name_out);
    }

    public string[] ReadInput()
    {
        if (_path_input_file == null) Environment.Exit(1);

        string[] res = Array.Empty<string>();
        try
        {
            res =  File.ReadAllLines(_path_input_file);
        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
            Environment.Exit(1);
        }

        return res;
    }

    public void WriteOutput(string result)
    {
        if (_path_output_file == null) Environment.Exit(1);

        try
        {
            Directory.CreateDirectory(_path_output_root); // output directory might not exist

            File.WriteAllText(_path_output_file, result);
            Console.WriteLine($"Puzzle output: {_path_output_file}");

        }
        catch (Exception ex)
        {
            Console.WriteLine(ex.Message);
        }
    }
}
