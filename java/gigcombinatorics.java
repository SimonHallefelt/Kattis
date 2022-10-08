import java.util.ArrayList;
import java.util.Scanner;

public class gigcombinatorics {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        int n = scanner.nextInt();
        ArrayList<Integer> al = new ArrayList<>();
        for(int i = 0; i < n; i++){
            al.add(scanner.nextInt());
        }

        int total = 0;
        int t = 0;
        ArrayList<Integer> al3 = new ArrayList<>();

        for(int i = n-1; i >= 0; i--){
            if(al.get(i) == 1){
                for(int j = 0; j < al3.size(); j++){
                    total += Math.pow(2, t-al3.get(j))-1;
                }
            }else if(al.get(i) == 2){
                t++;
            }else{
                al3.add(t);
            }
        }

        System.out.println(total);

    }
}
