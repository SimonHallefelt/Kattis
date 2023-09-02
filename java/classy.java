import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.HashMap;
import java.util.Scanner;

public class classy {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        for(int i = 0; i < n; i++) {
            List<String> list = new ArrayList<>();
            HashMap<String, List<String>> hMap = new HashMap<>();
            int m = sc.nextInt();
            sc.nextLine();
            for(int j = 0; j < m; j++) {
                String line = sc.nextLine();
                String[] st = line.split(":");
                String name = st[0];
                st = st[1].split(" ");
                st = st[1].split("-");
                StringBuilder c = new StringBuilder();
                for (String s : st) {
                    if (s.equals("upper")) {
                        c.append("a");
                    }else if (s.equals("middle")){
                        c.append("b");
                    }else {
                        c.append("c");
                    }
                }
                c = c.reverse();
                for(int k = c.length(); k < 31; k++){
                    c.append("b");
                }
                String cl = c.toString();
                if(!hMap.containsKey(cl)){
                    list.add(cl);
                    List<String> temp = new ArrayList<>();
                    temp.add(name);
                    hMap.put(cl, temp);
                }else{
                    hMap.get(cl).add(name);;
                }
            }
            Collections.sort(list);
            for (String s : list){
                Collections.sort(hMap.get(s));
                for (String na : hMap.get(s)) {
                    System.out.println(na);
                }
            }
            for(int j = 0; j < 30; j++) {
                System.out.print("=");
            }System.out.println();
        }

    }
} 