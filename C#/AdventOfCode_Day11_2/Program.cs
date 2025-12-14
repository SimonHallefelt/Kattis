// var data = File.ReadAllLines("../../../ExampleData.txt"); // 2
var data = File.ReadAllLines("../../../Data.txt"); // 371113003846800

var map = new Dictionary<string, List<string>>();
foreach (var line in data)
{
    var split = line.Split(": ").ToList();
    map[split.First()] = split.Last().Split(" ").ToList();
}

var start = "svr";
var end = "out";
var ends = new Dictionary<(string, bool, bool), long>();
var result = dfs(start, end, false, false);
long dfs(string pos, string target, bool fft, bool dac)
{
    var fftLocal = pos == "fft";
    var dacLocal = pos == "dac";
    if (ends.ContainsKey((pos, fft || fftLocal, dac || dacLocal)))
        return ends[(pos, fft || fftLocal, dac || dacLocal)];
    var sum = 0L;
    foreach(var path in map[pos])
    {
        if (path.Equals(target))
        {
            if (fft && dac)
                sum++;
            continue;
        }
        sum += dfs(path, target, fft || fftLocal, dac || dacLocal);
    }
    ends[(pos, fft || fftLocal, dac || dacLocal)] = sum;
    return sum;
}

Console.WriteLine($"{result}");