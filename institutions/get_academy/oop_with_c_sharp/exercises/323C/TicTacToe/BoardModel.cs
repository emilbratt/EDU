namespace TicTacToe;
class BoardModel
{
    private static readonly int _size = 9;
    private static readonly int _row_total = 3;
    private static readonly int _col_total = 3;
    private readonly Random _random = new Random();
    private Square[] _squares = new Square[_size];

    public BoardModel()
    {
        for (int i = 0; i < _size; i++)
        {
            _squares[i] = new Square();
        }
    }

    private int GetSquareIndex(int row, int col)
    {
        // rows and columns to start from 1, we adjust for that
        row--;
        col--;

        return (row * _col_total) + col;
    }

    public IEnumerable<int> Rows()
    {
        // iterating over the boards rows
        int row = 1;
        while (row <= _row_total)
            yield return row++;
    }

    public IEnumerable<int> Columns()
    {
        // iterating over the boards collumns
        int col = 1;
        while (col <= _col_total)
            yield return col++;
    }

    public int Width()
    {
        return _col_total;
    }

    public string GetSquareValue(int row, int col)
    {
        return _squares[GetSquareIndex(row, col)].Value();
    }

    public bool AvailableSquares()
    {
        for (int i = 0; i < _row_total * _col_total; i++)
        {
            if (_squares[i].Value() == " ") return true;
        }
        return false;
    }

    public bool CheckWin(bool player_one)
    {
        string mark = player_one ? "X" : "O";

        bool found;
        string first_m;
        string last_m;

        // check west -> east
        foreach (int row in Rows())
        {
            first_m = _squares[GetSquareIndex(row, 1)].Value();
            last_m  = _squares[GetSquareIndex(row, _col_total)].Value();
            if (first_m != mark || last_m != mark) continue;

            found = true;
            foreach (int col in Columns())
            {
                string m  = _squares[GetSquareIndex(row, col)].Value();
                if (mark != m) found = false;
            }
            if (found) return true;
        }

        // check north -> south
        foreach (int col in Columns())
        {
            first_m  = _squares[GetSquareIndex(1, col)].Value();
            last_m   = _squares[GetSquareIndex(_row_total, col)].Value();
            if (first_m != mark || last_m != mark) continue;

            found = true;
            foreach (int row in Rows())
            {
                string m  = _squares[GetSquareIndex(row, col)].Value();
                if (mark != m) found = false;
            }
            if (found) return true;
        }

        // check north west -> south east
        first_m  = _squares[GetSquareIndex(1, 1)].Value();
        last_m   = _squares[GetSquareIndex(_row_total, _col_total)].Value();
        if (first_m == mark || last_m == mark)
        {
            found = true;
            foreach (int row in Rows())
            {
                string m  = _squares[GetSquareIndex(row, row)].Value();
                if (mark != m) found = false;
            }
            if (found) return true;
        }

        // check diagonal north east -> south west
        first_m  = _squares[GetSquareIndex(1, _col_total)].Value();
        last_m   = _squares[GetSquareIndex(_row_total, 1)].Value();
        if (first_m == mark || last_m == mark)
        {
            found = true;
            foreach (int col in Columns())
            {
                string m  = _squares[GetSquareIndex(col, _col_total - col + 1)].Value();
                if (mark != m) found = false;
            }
            if (found) return true;
        }

        return false;
    }

    public bool MartPosition(bool player_one, string position)
    {
        if (position.Length < 2) return false;

        int col = position[0] switch
        {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => 0,
        };

        int row = position[1] switch
        {
            '1' => 1,
            '2' => 2,
            '3' => 3,
            _ => 0,
        };

        if (row == 0 || col == 0) return false;

        return _squares[GetSquareIndex(row, col)].Mark(player_one);
    }

    public bool MarkRandomPosition(bool player_one)
    {
        int row = _random.Next(1, _row_total+1);
        int col = _random.Next(1, _col_total+1);

        return _squares[GetSquareIndex(row, col)].Mark(player_one);
    }
}
