import java.io.*;
import java.util.*;
import java.util.LinkedList;

public class illumination {
    public static void main(String[] args) {
        List<Integer[]> nodes = new ArrayList<>();
        List<List<Integer>> rows = new ArrayList<>();
        List<List<Integer>> cols = new ArrayList<>();

        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int r = sc.nextInt();
        int k = sc.nextInt();

        for(int i = 0; i < n; i++){
            rows.add(new ArrayList<Integer>());
            cols.add(new ArrayList<Integer>());
        }

        for(int i = 0; i < k; i++){
            Integer[] temp = new Integer[2];
            temp[0] = sc.nextInt()-1;
            temp[1] = sc.nextInt()-1;
            nodes.add(temp);
            rows.get(temp[0]).add(i);
            cols.get(temp[1]).add(i);
        }
        sc.close();

        twoSat g = new twoSat(k);
        for(int i = 0; i < n; i++){
            for(int j = 0; j < rows.get(i).size(); j++){
                for(int l = j+1; l < rows.get(i).size(); l++){
                    if(Math.abs(nodes.get(rows.get(i).get(j))[0] - nodes.get(rows.get(i).get(l))[0]) <= r){
                        g.addOr(rows.get(i).get(j), rows.get(i).get(l), true, true);
                    }
                }
            }
            for(int j = 0; j < cols.get(i).size(); j++){
                for(int l = j+1; l < cols.get(i).size(); l++){
                    if(Math.abs(nodes.get(cols.get(i).get(j))[1] - nodes.get(cols.get(i).get(l))[1]) <= r){
                        g.addOr(cols.get(i).get(j), cols.get(i).get(l), false, false);
                    }
                }
            }
        }
        if(g.solve()){
            System.out.println("1");
        }else{
            System.out.println("0");
        }

    }
}

//2sat problem
// time complexity: is linear



class twoSat{
    int nodes;
    List<Integer[]> nodesPointTo = new ArrayList<Integer[]>();
    ArrayList<ArrayList<Integer>> list;
    Map<Integer, Integer> containdInComponent;

    twoSat(int n){
        this.nodes = n;
        


    }

    public void addOr(int a, int b, boolean c, boolean d){
        Integer[] temp = new Integer[2];
        temp[0] = getNumber(a, !c);
        temp[1] = getNumber(b, d);
        nodesPointTo.add(temp);
        temp[0] = getNumber(b, !d);
        temp[1] = getNumber(a, c);
        nodesPointTo.add(temp);
    }

    public void addXor(int a, int b, boolean c, boolean d){
        
    }

    public void addAnd(int a, int b, boolean c, boolean d){
        
    }

    public void addSingel(int a, boolean b){
        
    }

    public int getNumber(int a, boolean b){// ett index i listan till noll index i arrayen
        if(b){
            return (a - 1) * 2;
        }else{
            return ((a - 1) * 2) + 1;
        }
    }

    public boolean solve(){
        kosaraju g = new kosaraju(nodesPointTo.size());
        for(int i = 0; i < nodesPointTo.size(); i++){
            g.addEdge(nodesPointTo.get(i)[0], nodesPointTo.get(i)[1]);
        }
        list = g.printSCCs();

        containdInComponent = new HashMap<Integer, Integer>();
        for(int i = 0; i < list.size(); i++){
            for(int j = 0; j < list.get(i).size(); j++){
                containdInComponent.put(list.get(i).get(j), i);
            }
        }

        for(int i = 0; i < nodes; i += 2){
            if(containdInComponent.get(i) == containdInComponent.get(i+1)){
                return false;
            }
        }

        return true;
    }

    public List<Boolean> getOneSolution(){
        List<Boolean> solution = new ArrayList<Boolean>(nodes);
        for(int i = 0; i < nodes; i += 2){
            if(containdInComponent.get(i) > containdInComponent.get(i+1)){
                solution.add(true);
            }else{
                solution.add(false);
            }
        }
        return solution;
    }

    public List<List<Boolean>> getAllSolutions(){



        return null;
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