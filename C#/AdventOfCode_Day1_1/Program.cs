var startValue = 50;

var minValue = 0;
var maxValue = 100;

// var data = File.ReadAllLines("../../../ExampleData.txt"); // 3
var data = File.ReadAllLines("../../../Data.txt"); // 1154

var value = startValue;
var result = 0;
foreach (var line in data)
{
    var direction = line[0];
    var distance = int.Parse(line.Substring(1));
    value += direction == 'R' ? distance : -distance;
    value %= maxValue;
    value = (value + maxValue) % maxValue;
    if (value == 0)
        result++;
    Console.WriteLine($"{value}, {result}");
}

Console.WriteLine(result);
