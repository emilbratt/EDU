namespace Recipes.Ingredients
{
    class Cheese : IIngredient
    {
        private string _name = "Cheese";
        private float _price = 110;
        private string _unit = "kg.";
        private float _amount;

        public Cheese(float amount)
        {
            _amount = amount;
        }
        public void PrintSummary()
        {
            Console.WriteLine($"{_name}: {_amount} {_unit}.");
        }
        public float Amount()
        {
            return _amount;
        }
        public string Unit()
        {
            return _unit;
        }

        public string Name()
        {
            return _name;
        }

        public float Price()
        {
            return _price * _amount;
        }
    }
}
