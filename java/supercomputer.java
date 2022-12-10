import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.StringTokenizer;

class supercomputer {
    public static void main(String[] args) {
        FastReader sc = new FastReader();
        int n = sc.nextInt() +1;
        int k = sc.nextInt();
        int[] arr = new int[n];
        /* for(int i = 0; i < n; i++){
            arr[i] = 0;
        } */

        FenwickTree fte = new FenwickTree(n);

        for(int i = 0; i < k; i++){
            String input = sc.nextLine();
            String[] inputArr = input.split(" ");

            if(inputArr[0].equals("F")){
                int index = Integer.parseInt(inputArr[1]);
                arr[index] = (arr[index] == 1) ? 0 : 1;
                //arr[index] = (arr[index] +1)%2;
                if(arr[index] == 1){
                    fte.updateFenwick(index, 1);
                }else{
                    fte.updateFenwick(index, -1);
                }
            }else if (inputArr[0].equals("C")){
                int from = Integer.parseInt(inputArr[1]);
                int to = Integer.parseInt(inputArr[2]);
                int temp1 = fte.getArrSum(from-1);
                int temp2 = fte.getArrSum(to);
                System.out.println(temp2 - temp1);
            }
        }
    }

    static public class FenwickTree {
        int MAX_SIZE; 
        private int fenArr[];  
        
        FenwickTree(int MAX_SIZE){
            this.MAX_SIZE = MAX_SIZE;
            fenArr = new int[MAX_SIZE +1];

            // Initializing fenArr[] as 0
            /* for(int i = 1; i <= MAX_SIZE; i++) {  
                fenArr[i] = 0;
            } */
        }

        // s --> It is number of element available in the input array.  
        // fenArr[0 ... s] --> The array that represents the Fenwick Tree  
        // a[0 ... s - 1] --> It is the input array for which the prefix sum is generated.  
        // Returns the sum of a[0... idx]. The method assumes  
        // that the array is already preprocessed and  
        // the partial sums of the array elements are kept  
        // in fenArr[].  
        int getArrSum(int idx) { 
            int total = 0;
            // index in the fenTree[] is one more than the  
            // index in the array a[]  
            idx = idx + 1;
            // Traversing the ancestors of the fenTree[idx]  
            while(idx > 0) {  
                // Adding the current element of the array fenArr[]  
                // to the total  
                total = total + fenArr[idx];  
                // Moving the index to the parent node in  
                // getArrSum view  
                idx -= idx & (-idx);  
            }  
            return total;  
        }  
        // Updating a node in the Fenwick Tree  
        // at a given index in the array fenArr[]. The given input value  
        // 'v' is added to the fenArr[idx] and therefore, it is also added to the  
        // ancestors of the tree too.  
        public void updateFenwick(int idx, int v) {  
            // index in the array fenArr[] is 1 more than the  
            // index in the array a[]  
            idx = idx + 1;
            // Traversing all the ancestors and adding 'v'  
            while(idx <= MAX_SIZE) {  
                // Add 'val' to current node of BIT Tree  
                fenArr[idx] = fenArr[idx]  + v;  
                
                // Updating the idx to that of parent  
                // in the update View  
                idx = idx + (idx & (-idx));  
            }  
        }
        // Method to build the Fenwick tree  
        // from the given array.  
        public void constructFenTreeFromArray(int[] arr) {
            // Storing the original values in the fenArr[]
            // using the mehtod updateFenwick()
            for(int j = 0; j < arr.length ; j++) {  
                updateFenwick(j, arr[j]);  
            }  
        }
    }

    static class FastReader {
        BufferedReader br;
        StringTokenizer st;
    
        public FastReader()
        {
            br = new BufferedReader(
            new InputStreamReader(System.in));
        }
    
        String next()
        {
            while (st == null || !st.hasMoreElements()) {
                try {
                    st = new StringTokenizer(br.readLine());
                }
                catch (IOException e) {
                    e.printStackTrace();
                }
            }
            return st.nextToken();
        }
    
        int nextInt() { return Integer.parseInt(next()); }
    
        long nextLong() { return Long.parseLong(next()); }
    
        double nextDouble()
        {
            return Double.parseDouble(next());
        }
    
        String nextLine()
        {
            String str = "";
            try {
                if(st.hasMoreTokens()){
                    str = st.nextToken("\n");
                }
                else{
                    str = br.readLine();
                }
            }
            catch (IOException e) {
                e.printStackTrace();
            }
            return str;
        }
    }
}
