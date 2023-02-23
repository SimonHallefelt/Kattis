import java.util.ArrayList;
import java.util.HashSet;
import java.util.Iterator;
import java.util.LinkedList;
import java.util.Scanner;
import java.util.Stack;

public class movienight {
    static ArrayList<ArrayList<Integer>> pointToYou = new ArrayList<ArrayList<Integer>>();
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        for(int i = 0; i < n; i++){
            pointToYou.add(new ArrayList<Integer>());
        }
        kosaraju g = new kosaraju(n);
        for (int i = 0; i < n; i++) {
            int a = sc.nextInt();
            g.addEdge(i, a-1);
            pointToYou.get(a-1).add(i);
        }
        sc.close();

        ArrayList<ArrayList<Integer>> list = g.printSCCs();
        
        long subsets = 0;
        for(ArrayList<Integer> a : list){
            if(a.size() > 1){
                if(subsets == 0){
                    subsets = treebild(a);
                }else{
                    long temp = treebild(a);
                    subsets = ((subsets * (temp + 1)) + temp) % 1000000007;
                }
            }
        }
        System.out.println(subsets % 1000000007);
    }

    public static long treebild(ArrayList<Integer> a){
        long sum = 1;
        ArrayList<Long> b = new ArrayList<>();
        HashSet<Integer> set = new HashSet<Integer>();
        for(Integer i : a){
            set.add(i);
        }
        for(Integer i : a){
            if(pointToYou.get(i).size() > 0){
                for(Integer j : pointToYou.get(i)){
                    if(!set.contains(j)){
                        b.add(treebildint((long)j) + 1);
                    }
                }
            }
        }
        if(b.size() > 0){
            for(Long j : b){
                sum = (sum * j) % 1000000007;
            }
        }
        return sum;
    }
    public static Long treebildint(long i){
        long sum = 1;
        ArrayList<Long> b = new ArrayList<>();
        if(pointToYou.get((int)i).size() > 0){
            for(Integer j : pointToYou.get((int)i)){
                b.add(treebildint(j) + 1);
            }
        }
        if(b.size() > 0){
            for(Long j : b){
                sum = (sum * j) % 1000000007;
            }
        }
        return sum;
    }
}



class kosaraju {
    // This class represents a directed graph using adjacency list
    // representation
    private int V;   // Nomber of vertices
    private LinkedList<Integer> adj[]; //Adjacency List
    
    kosaraju(int v){
        V = v;
        adj = new LinkedList[v];
        for (int i=0; i<v; ++i){
            adj[i] = new LinkedList();
        }
    }
     
    //Function to add an edge into the graph
    void addEdge(int v, int w)  { 
        adj[v].add(w); 
    }
     
    // A recursive function to print DFS starting from v
    ArrayList<Integer> DFSUtil(int v, boolean visited[], ArrayList<Integer> list){
        // Mark the current node as visited and print it
        visited[v] = true;
        list.add(v);
     
        int n;
     
        // Recur for all the vertices adjacent to this vertex
        Iterator<Integer> i =adj[v].iterator();
        while (i.hasNext())
        {
            n = i.next();
            if (!visited[n]){
                DFSUtil(n, visited, list);
            }
        }
        return list;
    }
     
    // Function that returns reverse (or transpose) of this graph
    kosaraju getTranspose(){
        kosaraju g = new kosaraju(V);
        for (int v = 0; v < V; v++){
            // Recur for all the vertices adjacent to this vertex
            Iterator<Integer> i =adj[v].listIterator();
            while(i.hasNext()){
                g.adj[i.next()].add(v);
            }
        }
        return g;
    }
     
    void fillOrder(int v, boolean visited[], Stack stack){
        // Mark the current node as visited and print it
        visited[v] = true;
     
        // Recur for all the vertices adjacent to this vertex
        Iterator<Integer> i = adj[v].iterator();
        while (i.hasNext()){
            int n = i.next();
            if(!visited[n]){
                fillOrder(n, visited, stack);
            }
        }
     
        // All vertices reachable from v are processed by now,
        // push v to Stack
        stack.push(v);
    }
     
    // The main function that finds and prints all strongly
    // connected components
    ArrayList<ArrayList<Integer>> printSCCs(){
        Stack stack = new Stack();
     
        // Mark all the vertices as not visited (For first DFS)
        boolean visited[] = new boolean[V];
        for(int i = 0; i < V; i++){
            visited[i] = false;
        }
     
        // Fill vertices in stack according to their finishing
        // times
        for (int i = 0; i < V; i++){
            if (visited[i] == false){
                fillOrder(i, visited, stack);
            }
        }
     
        // Create a reversed graph
        kosaraju gr = getTranspose();
     
        // Mark all the vertices as not visited (For second DFS)
        for (int i = 0; i < V; i++){
            visited[i] = false;
        }
     
        ArrayList<ArrayList<Integer>> list = new ArrayList<ArrayList<Integer>>();
        // Now process all vertices in order defined by Stack
        while (stack.empty() == false){
            // Pop a vertex from stack
            int v = (int)stack.pop();
     
            // Print Strongly connected component of the popped vertex
            if (visited[v] == false){
                list.add(gr.DFSUtil(v, visited, new ArrayList<Integer>()));
            }
        }
        return list;
    }
}
