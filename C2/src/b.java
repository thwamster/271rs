public class b {

    private static long[] cache; // Array for memoization

    public static void main(String[] args) {
        int n = 50; // Example: compute the 50th Fibonacci number
        cache = new long[n + 1]; // Initialize cache array
        System.out.println("Fibonacci(" + n + ") = " + fib(n));
    }

    public static long fib(int n) {
        if (n <= 0) return 0;
        if (n == 1) return 1;

        // Return cached value if already computed
        if (cache[n] != 0) return cache[n];

        // Recursive call and store result in cache
        cache[n] = fib(n - 1) + fib(n - 2);
        return cache[n];
    }
}
