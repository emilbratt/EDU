namespace Recipes.Ingredients
{
    class Water : IIngredient
    {
        private float _amount;
        private string _name = "Water";
        private float _price = 0;
        private string _unit = "l.";

        public Water(float amount)
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
