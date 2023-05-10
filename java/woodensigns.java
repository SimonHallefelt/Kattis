import java.util.Scanner;

public class woodensigns {
    
    static int[] steps;
    static int n;
    static int[][] dp;
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        n = sc.nextInt();
        steps = new int[n+1];
        dp = new int[n+2][n+2];
        for(int i = 0; i < n+1; i++){
            steps[i] = sc.nextInt();
        }
        sc.close();
        
        System.out.println(solve(2, 1, steps[0], steps[1]));
        
    }
    
    public static long solve(int step, long amount, int l, int r){
        long mod = 2147483647;
        if(step == n+1){
            return amount;
        }

        if(dp[l][r] != 0){
            return dp[l][r];
        }

        int s = steps[step];
        if(s < l){
            long temp1 = solve(step+1, amount, s, r);
            dp[l][r] = (int)temp1;
            return temp1;
        }
        else if(s > r){
            long temp = solve(step+1, amount, l, s);
            dp[l][r] = (int)temp;
            return temp;
        }
        else{
            long temp1 = solve(step+1, amount, s, r);
            long temp2 = solve(step+1, amount, l, s);
            long temp = (temp1 + temp2) % mod;
            dp[l][r] = (int)temp;
            return temp;
        }
    }
}