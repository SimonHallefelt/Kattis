// var data = File.ReadAllLines("../../../ExampleData.txt"); // 357
var data = File.ReadAllLines("../../../Data.txt"); // 17193

var sum = 0;
foreach(var line in data)
{
    var largest = 0;
    var secondLargest = 0;
    var lineLength = line.Count();
    for (var i = 0; i < lineLength; i++)
    {
        var value = int.Parse(line[i].ToString());
        if (value > largest && i+1 < lineLength)
        {
            largest = value;
            secondLargest = 0;
        }
        else if (value > secondLargest)
            secondLargest = value;
    }
    sum += largest * 10 + secondLargest;
    Console.WriteLine($"{sum}, {largest * 10 + secondLargest}");
}

