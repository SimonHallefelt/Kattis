import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.lang.reflect.Array;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.HashSet;
import java.util.Scanner;
import java.util.StringTokenizer;

public class typo {
    public static void main(String[] args) {
        FastReader scanner = new FastReader();

        int n = scanner.nextInt();
        HashMap<Integer, ArrayList<String>> hm = new HashMap<>();
        HashMap<Integer, HashSet<Integer>> hm2 = new HashMap<>();
        int p = 1000000181;
        ArrayList<String> inputOrder = new ArrayList<>();

        for(int i = 0; i < n; i++){
            String s = scanner.nextLine();
            int key = s.length();
            inputOrder.add(s);

            int hash = 0;
            for(int j = 0; j < s.length(); j++){
                hash = (hash*256+s.charAt(j))%p;
            }

            if(hm.containsKey(key)){
                hm.get(key).add(s);
                hm2.get(key).add(hash);
            } else {
                ArrayList<String> al = new ArrayList<>();
                HashSet<Integer> al2 = new HashSet<>();
                al.add(s);
                al2.add(hash);
                hm.put(key, al);
                hm2.put(key, al2);
            }
        }

        HashSet<Integer> keys = new HashSet<>();
        for(int i : hm.keySet()){
            keys.add(i);
        }

        HashSet<Integer> filterdKeys = new HashSet<>();
        for(int i : keys){
            if(keys.contains(i-1)){
                filterdKeys.add(i);
            }
        }

        HashSet<String> hs = new HashSet<>();
        for(int i : filterdKeys){
            for(String s : hm.get(i)){
                for(int j = 0; j < i; j++){
                    int hash = 0;
                    for(int k = 0; k < i; k++){
                        if(k != j){
                            hash = (hash*256+s.charAt(k))%p;
                        }
                    }
                    if(hm2.get(i-1).contains(hash)){
                        hs.add(s);
                    }
                }
            }
        }

        if(hs.size() == 0){
            System.out.println("NO TYPOS");
            return;
        } else {
            for(String s : inputOrder){
                if(hs.contains(s)){
                    System.out.println(s);
                }
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