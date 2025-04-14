main()
function main() {
    const readline = require('readline');

    // Create an interface for reading input
    const rl = readline.createInterface({
        input: process.stdin,
        output: process.stdout
    });

    // Read input line by line
    let lines = [];
    rl.on('line', (line) => {
        lines.push(line) 
        if (line.length === 0) {
            rl.close()
        }
    });

    rl.on('close', () => {
        lines = lines.filter(line => line.trim().length > 0);
        handleInput(lines)
    });
}

function handleInput(lines) {
    let starts = []
    let graph = {}
    lines.forEach(line => {
        line = line.split(' ')
        starts.push(line[0])
        if (!graph.hasOwnProperty(line[0])) {
            graph[line[0]] = []
        }
        line.forEach(name => {
            if (name !== line[0]) {
                graph[line[0]].push(name)
                if (!graph.hasOwnProperty(name)) {
                    graph[name] = []
                }
                graph[name].push(line[0])
            }
        })
    });

    endNode = 'PAUL_ERDOS'
    distances[endNode] = 0
    BFS(graph, endNode, 'bla')
    starts.forEach(name => {
        if (distances[name] === undefined) {
            console.log(name, 'no-connection')
        } else {
            console.log(name, distances[name])
        }
    })
}

let distances = {}
function BFS(graph, startNode, endNode) {
    let path = []
    let possibleRouts = [startNode]
    let pastNode = {}
    pastNode[startNode] = startNode
    if (startNode === endNode) {
        return [startNode]
    }
    
    while(possibleRouts.length > 0) {
        const node = possibleRouts.shift()
        graph[node].forEach(newNode => {
            if (!pastNode.hasOwnProperty(newNode)) {
                pastNode[newNode] = node
                possibleRouts.push(newNode)
                distances[newNode] = distances[node] + 1
                if (newNode === endNode) {
                    path.push(endNode)
                    while (newNode !== startNode) {
                        newNode = pastNode[newNode]
                        path.push(newNode)
                    }
                    return path.reverse()
                }
            }
        })
    }

    return path
}