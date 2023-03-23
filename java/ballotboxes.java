import java.util.Arrays;
import java.util.Scanner;

public class ballotboxes {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        //int bla2 = 0;
        //while(bla2 < 2){
        while(true){
            int n = sc.nextInt();
            int b = sc.nextInt() - n;
            if(n == -1 && b == 0) break;
            int[] boxes = new int[n];
            int sum = 0;
            for(int i = 0; i < n; i++){
                boxes[i] = sc.nextInt();
                sum += boxes[i];
            }

            Arrays.sort(boxes);

            int max = boxes[n-1];
            int min = sum/(n+b);
            int a = binerySearch(boxes, b, min, max);

            System.out.println(a);
            //bla2++;
        }
    }

    public static int binerySearch(int[] boxes, int b, int min, int max){
        int mid = (min + max)/2;
        int used = 0;
        for(int i = boxes.length-1; i >= 0; i--){
            if(boxes[i] <= mid || used > b){
                break;
            }
            else{
                for(int j = 2; ; j++){
                    if(boxes[i] <= mid*j){
                        //System.out.println(" j: " + j);
                        used += (j-1);
                        break;
                    }
                }
            }
        }
        if(used > b){
            //System.out.println("inc, used: " + used + " b: " + b + " min: " + min + " max: " + max + " mid: " + mid);
            if(min == mid) return max;
            return binerySearch(boxes, b, mid, max);
        }else if(min < mid){
            //System.out.println("dec, used: " + used + " b: " + b + " min: " + min + " max: " + max + " mid: " + mid);
            return binerySearch(boxes, b, min, mid);
        }else{
            //System.out.println("return, used: " + used + " b: " + b + " min: " + min + " max: " + max + " mid: " + mid);
            return mid;
        }
    }
}
