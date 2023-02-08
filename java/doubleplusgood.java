import java.util.ArrayList;
import java.util.HashSet;
import java.util.Scanner;

public class doubleplusgood {
    static HashSet<Long> set = new HashSet<>();
    static ArrayList<Long> list = new ArrayList<>();
    static String[] words;
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        String word = scanner.nextLine();
        scanner.close();

        words = word.split("\\+");
        for(int i = 0; i < words.length; i++ ){
            list.add(Long.parseLong(words[i]));
        }

        sum(0, 0, 0);

        System.out.println(set.size());
    }

    public static void sum(long a, long b, int c){
        if(c >= list.size()){
            set.add(a+b);
            return;
        }
        
        long temp = b;
        if(temp != 0) {
            sum(a+b, 0, c);
            for(int i = 0; i < words[c].length(); i++ ){
                temp = temp*10;
            }
        }

        temp += list.get(c);
        sum(a, temp, c+1);
    }
}