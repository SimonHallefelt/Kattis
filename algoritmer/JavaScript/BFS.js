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

function test1() {
    let graph = {}
    graph[1] = [2,3]
    graph[2] = [3,4]
    graph[3] = [2,3,4,5]
    graph[4] = [2,3,4]
    graph[5] = [4,6]
    graph[6] = [5]
    graph[7] = [8]
    graph[8] = [7]
    console.log(1 === BFS(graph, 1, 1).length);
    console.log(4 === BFS(graph, 1, 6).length);
    console.log(0 === BFS(graph, 1, 8).length);
    console.log(2 === BFS(graph, 7, 8).length);
    console.log(2 === BFS(graph, 8, 7).length);

    console.log(0 !== BFS(graph, 7, 7).length);
}
test1()