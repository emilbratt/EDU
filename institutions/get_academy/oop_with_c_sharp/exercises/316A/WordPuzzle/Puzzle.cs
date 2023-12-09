namespace WordPuzzle;
class Puzzle
{
    private static readonly int _min_word_length = 7;
    private static readonly int _max_word_length = 10;
    private int _word_puzzle_count = 200;
    private readonly int _max_overlap = 5;
    private readonly int _min_overlap = 3;
    private readonly bool _overlap_must_be_a_word = true;
    private readonly Random _rnd = new();
    private readonly WordLists _wordlist;
    private readonly string[] _word_puzzles;

    public Puzzle(WordLists wordlist,
                  int max_overlap_length,
                  int min_overlap_length,
                  int word_puzzle_count,
                  bool overlap_must_be_a_word)
    {
        _word_puzzles = new string[_word_puzzle_count];

        _max_overlap = max_overlap_length;
        _min_overlap = min_overlap_length;
        _word_puzzle_count = word_puzzle_count;
        _overlap_must_be_a_word = overlap_must_be_a_word;

        _wordlist = wordlist;
    }

    private string? GetNext(string current_word)
    {
        foreach (string new_word in _wordlist.AllWords())
        {
            if (new_word.Length < _min_word_length) continue;
            if (new_word.Length > _max_word_length) continue;

            int len = WordLib.CountShortestOverlap(_min_overlap, _max_overlap, current_word, new_word);

            if (len >= _min_overlap)
            {
                string part_that_overlaps = WordLib.ExtractShortestOverlappingPart(_min_overlap, current_word, new_word);

                string result = $"{current_word} - {new_word} -> {part_that_overlaps} ({len})";

                if(_overlap_must_be_a_word)
                {
                    if (_wordlist.WordExists(part_that_overlaps))
                    {
                        return result;
                    }
                }
                else
                {
                    return result;
                }

            }
        }

        return null;
    }

    public string[] GetPuzzleWords()
    {
        while (_word_puzzle_count > 0)
        {
            int random_index = _rnd.Next(0, _wordlist.WordCount());

            string random_word = _wordlist.GetWordByIndex(random_index);

            string? res = GetNext(random_word);
            if (res == null) continue;

            _word_puzzles[_word_puzzle_count-1] = res;
            _word_puzzle_count--;
        }

        return _word_puzzles;
    }
}
