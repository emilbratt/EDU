namespace TicTacToe;
class Square
{
    private string _value;

    public Square()
    {
        _value = " ";
    }

    public string Value()
    {
        return _value;
    }

    public bool Mark(bool for_player_one)
    {
        if (_value != " ") return false;

        _value = for_player_one ? "X" : "O";
        return true;
    }
}
