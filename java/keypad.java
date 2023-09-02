import java.util.ArrayList;
import java.util.Scanner;

public class keypad {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        for (int i = 0; i < n; i++) {
            int r = sc.nextInt();
            int c = sc.nextInt(); sc.nextLine();

            ArrayList<ArrayList<String>> grid = new ArrayList<>();
            for (int j = 0; j < r; j++) {
                char[] s = sc.nextLine().toCharArray();
                ArrayList<String> temp = new ArrayList<>();
                for (char ca : s) {
                    temp.add(String.valueOf(ca));
                }
                grid.add(temp);
            } 

            Boolean impos = true;
            ArrayList<ArrayList<String>> awnser = new ArrayList<>();
            for (int j = 0; j < r; j++) {
                ArrayList<String> temp2 = new ArrayList<>();
                for (int k = 0; k < c; k++) {
                    String a = "N";
                    if (grid.get(j).get(k).equals("1")) {
                        int temp = 0;
                        if (j-1 >= 0){
                            if (grid.get(j-1).get(k).equals("0")){
                                temp++;
                            }
                        }else temp++;
                        if (j+1 < r){
                            if (grid.get(j+1).get(k).equals("0")){
                                temp++;
                            }
                        }else temp++;
                        if (k-1 >= 0){
                            if (grid.get(j).get(k-1).equals("0")){
                                temp++;
                            }
                        }else temp++;
                        if (k+1 < c){
                            if (grid.get(j).get(k+1).equals("0")){
                                temp++;
                            }
                        }else temp++;

                        if (temp == 4) {
                            a = "P";
                            impos = false;
                        }else if(temp > 0){
                            a = "I";
                            impos = false;
                        }
                    }
                    temp2.add(a);
                }
                awnser.add(temp2);
            }

            if(impos) {
                System.out.println("impossible");
            }else{
                for (ArrayList<String> ar: awnser) {
                    for (String str: ar) {
                        System.out.print(str);
                    }System.out.println();
                }
            }System.out.println("----------");
        }
    }
}
