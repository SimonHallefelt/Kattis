import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.Queue;

public class BFS {
    private ArrayList<Integer> bfs(HashMap<Integer, HashSet<Integer>> graf, int startNode, int endNode) {
        Queue<Integer> possibleRouts = new LinkedList<>();
        HashMap<Integer, Integer> pastNodes = new HashMap<>();
        ArrayList<Integer> path = new ArrayList<>();
        if(startNode != endNode) possibleRouts.add(startNode);

        outer: while(!possibleRouts.isEmpty()){
            int v = possibleRouts.poll();
            for(int s : graf.get(v)){
                if(!pastNodes.containsKey(s)){
                    possibleRouts.add(s);
                    pastNodes.put(s, v);
                    if(s == endNode){
                        while(true){
                            path.add(s);
                            if(s == startNode) break outer;
                            s = pastNodes.get(s);
                        }
                    }
                }
            }
        }

        Collections.reverse(path);
        return path;
    }
}