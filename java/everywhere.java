import java.util.HashSet;
import java.util.Scanner;

public class everywhere {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        for (int i = 0; i < n; i++) {
            int m = sc.nextInt(); sc.nextLine();
            HashSet<String> hSet = new HashSet<>();
            for (int j = 0; j < m; j++){
                hSet.add(sc.nextLine());
            }
            System.out.println(hSet.size());
        }
    }
}
