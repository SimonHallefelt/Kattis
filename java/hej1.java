import java.util.Scanner;

class hej1{
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);

        String s = scanner.next();
        scanner.close();

        StringBuilder ns = new StringBuilder();

        System.out.println(theThing(ns, s.length()/2, s.length()));
    }

    private static int theThing(StringBuilder s, int len, int nuvarandeSvar){
        int nyttSvar = len;

        return nyttSvar;
    }
}
