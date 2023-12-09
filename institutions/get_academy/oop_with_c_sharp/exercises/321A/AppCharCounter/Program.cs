var counter = new CharCounter("something");
while (!string.IsNullOrWhiteSpace(counter.GetText()))
{
    counter.AddText();
    counter.ShowCounts();
}

