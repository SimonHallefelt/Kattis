import java.util.HashMap;
import java.util.Scanner;

public class cakeymccakeface {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int m = sc.nextInt();
        long[] a = new long[n];
        long[] b = new long[m];

        for(int i = 0; i < n; i++){
            a[i] = sc.nextLong();
        }
        for(int i = 0; i < m; i++){
            b[i] = sc.nextLong();
        }
        sc.close();

        int t = 0;
        HashMap<Long, Integer> map = new HashMap<Long, Integer>();
        for(int i = 0; i < n; i++){
            for(int j = t; j < m; j++){
                long temp = b[j] - a[i];
                if(temp < 0){
                    t = j;
                }
                else{
                    if(map.containsKey(temp)){
                        map.put(temp, map.get(temp) + 1);
                    }else{
                        map.put(temp, 1);
                    }
                }
            }
        }

        long max = 0;
        long maxKey = 0;
        for(long key : map.keySet()){
            System.out.println("key: " + key + " " + map.get(key));
            if(map.get(key) == max && key < maxKey){
                maxKey = key;
            }else if(map.get(key) > max){
                max = map.get(key);
                maxKey = key;
            }
        }

        System.out.println(maxKey);
    }
}
