import java.util.Scanner;

public class scalingrecipe {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        long n = sc.nextLong();
        Double x = sc.nextDouble();
        Double y = sc.nextDouble();
        for (int i = 0; i < n; i++) {
            Double d = sc.nextDouble() * (y/x);
            System.out.println(Math.round(d));
        }
    }
}
