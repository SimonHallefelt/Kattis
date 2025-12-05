// var data = File.ReadAllLines("../../../ExampleData.txt"); // 14
var data = File.ReadAllLines("../../../Data.txt"); // 341753674214273

var ranges = new List<(long, long)>();
var ids = new List<long>();

foreach (var line in data)
{
    var l = line.Split("-");
    if (l.Count() == 2)
        ranges.Add((long.Parse(l[0]), long.Parse(l[1])));
    else if (!string.IsNullOrEmpty(line))
        break;
}

ranges.Sort();

long sum = 0;
for (var i = 0; i+1 < ranges.Count(); i++)
{
    if (ranges[i+1].Item1 <= ranges[i].Item2)
        ranges[i+1] = (ranges[i].Item1, Math.Max(ranges[i].Item2, ranges[i+1].Item2));
    else
        sum += 1 + ranges[i].Item2 - ranges[i].Item1;
    if (i + 2 == ranges.Count())
        sum += 1 + ranges[i+1].Item2 - ranges[i+1].Item1;
    
    Console.WriteLine($"{sum}, {ranges[i]}, {ranges[i+1]}");
}

Console.WriteLine($"{sum}");