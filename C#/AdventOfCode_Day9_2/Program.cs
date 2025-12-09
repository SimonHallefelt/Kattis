// var data = File.ReadAllLines("../../../ExampleData.txt"); // 24
var data = File.ReadAllLines("../../../Data.txt"); // 1588990708

var positions = new List<(long, long)>();
var rowMax = 0L;
var rowMin = 1000000L;
var colMax = 0L;
var colMin = 1000000L;
var lowestPos = (long.MaxValue, long.MaxValue);
foreach (var line in data)
{
    var split = line.Split(",").Select(long.Parse).ToList();
    positions.Add((split[0], split[1]));
    rowMax = Math.Max(rowMax, split[0]);
    rowMin = Math.Min(rowMin, split[0]);
    colMax = Math.Max(colMax, split[1]);
    colMin = Math.Min(colMin, split[1]);
    if (split[0] < lowestPos.Item1)
        lowestPos = positions.Last();
}
rowMax += 2;
rowMin -= 2;
colMax += 2;
colMin -= 2;

Console.WriteLine($"get all distances");
var distances = new List<(long, (long, long), (long, long))>();
var positionsHash = positions.ToHashSet();
for (var i = 0; i + 1 < positions.Count(); i++)
{
    for (var j = i + 1; j < positions.Count(); j++)
    {
        var pos1 = positions[i];
        var pos2 = positions[j];
        if (positionsHash.Any(p => PosInside(p, pos1, pos2)))
            continue;
        var distance = (Math.Abs(pos1.Item1 - pos2.Item1) + 1) * (Math.Abs(pos1.Item2 - pos2.Item2) + 1);
        distances.Add((distance, pos1, pos2));
    }
}
bool PosInside((long, long) pos, (long, long) c1, (long, long) c2)
{
    if (pos.Item1 <= Math.Min(c1.Item1, c2.Item1) || Math.Max(c1.Item1, c2.Item1) <= pos.Item1)
        return false;
    if (pos.Item2 <= Math.Min(c1.Item2, c2.Item2) || Math.Max(c1.Item2, c2.Item2) <= pos.Item2)
        return false;
    return true;
}
distances = distances.OrderBy(d => d.Item1).ToList();
distances.Reverse();

Console.WriteLine($"make border");
var border = new HashSet<(long, long)>();
for (var i = 0; i < positions.Count(); i++)
{
    var pos1 = positions[i];
    var pos2 = positions[(i + 1) % positions.Count()];
    border.UnionWith(PosRange(pos1, pos2));
}
HashSet<(long,long)> PosRange((long,long) pos1, (long,long) pos2)
{
    var range = new HashSet<(long, long)>();
    var yDiff = Math.Abs(pos1.Item1 - pos2.Item1);
    var xDiff = Math.Abs(pos1.Item2 - pos2.Item2);
    var start = yDiff == 0 ? Math.Min(pos1.Item2, pos2.Item2) : Math.Min(pos1.Item1, pos2.Item1);
    var end = start + (yDiff == 0 ? xDiff : yDiff);
    for (var j = start; j <= end; j++)
    {
        range.Add(yDiff == 0 ? (pos1.Item1, j) : (j, pos1.Item2));
    }
    return range;
}

Console.WriteLine($"find outBorder");
var outBorder = new HashSet<(long, long)>();
var visitedOld = new HashSet<(long, long)> { (lowestPos.Item1-1, lowestPos.Item2)};
var directions = new List<(long, long)> { (1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (-1, 1), (-1, -1), (1, -1) };
while (visitedOld.Count() > 0)
{
    var visited = new HashSet<(long, long)>();
    foreach (var v in visitedOld)
    {
        var sideOfBorder = false;
        var visited2 = new HashSet<(long, long)>();
        foreach (var d in directions)
        {
            var r = v.Item1 + d.Item1;
            var c = v.Item2 + d.Item2;
            if (border.Contains((r, c)))
            {
                outBorder.Add(v);
                sideOfBorder = true;
                continue;
            }
            if (outBorder.Contains((r, c)))
                continue;
            visited2.Add((r, c));
        }
        if (sideOfBorder)
            visited.UnionWith(visited2);
    }
    visitedOld = visited;
}


Console.WriteLine($"find result");
foreach (var distance in distances)
{
    var pos1 = distance.Item2;
    var pos2 = distance.Item3;
    var pos3 = (pos1.Item1, pos2.Item2);
    var pos4 = (pos2.Item1, pos1.Item2);
    var edge = new HashSet<(long, long)>();
    edge.UnionWith(PosRange(pos1, pos3));
    edge.UnionWith(PosRange(pos1, pos4));
    if (outBorder.Overlaps(edge))
        continue;
    Console.WriteLine($"{distance.Item1}");
    break;
}
