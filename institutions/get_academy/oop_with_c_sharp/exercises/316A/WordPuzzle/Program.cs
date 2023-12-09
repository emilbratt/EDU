namespace WordPuzzle;
class Program
{
    private static readonly string _language = "Norwegian";
    public static readonly int WordPuzzleCount = 200;
    public static readonly int MaxOverlap = 5;
    public static readonly int MinOverlap = 3;
    public static readonly bool _overlap_must_be_a_word = true;
    static void Main()
    {
        var wordlist = new WordLists(_language);

        var puzzle = new Puzzle(wordlist, MaxOverlap, MinOverlap, WordPuzzleCount, _overlap_must_be_a_word);

        string[] word_puzzles = puzzle.GetPuzzleWords();

        foreach (string result in word_puzzles)
        {
            Console.WriteLine(result + "\n");
        }
    }
}
