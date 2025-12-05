// var data = File.ReadAllLines("../../../ExampleData.txt"); // 3
var data = File.ReadAllLines("../../../Data.txt"); // 652

var ranges = new List<(long, long)>();
var ids = new List<long>();

foreach (var line in data)
{
    var l = line.Split("-");
    if (l.Count() == 2)
        ranges.Add((long.Parse(l[0]), long.Parse(l[1])));
    else if (!string.IsNullOrEmpty(line))
        ids.Add(long.Parse(l[0]));
}

ranges.Sort();
ids.Sort();

var sum = 0;
var i = 0;
foreach (var range in ranges)
{
    for (; i < ids.Count(); i++)
    {
        if (range.Item1 <= ids[i] && ids[i] <= range.Item2)
            sum++;
        else if (ids[i] > range.Item2)
            break;
    }
}

Console.WriteLine($"{sum}");