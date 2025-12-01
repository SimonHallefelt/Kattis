var startValue = 50;

var minValue = 0;
var maxValue = 100;

// var data = File.ReadAllLines("../../../ExampleData.txt"); // 6
var data = File.ReadAllLines("../../../Data.txt"); // 6819

var value = startValue;
var result = 0;
foreach (var line in data)
{
    var direction = line[0];
    var distance = int.Parse(line.Substring(1));
    var OldValue = value;
    value += direction == 'R' ? distance : -distance;
    if (OldValue == 0 && value < minValue)
        value += maxValue;
    while (value < minValue)
    {
        result++;
        value += maxValue;
    }
    if (value == 0)
        result++;
    while (value >= maxValue)
    {
        result++;
        value -= maxValue;
    }
    Console.WriteLine($"{value}, {line}, {OldValue}, {result}");
}

Console.WriteLine(result);
