// var data = File.ReadAllLines("../../../ExampleData.txt"); // 33
var data = File.ReadAllLines("../../../Data.txt"); // 

var lightTargets = new List<List<bool>>();
var buttons = new List<List<List<int>>>();
var requirements = new List<List<int>>();

foreach (var line in data)
{
    var split = line.Replace("[", "").Replace("]", "").Replace("(", "").Replace(")", "").Replace("{", "").Replace("}", "").Split(" ").ToList();
    lightTargets.Add(split[0].Select(c => c == '#').ToList());
    buttons.Add(split.GetRange(1, split.Count() - 2).Select(s => s.Split(",").Select(int.Parse).ToList()).ToList());
    requirements.Add(split.Last().Split(",").Select(int.Parse).ToList());
}

var result = 0L;
for (int i = 0; i < lightTargets.Count(); i++)
{
    var button = buttons[i];
    var requirement = requirements[i];
    var value = bfs(requirement, button);
    if (value == -1)
        Console.WriteLine($"bfs failed on {i}");
    result += value;
    Console.WriteLine($"{result}, {i}");
}
long bfs(List<int> target, List<List<int>> paths)
{
    var tar = string.Join(" ", target);
    var startValue = Enumerable.Repeat(0, target.Count()).ToList();
    var visited = new HashSet<string> { string.Join(" ", startValue) };
    var queue = new Queue<(List<int>, long)>();
    queue.Enqueue((startValue, 0L));
    while (queue.Count() > 0)
    {
        var q = queue.Dequeue();
        var Counter = q.Item1;
        var steps = q.Item2;
        if (string.Join(" ", Counter).Equals(tar))
        {
            Console.WriteLine($"visited.Count = {visited.Count()}, target = {tar}");
            return steps;
        }
        foreach(var path in paths)
        {
            var newCounter = Counter.ToList();
            var skip = false;
            foreach (var button in path)
            {
                newCounter[button]++;
                if (newCounter[button] > target[button])
                {
                    skip = true;
                    break;
                }
            }
            if (skip)
                continue;
            if (!visited.Add(string.Join(" ", newCounter)))
                continue;
            queue.Enqueue((newCounter, steps + 1));
        }
    }
    return -1;
}

Console.WriteLine($"{result}");