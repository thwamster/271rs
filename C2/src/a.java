import java.util.HashMap;
import java.util.Map;

public class a {

    // Memoization map to store already computed Fibonacci numbers
    private static Map<Integer, Long> memo = new HashMap<>();

    public static void main(String[] args) {
        int n = 50; // Example: compute the 50th Fibonacci number
        System.out.println("Fibonacci(" + n + ") = " + fibonacci(n));
    }

    public static long fibonacci(int n) {
        if (n <= 0) {
            return 0;
        } else if (n == 1) {
            return 1;
        }

        // Check if result is already computed
        if (memo.containsKey(n)) {
            return memo.get(n);
        }

        // Recursive computation with memoization
        long result = fibonacci(n - 1) + fibonacci(n - 2);
        memo.put(n, result); // Store result in memo map

        return result;
    }
}
