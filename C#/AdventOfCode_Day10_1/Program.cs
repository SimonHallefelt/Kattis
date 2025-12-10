// var data = File.ReadAllLines("../../../ExampleData.txt"); // 7
var data = File.ReadAllLines("../../../Data.txt"); // 390

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
    var lightTarget = lightTargets[i];
    var button = buttons[i];
    result += bfs(lightTarget, button);
}
long bfs(List<bool> target, List<List<int>> paths)
{
    var tar = string.Join("", target);
    var startValue = Enumerable.Repeat(false, target.Count()).ToList();
    var visited = new HashSet<string> { string.Join("", startValue) };
    var queue = new Queue<(List<bool>, long)>();
    queue.Enqueue((startValue, 0L));
    while (queue.Count() > 0)
    {
        var q = queue.Dequeue();
        var lights = q.Item1;
        var steps = q.Item2;
        if (string.Join("", lights).Equals(tar))
            return steps;
        foreach(var path in paths)
        {
            var newLights = lights.ToList();
            foreach (var button in path)
            {
                newLights[button] = !newLights[button];
            }
            if (!visited.Add(string.Join("", newLights)))
                continue;
            queue.Enqueue((newLights, steps + 1));
        }
    }
    return -1;
}

Console.WriteLine($"{result}");