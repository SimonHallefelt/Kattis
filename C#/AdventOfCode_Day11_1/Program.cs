// var data = File.ReadAllLines("../../../ExampleData.txt"); // 5
var data = File.ReadAllLines("../../../Data.txt"); // 658

var map = new Dictionary<string, List<string>>();
foreach (var line in data)
{
    var split = line.Split(": ").ToList();
    map[split.First()] = split.Last().Split(" ").ToList();
}

var start = "you";
var end = "out";
var ends = new Dictionary<string, long>();
var result = dfs(start, end);
long dfs(string pos, string target)
{
    if (ends.ContainsKey(pos))
        return ends[pos];
    var sum = 0L;
    foreach(var path in map[pos])
    {
        sum += path.Equals(target) ? 1 : dfs(path, target);
    }
    ends[pos] = sum;
    return sum;
}

Console.WriteLine($"{result}");