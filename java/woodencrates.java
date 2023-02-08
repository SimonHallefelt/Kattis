import java.util.ArrayList;
import java.util.Scanner;

import javax.print.StreamPrintService;

public class woodencrates {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        int n = scanner.nextInt();
        scanner.nextLine();
        String s = scanner.nextLine();
        scanner.close();

        long[] a = new long[n];
        String[] b = new String[n];
        b = s.split(" ");
        long total = 0;
        for(int i = 0; i < n; i++ ){
            long temp = Long.parseLong(b[i]);
            a[i] = temp;
            total += temp;
        }
        
        int hojd = 0;
        int hogar = 0;
        for(long i = n; i <= total; i++ ){
            if (total % i == 0) {
                hojd = (int) (total / i);
                hogar = (int) i;
                break;
            }
        }
        System.out.println("h = " + hogar + " " + hojd);

        ArrayList<Integer> c = new ArrayList<>();
        for(int i = 0; i < hogar; i++ ){
            if(i < n){
                c.add((int) (a[i] - hojd));
            }else{
                c.add(-hojd);
            }
        }

        int counter = 0;
        long lador = 0;
        long steps = 0;
        long position = 0;
        for(int i = 0; i < hogar; i++ ){
            if(lador < 0 && c.get(i) > 0){
                System.out.println("hej_1");
                while(c.get(i) >= Math.abs(c.get(counter)) && lador != 0){
                    int temp = Math.abs(c.get(counter));
                    c.set(i, c.get(i)-temp);
                    c.set(counter, 0);
                    steps += i-position;
                    position = counter;
                    steps += temp * ((i-counter)*2)-(i-counter);
                    steps += temp * 2;
                    counter++;
                    lador += temp;
                    System.out.println("hej_1_1 steps:" + steps);
                }
                if(c.get(i) != 0 && lador != 0){
                    int temp = c.get(i);
                    c.set(i, 0);
                    c.set(counter, c.get(counter)+temp);
                    steps += i-position;
                    position = counter;
                    steps += temp * ((i-counter)*2)-(i-counter);
                    steps += temp * 2;
                    lador += temp;
                    System.out.println("hej_1_2 steps:" + steps);
                }
                if(c.get(i) != 0 && lador == 0){
                    lador += c.get(i);
                    System.out.println("hej_1_3 steps:" + steps);
                }
            }else if(lador > 0 && c.get(i) < 0){
                System.out.println("hej_2");
                while(Math.abs(c.get(i)) > c.get(counter) && lador != 0){
                    int temp = c.get(counter);
                    c.set(i, c.get(i)+temp);
                    c.set(counter, 0);
                    steps += Math.abs(counter-position);
                    position = i;
                    steps += temp * ((i-counter)*2)-(i-counter);
                    steps += temp * 2;
                    counter++;
                    lador -= temp;
                    System.out.println("hej_2_1 steps:" + steps);
                }
                if(c.get(i) != 0 && lador != 0){    
                    int temp = Math.abs(c.get(i));
                    c.set(i, 0);
                    c.set(counter, c.get(counter)-temp);
                    steps += Math.abs(counter-position);
                    position = i;
                    steps += temp * ((i-counter)*2)-(i-counter);
                    steps += temp * 2;
                    lador -= temp;
                    System.out.println("hej_2_2 steps:" + steps);
                }
                if(c.get(i) != 0 && lador == 0){
                    lador += c.get(i);
                    System.out.println("hej_2_3 steps:" + steps);
                }
            }else{
                System.out.println("hej_3");
                lador += c.get(i);
            }
            
            System.out.println("steps " + steps + " lador " + lador);
            for(int j = counter; j < i; j++ ){
                if(c.get(j) == 0){
                    counter++;
                }else{
                    break;
                }
            }
        }

        System.out.println(steps);
    }
}