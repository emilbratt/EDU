namespace CountWordsInUserInput;
class Program
{
    private static readonly char[] _termination_characters = { ' ', '\t', '\n', '\r', '.', ',', ';', '!' };
    static void Main()
    {
        Console.Write("Write some text and get the total word count: ");
        string test_string = Console.ReadLine() ?? "";
        string[] test_string_arr = test_string.Split(_termination_characters, StringSplitOptions.RemoveEmptyEntries);
        int word_count = test_string_arr.Length;
        Console.WriteLine($"Text input:\n{test_string}\n\nWords total: {word_count}");
    }
}
