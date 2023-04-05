import java.util.Scanner;

public class frosting {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        long[] a = new long[3];
        long[] b = new long[3];
        long[] c = new long[3];

        for(int i = 0; i < 3; i++){
            a[i] = 0;
            b[i] = 0;
            c[i] = 0;
        }

        for(int i = 0; i < n; i++){
            a[i%3] += sc.nextLong();
        }
        for(int i = 0; i < n; i++){
            b[i%3] += sc.nextLong();
        }
        sc.close();

        for(int i = 0; i < 3; i++){
            for(int j = 0; j < 3; j++){
                c[(i+j)%3] += a[i] * b[j];
            }
        }
        System.out.println(c[1] + " " + c[2] + " " + c[0]);
    }
}
