import java.util.ArrayList;
import java.util.Scanner;

public class pandemicshopping {
    static ArrayList<Integer> path = new ArrayList<>();
    static int n;
    static int m;
    public static void main(String[] args) {
        
        Scanner scanner = new Scanner(System.in);
        int total = 0;

        n = scanner.nextInt();
        m = scanner.nextInt();

        for(int i = 0; i < n+2; i++){
            path.add(0);
        }

        for(int i = 0; i < m; i++){
            String temp1 = scanner.next();
            String temp2 = scanner.next();
            //System.out.println(temp1);
            if(temp1.equals("A")) {
                if(temp2.equals("S2N")){
                    path.set(0, 2);
                }else{
                    path.set(0, 1);
                }
            }else if(temp1.equals("B")) {
                if(temp2.equals("S2N")){
                    path.set(1, 2);
                }else{
                    path.set(1, 1);
                }
            }else{
                int j = Integer.parseInt(temp1) +1;
                if(temp2.equals("W2E")){
                    path.set(j, 1);
                }else{
                    path.set(j, 2);
                }
            }
        }

        if(path.get(0) != path.get(1) && path.get(0) != 0 && path.get(1) != 0){
            total += pow(n,m);
        }else if(path.get(0) == path.get(1) && path.get(0) != 0){
            total += shift();
        }else if(path.get(0) == 0 && path.get(1) == 0){
            total += pow(n,m);
            total += shift();
        }else{
            total += pow(n,m);
            total += shift();
        }

        System.out.println(total);
    }

    private static int pow(int n, int m) {
        if(path.get(0) == path.get(2) && path.get(0) != 0){
            return 0;
        }else if(path.get(1) == path.get(n+1) && path.get(1) != 0){
            return 0;
        }else if(path.get(2) == path.get(n+1) && path.get(2) != 0){
            return 0;
        }else{
            int b = 1;
            if(path.get(0) == path.get(1) && 
            path.get(1) == path.get(2) && 
            path.get(2) == path.get(n-1) && 
            path.get(0) == 0){
                b=2;
            }
            int a = 0;
            for(int i = 2; i < n+2; i+=2){
                if(path.get(i) == 0){
                    a++;
                }
            }
            return (int) Math.pow(2, a)*2;
        }
    }

    private static int shift() {
        if(n%2 == 0) {
            //System.out.println("hej1");
            return 0;
        }
        for(int i = 2; i < n+2; i+=2){
            if(path.get(i) != 0){
                for(int j = 3; j < n+2; j+=2){
                    if(path.get(j) != 0){
                        if(path.get(i) == path.get(j)){
                            //System.out.println(path.get(i) +" hej2 " + path.get(j));
                            return 0;
                        }
                    }
                }
            }
        }
        for(int i = 2; i < n+2; i+=2){
            if(path.get(i) != 0){
                return 1;
            }
        }
        return 2;
    }
}
