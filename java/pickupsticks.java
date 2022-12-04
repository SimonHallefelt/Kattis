import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.HashMap;
import java.util.HashSet;
import java.util.LinkedList;
import java.util.Map;
import java.util.Queue;
import java.util.Set;
import java.util.StringTokenizer;

class pickupsticks{
    static Map<Integer, Queue<Integer>> map = new HashMap<>();
    static Set<Integer> visited = new HashSet<>();
    static Set<Integer> free = new HashSet<>();
    static Queue<Integer> answer = new LinkedList<>();

    public static void main(String[] args) {
        FastReader  fr = new FastReader();
        int numberOfSticks = fr.nextInt();
        int lines = fr.nextInt();
        for(int i = 1; i < numberOfSticks+1; i++){
            map.put(i, new LinkedList<>());
        }
        for(int i = 0; i < lines; i++){
            int a = fr.nextInt();
            int b = fr.nextInt();
            map.get(b).add(a);
        }
        
        for(int i = 1; i < numberOfSticks+1; i++){
            if(!visited.contains(i)){
                dfs(i);
            }
        }

        for(int i = 0; i < numberOfSticks; i++){
            System.out.println(answer.poll());
        }
    }

    public static void dfs(int i){
        visited.add(i);
        Queue<Integer> q = map.get(i);
        while(!q.isEmpty()){
            int j = q.poll();
            if(!visited.contains(j)){
                dfs(j);
            }else if(!free.contains(j)){
                System.out.println("IMPOSSIBLE");
                System.exit(0);
            }
        }
        free.add(i);
        answer.add(i);
    } 

    static class FastReader {
        BufferedReader br;
        StringTokenizer st;
  
        public FastReader()
        {
            br = new BufferedReader(
                new InputStreamReader(System.in));
        }
  
        String next()
        {
            while (st == null || !st.hasMoreElements()) {
                try {
                    st = new StringTokenizer(br.readLine());
                }
                catch (IOException e) {
                    e.printStackTrace();
                }
            }
            return st.nextToken();
        }
  
        int nextInt() { return Integer.parseInt(next()); }
  
        long nextLong() { return Long.parseLong(next()); }
  
        double nextDouble()
        {
            return Double.parseDouble(next());
        }
  
        String nextLine()
        {
            String str = "";
            try {
                if(st.hasMoreTokens()){
                    str = st.nextToken("\n");
                }
                else{
                    str = br.readLine();
                }
            }
            catch (IOException e) {
                e.printStackTrace();
            }
            return str;
        }
    }
}