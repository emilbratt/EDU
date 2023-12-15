Ta utgangspunkt i denne koden: github.com/GetAcademy/ConsoleStarsTilOppgaveInterface

Programmet viser to typer stjerner.
    - En type som blinker
    - En type som flytter seg.

Skriv om koden ved å innføre et interface.
Forløkken i hovedprogrammet skal kunne forenkles til dette:

```c#
foreach (var star in stars)
{
    star.Show();
    star.Update();
}
```
