import java.util.Scanner;

public class hangman {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        
        String word = scanner.nextLine();
        String guess = scanner.nextLine();

        scanner.close();

        int correct = 0;
        int wrong = 0;
        for(int i = 0; i < guess.length(); i++ ){
            boolean flag = false;
            for(int j = 0; j < word.length(); j++ ){
                if(guess.charAt(i) == word.charAt(j)){
                    correct++;
                    flag = true;
                }
            }
            if(!flag){
                wrong++;
            }
            if (correct == word.length()) {
                System.out.println("WIN");
                break;
            }else if(wrong == 10){
                System.out.println("LOSE");
                break;
            }
        }
    }
}
