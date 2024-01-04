using Recipes.Ingredients;

namespace Recipes;

class Pizza : IRecipe
{
    private int _qty;
    private float _price;
    private string _name = "Pizza";

    private List<IIngredient> _ingredients = new()
    {
        new Flour(0.7f),
        new Water(2),
        new Cheese(0.2f),
    };

    public Pizza(int qty)
    {
        _qty = qty;
        foreach (IIngredient ingredient in _ingredients)
        {
            _price += ingredient.Price() * _qty;
        }
    }

    public void PrintIngredients()
    {
        Console.WriteLine($"Ingredients for {_qty} Pizza");
        foreach (IIngredient ingredient in _ingredients)
        {
            Console.Write(ingredient.Name());
            Console.Write(": ");
            Console.Write(_qty * ingredient.Amount());
            Console.Write(" ");
            Console.Write(ingredient.Unit());
            Console.Write("\n");
        }
    }

    public string Name() => _name;

    public float Price() => _price;
    
    public string Steps()
    {
        // print the steps for making this meal
        return String.Empty;
    }
}
