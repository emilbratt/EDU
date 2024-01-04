namespace Recipes
{
    public interface IRecipe
    {
        string Name();
        float Price();
        string Steps();
        void PrintIngredients();
    }
}
