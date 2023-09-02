import java.util.ArrayList;
import java.util.Scanner;

public class vote {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        for (int i = 0; i < n; i++) {
            int m = sc.nextInt();
            long total = 0;
            long most = 0;
            int can = 0;
            boolean equ = true;
            for (int j = 0; j < m; j++){
                int votes = sc.nextInt();
                total += votes;
                if(most < votes) {
                    most = votes;
                    can = j+1;
                    equ = false;
                }else if (most == votes) {
                    equ = true;
                }
            }
            if(most *2 > total){
                System.out.println("majority winner " + can);
            }else if (!equ) {
                System.out.println("minority winner " + can);
            }else {
                System.out.println("no winner");
            }
        }
    }
}
