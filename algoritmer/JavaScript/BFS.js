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