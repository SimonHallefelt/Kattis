import java.util.ArrayList;
import java.util.Scanner;

public class mrcodeformatgrader {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        int n = scanner.nextInt();
        int m = scanner.nextInt();
        scanner.nextLine();
        String s = scanner.nextLine();
        String[] s2 = s.split(" ");
        int[] arr = new int[n];
        for(int i = 0; i < m; i++){
            arr[i] = Integer.parseInt(s2[i]);
        }

        ArrayList<String> error = new ArrayList<>();
        ArrayList<String> correct = new ArrayList();

        error.add("Errors: ");
        correct.add("Correct: ");

        int last = arr[0];
        int last2 = arr[0];
        
        if(arr[0] != 1){
            if(arr[0] >= 3){
                correct.add("1-" + (arr[0]-1));
            }else{
                correct.add("1");
            }
        }

        int temp = arr[0];
        for(int i = 0; i < m; i++){
            temp = arr[i];
            if(i != m-1){
                if(temp-1 <= last2){
                    last2 = temp;
                }else{
                    if(last2 == last){
                        error.add(last+"");
                    }else{
                        error.add(last + "-" + last2);
                    }
                    if(temp > last2+2){
                        correct.add((last2+1) + "-" + (temp-1));
                    }else{
                        correct.add((temp-1)+"");
                    }
                    last = temp;
                    last2 = temp;
                }
            }else{
                if (temp == last2){
                    error.add(temp+"");
                }else if(temp-1 == last2){
                    error.add(last + "-" + temp);
                }else{
                    if(last2 == last){
                        error.add(last+"");
                    }else{
                        error.add(last + "-" + last2);
                    }
                    error.add(temp+"");

                    if(temp > last2+2){
                        correct.add((last2+1) + "-" + (temp-1));
                    }else{
                        correct.add((temp-1)+"");
                    }
                }
                if(temp != n){
                    if(temp+1 == n){
                        correct.add(n+"");
                    }else{
                        correct.add((temp+1) + "-" + n);
                    }
                }
            }
        }

        StringBuilder errors = new StringBuilder();
        StringBuilder corrects = new StringBuilder();
        errors.append(error.get(0));
        corrects.append(correct.get(0));
        errors.append(error.get(1));
        corrects.append(correct.get(1));

        for(int i = 2; i < error.size(); i++){
            if(i < error.size()-1){
                errors.append(", " + error.get(i));
            }else{
                errors.append(" and " + error.get(i));
            }
        }

        for(int i = 2; i < correct.size(); i++){
            if(i < correct.size()-1){
                corrects.append(", " + correct.get(i));
            }else{
                corrects.append(" and " + correct.get(i));
            }
        }

        System.out.println(errors);
        System.out.println(corrects);
    }
}
