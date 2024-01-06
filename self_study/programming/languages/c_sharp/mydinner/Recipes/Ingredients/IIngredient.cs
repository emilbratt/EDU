namespace Recipes.Ingredients
{
    public interface IIngredient
    {
        float Amount { get; }
        string Name { get; }
        float Price { get; }
        string Unit { get; }

        string Summary(float qty);
    }
}
