namespace Recipes.Ingredients
{
    public interface IIngredient
    {
        string Name();
        string Unit();
        float Amount();
        float Price();
        void PrintSummary();
    }
}
