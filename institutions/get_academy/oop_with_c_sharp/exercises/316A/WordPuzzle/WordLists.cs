using System.IO.Compression;
using System.Text;

namespace WordPuzzle;
class WordLists
{
    private static readonly string WordListZipFile = "wordlists.zip";
    private readonly string[] _wordlist;

    public WordLists(string language)
    {
        var filename = language.ToLower() + ".txt";
        var wordlist = new List<string>();

        try
        {
            // ensure the zip file exists
            if (!File.Exists(WordListZipFile))
            {
                Console.WriteLine($"Zip file '{WordListZipFile}' not found");
                Environment.Exit(1);
            }

            // extract the zip file
            using (ZipArchive archive = ZipFile.OpenRead(WordListZipFile))
            {
                // get the specified wordlist (text file)
                ZipArchiveEntry? entry = archive.GetEntry(filename);

                if (entry == null)
                {
                    Console.WriteLine($"File '{filename}' not found in the zip archive.");
                    Environment.Exit(1);
                }

                // read contents of text file
                using (Stream stream = entry.Open())
                using (StreamReader reader = new StreamReader(stream, Encoding.UTF8))
                {
                    // add words to list
                    string last_word = "";

                    while (!reader.EndOfStream)
                    {
                        string? line = reader.ReadLine();
                        if (line == null) continue;

                        string  word = line.Split('\t')[1];
                        if (word == last_word)  continue;
                        if (word.Contains('-')) continue;
                        if (word.Contains(' ')) continue;

                        wordlist.Add(word);
                        last_word = word;
                    }
                }
            }
        }
        catch (Exception e)
        {
            Console.WriteLine(e.Message);
            Environment.Exit(1);
        }

        if (!wordlist.Any())
        {
            Console.WriteLine("Something unextepcted happened when loading the wordlist (possible empty list)");
            Environment.Exit(1);
        }

        _wordlist = wordlist.ToArray();
    }

    public int WordCount()
    {
        return _wordlist.Length;
    }

    public string GetWordByIndex(int index)
    {
        return _wordlist[index];
    }

    public IEnumerable<string> AllWords()
    {
        foreach (string word in _wordlist) yield return word;
    }

    public bool WordExists(string word)
    {
        return _wordlist.Contains(word);
    }
}
