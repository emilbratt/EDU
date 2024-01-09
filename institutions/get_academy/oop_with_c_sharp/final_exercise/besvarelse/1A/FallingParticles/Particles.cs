namespace FallingParticles;

internal class Particles : IComponents
{
    private List<Particle> _particles = [];
    private static readonly Random _random = new Random();
    private int _idle_spawn_particle = 45;
    private int _idle_increase_level = 0;
    private int _rounds_between_spawn = 50;
    public int Level = 1;
    public int Score = 0;

    public void DrawComponent()
        => _particles.AsParallel().ForAll(p => p.Draw());

    public void ResetComponent()
    {
        _particles = [];
        _idle_spawn_particle = 45;
        _rounds_between_spawn = 50;
        _idle_increase_level = 0;
        Level = 1;
        Score = 0;
    }

    public void Animate()
    {
        for (int i = _particles.Count - 1; i >= 0; i--)
        {
            Particle particle = _particles[i];

            particle.Y += 0.5f;

            // if particle is below the ground (outside boundary of console view), remove it
            if (particle.Y >= Console.WindowHeight)
            {
                _particles.Remove(_particles[i]);
                Score++;
            }
            else
            {
                _particles[i] = particle;
            }
        }

        if (_idle_increase_level == 100)
        {
            _idle_increase_level = 0;
            Level++;
        }

        if (_idle_spawn_particle >= _rounds_between_spawn)
        {
            Particle particle = new()
            {
                X = _random.Next(0, Console.WindowWidth - 1),
                Y = 1,
            };
            _particles.Add(particle);

            _rounds_between_spawn = 50 / Level;
            _idle_spawn_particle = 0;
        }

        _idle_increase_level++;
        _idle_spawn_particle++;
    }

    public bool Dead( (int x_start, int x_end) t)
    {
        // when game starts, it might not have any particles spawned yet
        if (_particles.Count == 0) return false;

        // this is the particle with the longest travel time (cast the values from float to int)
        (int x, int y) = ( (int)_particles[0].X, (int)_particles[0].Y ) ;

        // check if y indicates that it is still falling (i.e. is alive)
        if (y != Console.WindowHeight - 1) return false;

        // finally we check x and compare it to the x positions if it is caught (i.e. is saved)
        if (x >= t.x_start && x <= t.x_end) return false;

        // if non of the aboce, then the particle is dead (i.e. was not caught)
        return true;
    }
}
