// var data = File.ReadAllLines("../../../ExampleData.txt"); // 2
var data = File.ReadAllLines("../../../Data.txt"); // 593

var rawPresents = new Dictionary<int, List<List<bool>>>();
var areas = new List<(int, int, List<int>)>();
var presentNumber = 0;
foreach (var line in data)
{
    if (line.Count() == 0)
        continue;
    if (line.Contains("x"))
    {
        var split = line.Split(": ").ToList();
        var size = split.First().Split("x").Select(int.Parse).ToList();
        areas.Add((size.First(), size.Last(), split.Last().Split(" ").Select(int.Parse).ToList()));
    }
    else if (line.Contains(":"))
    {
        presentNumber = line.Split(":").Select(int.Parse).First();
        rawPresents[presentNumber] = new List<List<bool>>();
    }
    else
        rawPresents[presentNumber].Add(line.Select(c => c == '#').ToList());
}

var presents = new Dictionary<int, List<List<List<bool>>>>();
foreach(var rawPresent in rawPresents)
{
    var rawShape = rawPresent.Value;
    var all = new List<List<List<bool>>>
    {
        rawShape.Select(l => l.ToList()).ToList(),
        HFlip(rawShape.Select(l => l.ToList()).ToList()),
        VFlip(rawShape.Select(l => l.ToList()).ToList()),
        HFlip(VFlip(rawShape.Select(l => l.ToList()).ToList())),
        Rotate(rawShape.Select(l => l.ToList()).ToList()),
        HFlip(Rotate(rawShape.Select(l => l.ToList()).ToList())),
        VFlip(Rotate(rawShape.Select(l => l.ToList()).ToList())),
        HFlip(VFlip(Rotate(rawShape.Select(l => l.ToList()).ToList()))),
    };
    var seen = new HashSet<string>();
    var allOriontations = new List<List<List<bool>>>();
    foreach(var shape in all)
    {
        if (seen.Add(string.Join("", shape.Select(l => string.Join("", l)))))
            allOriontations.Add(shape);
    }
    presents[rawPresent.Key] = allOriontations;
}
List<List<bool>> HFlip(List<List<bool>> original)
{
    original.Reverse();
    return original;
}
List<List<bool>> VFlip(List<List<bool>> original)
{
    original.ForEach(r => r.Reverse());
    return original;
}
List<List<bool>> Rotate(List<List<bool>> original)
{
    var newShape = Enumerable.Repeat(new List<bool>(), original[0].Count).Select(l => l.ToList()).ToList();
    for(var i = original.Count-1; i >= 0; i--)
    {
        for(var j = 0; j < original[0].Count; j++)
        {
            newShape[j].Add(original[i][j]);
        }
    }
    return newShape;
}

var presentsPos = new Dictionary<int, List<List<(int, int)>>>();
foreach(var present in presents)
{
    var shapes = new List<List<(int, int)>>();
    foreach(var shape in present.Value)
    {
        var positions = new List<(int, int)>();
        for (var i = 0; i < shape.Count; i++)
        {
            for(var j = 0; j < shape[0].Count; j++)
            {
                if (shape[i][j])
                    positions.Add((i, j));
            }
        }
        shapes.Add(positions);
    }
    presentsPos[present.Key] = shapes;
}

var result = 0L;
foreach(var area in areas)
{
    var map = new List<List<bool>>();
    for (var i = 0; i < area.Item1; i++)
    {
        var row = new List<bool>();
        for(var j = 0; j < area.Item2; j++)
        {
            row.Add(false);
        }
        map.Add(row);
    }

    var shapes = new List<int>();
    for (var i = 0; i < area.Item3.Count; i++)
    {
        for (var j = 0; j < area.Item3[i]; j++)
        {
            shapes.Add(i);
        }
    }

    var mustBeTrue = shapes.Sum(s => presents[s][0].Sum(r => r.Sum(b => b ? 1 : 0)));
    var maxEmpty = area.Item1 * area.Item2 - mustBeTrue;
    if (maxEmpty < 0)
        continue;
    var b = Solve(0, 0, map, area.Item3, new HashSet<string>(), maxEmpty, 0);
    result += b ? 1 : 0;
}
bool Solve(int y, int x, List<List<bool>> map, List<int> shapes, HashSet<string> visited, int maxEmpty, int empty)
{
    if (shapes.Sum() == 0)
        return true;
    if (!visited.Add($"{y}|{x}" + string.Join("", map.Select(l => string.Join("", l))) + string.Join(" ", shapes)))
        return false;
    
    for (var r = y; r <= map.Count-3; r++)
    {
        for (var c = r == y ? x : 0; c <= map[0].Count-3; c++)
        {
            if (maxEmpty < empty)
                return false;
            for (var i = 0; i < shapes.Count; i++)
            {
                if (shapes[i] <= 0)
                    continue;
                foreach (var shape in presentsPos[i])
                {
                    var newMap = AddShape(r, c, map, shape);
                    if (newMap == null)
                        continue;
                    var newShapes = shapes.ToList();
                    newShapes[i]--;
                    if (Solve(r, c, newMap, newShapes, visited, maxEmpty, empty))
                        return true;
                }
            }
            empty += map[r][c] ? 0 : 1;
            if (r == map[0].Count - 3)
            {
                empty += map[r+1][c] ? 0 : 1;
                empty += map[r+2][c] ? 0 : 1;
            }
        }
        empty += map[r][map[0].Count-2] ? 0 : 1;
        empty += map[r][map[0].Count-1] ? 0 : 1;
    }

    return false;
}
List<List<bool>>? AddShape(int r, int c, List<List<bool>> map, List<(int, int)> shape)
{
    foreach (var pos in shape)
    {
        if (map[r+pos.Item1][c+pos.Item2])
            return null;
    }
    var newMap = map.Select(row => row.ToList()).ToList();
    foreach (var pos in shape)
    {
        newMap[r+pos.Item1][c+pos.Item2] = true;
    }
    return newMap;
}

Console.WriteLine($"{result}");