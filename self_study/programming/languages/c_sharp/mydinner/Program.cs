using Recipes;

class Program
{
    static void Main(string[] args)
    {
        var pizza = new Pizza();

        Meal(pizza);
    }

    public static void Meal(IRecipe meal)
    {
        Console.WriteLine($"Todays meal: {meal.Name}\n");
        Console.WriteLine("Ingredients");
        meal.PrintIngredients();
        Console.WriteLine("..and the amount changes based on how many we want");

        for (int qty = 3; qty <= 10; qty += 3)
        {
            meal.Qty = qty;
            Console.WriteLine($"\n{meal.Qty} {meal.Name}");
            meal.PrintIngredients();
            Console.WriteLine($"Total for all ingredients: {meal.Price},-");
        }

        Console.WriteLine("\nInstruction:");
        meal.PrintInstruction();

        // // extracting a single ingredient
        // string ingredient = meal.Ingredients[0].Name;
        // var flour = meal.GetIngredient(ingredient);
        // if (flour != null) Console.WriteLine($"{meal.Name} needs some {flour.Unit} of {flour.Name}");
        // else Console.WriteLine($"{meal.Name} does not contain {ingredient}");
    }
}
