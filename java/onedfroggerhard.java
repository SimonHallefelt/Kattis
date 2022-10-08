import java.util.ArrayList;
import java.util.HashSet;
import java.util.Scanner;
import java.util.Set;

public class onedfroggerhard {
    static ArrayList<Integer> bord = new ArrayList<>();
    static ArrayList<Set<Integer>> reatch = new ArrayList<>();
    static ArrayList<Boolean> cir = new ArrayList<>();
    static int n;
    static int total = 0;

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        n = scanner.nextInt();

        for(int i = 0; i < n; i++){
            bord.add(scanner.nextInt());
            Set<Integer> s = new HashSet<>();
            reatch.add(s);
            cir.add(false);
        }
        
        for(int i = 0; i < n; i++){
            hej(i);
        }

        System.out.println(total);
    }

    private static Set<Integer> hej(int i) {
        if(reatch.get(i).size() != 0){
            return reatch.get(i);
        }
        if(cir.get(i)){
            int temp = bord.get(i);
            reatch.get(i).add(bord.get(i));
            while(temp == i){
                reatch.get(i).add(bord.get(temp));
                temp = bord.get(temp);
            }
            return reatch.get(i);
        }
        cir.set(i, true);

        int moveTo = i+bord.get(i);
        if(moveTo < n && moveTo >= 0){
            reatch.get(i).addAll(hej(moveTo));
            reatch.get(i).add(bord.get(i));
            total += reatch.get(i).size();
            return reatch.get(i);
        }else{
            reatch.get(i).add(bord.get(i));
            total++;
            return reatch.get(i);
        }
    }
}