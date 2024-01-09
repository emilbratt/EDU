namespace FallingParticles;

internal class Components(params IComponents[] components)
{
    private readonly IComponents[] _components = components;

    public void Draw()
        => Array.ForEach(_components, c => c.DrawComponent());

    public void Reset()
        => _components.AsParallel().ForAll(c => c.ResetComponent());
}
