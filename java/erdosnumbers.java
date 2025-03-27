import java.util.ArrayList;
import java.util.Collections;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.Queue;
import java.util.Scanner;

public class erdosnumbers<T> {
    public static void main(String[] args) {
        erdosnumbers e = new erdosnumbers();
        e.run();
    }

    HashMap<String, Integer> shortcut = new HashMap<>();
    public void run() {
        Scanner sc = new Scanner(System.in);
        HashMap<String, HashSet<String>> graf = new HashMap<>();
        HashSet<String> seen = new HashSet<>();
        HashSet<String> seenStart = new HashSet<>();
        ArrayList<String> startNodes = new ArrayList<>();
        while (sc.hasNextLine()) {
            String input = sc.nextLine();
            if(input.isEmpty()) break;
            String[] s = input.split(" ");
            String startNode = s[0];
            startNodes.add(startNode);
            seenStart.add(startNode);
            for (String name : s) {
                seen.add(startNode);
                if(!graf.containsKey(startNode)) graf.put(startNode, new HashSet<>());
                if(!graf.containsKey(name)) graf.put(name, new HashSet<>());
                HashSet<String> hs = graf.get(startNode);
                hs.add(name); 
                hs = graf.get(name);
                hs.add(startNode);
            }
        }
        sc.close();

        for (String name : seen) {
            if(!seenStart.contains(name)) {
                graf.remove(name);
            }
        }

        String endNode = "PAUL_ERDOS";
        shortcut.put(endNode, 0);
        bfs(graf, endNode, "bla");
        for (String name : startNodes) {
            if (shortcut.containsKey(name)) System.out.println(name + " " + shortcut.get(name));
            else System.out.println(name + " no-connection");
        }
    }

    private ArrayList<String> bfs(HashMap<String, HashSet<String>> graf, String startNode, String endNode) {
        Queue<String> possibleRouts = new LinkedList<>();
        HashMap<String, String> pastNodes = new HashMap<>();
        ArrayList<String> path = new ArrayList<>();
        if(startNode != endNode) possibleRouts.add(startNode);

        pastNodes.put(startNode, startNode);
        outer: while(!possibleRouts.isEmpty()){
            String v = possibleRouts.poll();
            for(String s : graf.get(v)){
                if(!pastNodes.containsKey(s)){
                    possibleRouts.add(s);
                    pastNodes.put(s, v);
                    shortcut.put(s, shortcut.get(v)+1);
                    if(s.equals(endNode)){
                        while(true){
                            path.add(s);
                            if(s.equals(startNode)) break outer;
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


