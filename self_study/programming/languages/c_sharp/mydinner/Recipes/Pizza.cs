using System.Security.Cryptography.X509Certificates;
using Recipes.Ingredients;

namespace Recipes;

class Pizza() : IRecipe
{
    public IIngredient[] Ingredients { get; } = [
        new Flour(0.7f),
        new Water(0.3f),
        new Cheese(0.5f),
    ];

    public string Name => "Pizza";

    public float Price
        => (from ingredient in Ingredients select ingredient.Price * Qty).Sum();

    public float Qty { get; set; } = 1;

    public IIngredient GetIngredient(string name) => Ingredients.Where(i => i.Name == name).First();

    public void PrintIngredients()
        => Ingredients.AsParallel().ForAll(i => Console.WriteLine(i.Summary(Qty)));


    public void PrintInstruction()
    {
        Console.WriteLine("1. Mix the water with the flour to make pizza dough");
        Console.WriteLine("2. Let dough rise for a couple of hours");
        Console.WriteLine("3. Flatten the dough and stretch it out");
        Console.WriteLine("4. Put cheese on top");
        Console.WriteLine("5. Bake in the oven at 225c for 25 minutes");
        Console.WriteLine("\n--Bon appetit--\n");
    }
}
