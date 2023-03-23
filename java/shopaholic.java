import java.util.Arrays;
import java.util.Scanner;

public class shopaholic {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        int n = scanner.nextInt();
        int[] prices = new int[n];
        for(int i = 0; i < n; i++){
            prices[i] = scanner.nextInt();
        }
        scanner.close();

        Arrays.sort(prices);

        long sum = 0;
        for(int i = n-3; i >= 0; i-=3){
            sum += prices[i];
        }

        System.out.println(sum);
    }
}
