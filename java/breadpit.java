import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.LinkedList;
import java.util.Queue;
import java.util.Scanner;

public class breadpit {
    
    public static void main(String[] args) {
        run();
        
    }

    static int n, q;
    static ArrayList<Queue<Integer>> nodes = new ArrayList<>(n);
    public static void run(){
        
        Scanner scanner = new Scanner(System.in);
        n = scanner.nextInt();
        q = scanner.nextInt();
        

        for(int i = 0; i<n+1; i++){
            Queue<Integer> temp = new LinkedList<>();
            nodes.add(temp);
        }

        for(int i = 0; i<n-1; i++){
            nodes.get(scanner.nextInt()).add(i+1);
        }

        for(int i = 0; i<q; i++){
            funk(0);
        }
    }

    public static void funk(int i){
        if(nodes.get(i).isEmpty()){
            System.out.println(i);
        }else{
            int temp = nodes.get(i).poll();
            nodes.get(i).add(temp);
            funk(temp);
        }
    }
}
