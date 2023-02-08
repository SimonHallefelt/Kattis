import java.math.BigInteger;
import java.util.ArrayList;
import java.util.Scanner;

public class bikegears {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        int n = scanner.nextInt();
        int m = scanner.nextInt();
        BigInteger[] a = new BigInteger[n];
        BigInteger[] b = new BigInteger[m];

        for(int i = 0; i < n; i++ ){
            a[i] = scanner.nextBigInteger();
        }
        for(int i = 0; i < m; i++ ){
            b[i] = scanner.nextBigInteger();
        }

        scanner.close();

        ArrayList<ArrayList<BigInteger>> c = new ArrayList<>();
        for(int i = n-1; i >= 0; i-- ){
            for(int j = 0; j < m; j++ ){
                ArrayList<BigInteger> d = new ArrayList<>();
                d.add(a[i]);
                d.add(b[j]);
                c.add(d);
            }
        }   

        boolean flag = true;
        while(flag){
            flag = false;
            for(int i = 0; i < c.size()-1; i++ ){
                if(c.get(i).get(0).multiply(c.get(i+1).get(1)).compareTo(c.get(i+1).get(0).multiply(c.get(i).get(1))) == 1) {
                    ArrayList<BigInteger> temp = c.get(i);
                    c.set(i, c.get(i+1));
                    c.set(i+1, temp);
                    flag = true;
                }
            }
        }

        for(int i = 0; i < c.size(); i++ ){
            System.out.println("(" + c.get(i).get(0) + "," + c.get(i).get(1) + ")");
        }
    }
}