import java.util.HashSet;
import java.util.Scanner;

public class alienintegers {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        String input = scanner.nextLine();

        HashSet<String> hs = new HashSet<>();
        for(int i = 0; i < input.length(); i++){
            char c = input.charAt(i);
            String s = Character.toString(c);
            hs.add(s);
        }
        if(hs.size() == 10){
            System.out.println("Impossible");
            System.exit(0);
        }

        long min = Long.MAX_VALUE;
        long max = Long.MIN_VALUE;
        int a = Integer.parseInt(input.substring(0, 1));
        long temp = 0;
        Boolean flag2 = false;
        for(long i = a; i >= 0; i--){
            if(!hs.contains(Long.toString(i))){
                temp = i;
                flag2 = true;
                for(long j = 9; j >= 0; j--){
                    if(!hs.contains(Long.toString(j))){
                        for(int k = 1; k < input.length(); k++){
                            temp = temp * 10 + j;
                        }
                        min = temp;
                        break;
                    }
                }
                break;
            }
        }

        if(!flag2){
            for(long j = 9; j >= 0; j--){
                if(!hs.contains(Long.toString(j))){
                    for(long k = 1; k < input.length(); k++){
                        temp = temp * 10 + j;
                    }
                    min = temp;
                    break;
                }
            }
        }

        Boolean flag = false;
        for(int i = a; i < 10; i++){
            if(!hs.contains(Integer.toString(i))){
                temp = i;
                flag = true;
                for(long j = 0; j < 10; j++){
                    if(!hs.contains(Long.toString(j))){
                        for(int k = 1; k < input.length(); k++){
                            temp = temp * 10 + j;
                        }
                        max = temp;
                        break;
                    }
                }
                break;
            }
        }
        
        if(!hs.contains("1") && !flag){
            for(long j = 0; j < 2; j++){
                if(!hs.contains(Long.toString(j))){
                    temp = 10 + j;
                    for(long k = 1; k < input.length(); k++){
                        temp = temp * 10 + j;
                    }
                    max = temp;
                    break;
                }
            }
        }
        
        long ansmin = Long.parseLong(input) - min;
        long ansmax = max - Long.parseLong(input);
        if(ansmin < ansmax){
            System.out.println(min);
        } else if(ansmin > ansmax){
            System.out.println(max);
        } else {
            System.out.println(min + " " + max);
        }
    }
}
