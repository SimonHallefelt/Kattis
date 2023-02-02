import java.io.BufferedReader;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.StringTokenizer;

//fel

public class dimensionalanalysis {
    public static void main(String[] args) {
        FastReader scanner = new FastReader();
        int p = 1000000181;
        HashSet<String> hs = new HashSet<>();
        HashMap<String, Integer> hm = new HashMap<>();
        hs.add("1");
        hs.add("/'");
        hs.add("*");
        hm.put("1", 1);
        hm.put("/'", 1061);
        hm.put("*", 1013);
        
        int n = scanner.nextInt();
        for(int i = 1; i <= n; i++){
            String s = scanner.nextLine();
            String[] s2 = s.split(" = ");

            for(String temp : s2){
                
            }
            
        }

        
        


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
