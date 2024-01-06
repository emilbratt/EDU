using Recipes.Ingredients;

namespace Recipes
{
    public interface IRecipe
    {
        IIngredient[] Ingredients { get; }
        string Name { get; }
        float Price { get; }
        float Qty { get; set; }

        IIngredient? GetIngredient(string name);
        void PrintIngredients();
        void PrintInstruction();
    }
}
