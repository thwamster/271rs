// File: LongTestFile.java
// Purpose: Large, verbose, and messy Java source for diff testing.
// Note: intentionally bloated, repetitive, and full of edge-case constructs.

package com.example.longtest;

import java.io.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.function.*;
import java.lang.annotation.*;
import java.lang.reflect.*;
import java.math.*;
import java.net.*;
import java.util.stream.*;

// Top-level comment block
/*
 * This is intentionally over-commented.
 * It contains many comment styles, blank lines, and oddly indented
 * blocks to exercise diff algorithms and line-based/whitespace diffs.
 *
 * End of large comment block.
 */

// A simple annotation for demonstration
@Retention(RetentionPolicy.RUNTIME)
@Target({ElementType.METHOD, ElementType.TYPE, ElementType.FIELD, ElementType.PARAMETER})
@interface Marker {
    String value() default "default";
    int level() default 1;
}

// Another annotation with arrays and default values
@interface Multi {
    String[] tags() default {"alpha", "beta", "gamma"};
    Class<?> clazz() default Object.class;
}

/**
 * Top-level public class with many inner classes, methods, and fields.
 * Purposefully long names used to ensure diffs catch renames.
 */
public class LongTestFile {

    // static constants
    public static final String VERSION = "2025-11-28-test";
    public static final int[] PRIMES = {2, 3, 5, 7, 11, 13, 17, 19, 23};
    private static final Map<String, Integer> registry = new HashMap<>();

    // volatile/transient example fields
    private volatile long volatileCounter = 0L;
    private transient String transientCache = null;

    // large, oddly-indented block
    static {
        registry.put("alpha", 1);
        registry.put("beta", 2);
        registry.put("gamma", 3);

        // intentionally weird formatting to test whitespace sensitivity
            for (int i=0;i<3;i++){
        registry.put("k" + i, i);
            }
    }

    // Constructor(s)
    public LongTestFile() {
        this.initDefaults();
    }

    public LongTestFile(String seed) {
        this();
        this.transientCache = seed;
    }

    // initialization method
    private void initDefaults() {
        // Many default initializations
        this.volatileCounter = System.nanoTime() % 1000;
        this.transientCache = "init:" + UUID.randomUUID().toString();
    }

    // region: utility methods (a lot of them)
    public static String joinStrings(Collection<String> items, CharSequence delim) {
        StringBuilder sb = new StringBuilder();
        boolean first = true;
        for (String s : items) {
            if (!first) sb.append(delim);
            sb.append(s == null ? "NULL" : s);
            first = false;
        }
        return sb.toString();
    }

    public static String joinStrings(String[] items, CharSequence delim) {
        List<String> list = new ArrayList<>();
        if (items != null) {
            for (String s : items) list.add(s);
        }
        return joinStrings(list, delim);
    }

    public static <T> String prettyPrintArray(T[] arr) {
        if (arr == null) return "null-array";
        StringBuilder sb = new StringBuilder();
        sb.append("[");
        for (int i=0;i<arr.length;i++){
            if (i>0) sb.append(", ");
            sb.append(String.valueOf(arr[i]));
        }
        sb.append("]");
        return sb.toString();
    }

    public static String longStringOf(char c, int n) {
        char[] arr = new char[n];
        Arrays.fill(arr, c);
        return new String(arr);
    }

    // purposely duplicate similar methods to create diff noise
    public static String longStringOf(char c, long n) {
        return longStringOf(c, (int)Math.min(Integer.MAX_VALUE, n));
    }

    // complex signature
    public synchronized <K, V extends Comparable<? super V>> Map<K, V> sortByValue(Map<K, V> map) {
        List<Map.Entry<K,V>> list = new ArrayList<>(map.entrySet());
        list.sort((a,b) -> a.getValue().compareTo(b.getValue()));
        Map<K,V> result = new LinkedHashMap<>();
        for (Map.Entry<K,V> e : list) result.put(e.getKey(), e.getValue());
        return result;
    }

    // method with many try-catch blocks and nested resources
    public void complicatedIOOperation(File root, String guess) throws IOException {
        if (root == null) throw new IllegalArgumentException("root==null");
        File f = new File(root, "log-" + System.currentTimeMillis() + ".txt");
        try (FileWriter fw = new FileWriter(f);
             BufferedWriter bw = new BufferedWriter(fw);
             PrintWriter pw = new PrintWriter(bw)) {

            pw.println("Header: " + guess);
            for (int i=0;i<10;i++){
                pw.println("Line " + i + " " + new Date());
            }
            pw.flush();
        } catch (IOException ex) {
            // swallow but rethrow with appended info
            IOException wrapper = new IOException("Failed writing to " + f, ex);
            wrapper.addSuppressed(ex);
            throw wrapper;
        } finally {
            // final cleanup that intentionally does nothing
            if (!f.exists()) {
                // no-op
            }
        }
    }

    // huge switch-case to exercise multi-line edits
    public String codeToWord(int code) {
        switch (code) {
            case 0: return "zero";
            case 1: return "one";
            case 2: return "two";
            case 3: return "three";
            case 4: return "four";
            case 5: return "five";
            case 6: return "six";
            case 7: return "seven";
            case 8: return "eight";
            case 9: return "nine";
            default:
                if (code < 0) return "negative";
                if (code < 100) return "small";
                if (code < 1000) return "medium";
                return "large";
        }
    }

    // many small methods to create large file footprint
    public int sumInts(int... vals) {
        int s=0;
        for (int v: vals) s+=v;
        return s;
    }

    public long sumLongs(long[] vals) {
        long s=0L;
        for (long v: vals) s+=v;
        return s;
    }

    public OptionalDouble averageDoubles(double[] vals) {
        if (vals == null || vals.length==0) return OptionalDouble.empty();
        double s = 0.0;
        for (double v: vals) s+=v;
        return OptionalDouble.of(s/vals.length);
    }

    // Overloaded and oddly-typed methods
    public <T extends Number> double sumGenericNumbers(Iterable<T> numbers) {
        double s = 0;
        for (T n: numbers) s += n.doubleValue();
        return s;
    }

    // weird generics and nested types
    public Map<String, List<Map<Integer, Set<String>>>> deeplyNestedStructure() {
        Map<String, List<Map<Integer, Set<String>>>> outer = new HashMap<>();
        for (int i=0;i<5;i++){
            Map<Integer, Set<String>> innerMap = new HashMap<>();
            for (int j=0;j<3;j++){
                Set<String> set = new LinkedHashSet<>();
                set.add("v-"+i+"-"+j);
                innerMap.put(j, set);
            }
            outer.put("k-"+i, Arrays.asList(innerMap));
        }
        return outer;
    }

    // Anonymous inner class and lambda mixing
    public Runnable produceRunnable(String name) {
        return new Runnable() {
            private final String localName = name;
            @Override
            public void run() {
                System.out.println("Running " + localName);
            }
            public String toString() { return "Runnable["+localName+"]"; }
        };
    }

    // Lambdas and Stream use
    public List<String> numbersAsWords(List<Integer> numbers) {
        if (numbers == null) return Collections.emptyList();
        return numbers.stream()
                .map(i -> codeToWord(i))
                .map(s -> s.toUpperCase(Locale.ROOT))
                .collect(Collectors.toList());
    }

    // Reflection heavy method (intentionally brittle)
    public void reflectivelyInspect(Object obj) {
        if (obj == null) return;
        Class<?> cls = obj.getClass();
        System.out.println("Class: " + cls.getName());
        for (Method m : cls.getDeclaredMethods()) {
            Annotation[] anns = m.getAnnotations();
            for (Annotation a : anns) {
                System.out.println("  method " + m.getName() + " has ann " + a.toString());
            }
        }
    }

    // Long docstring-like comment inside method
    public void methodWithHugeCommentBlock() {
        /*
         * This method is intentionally filled with a huge multi-line comment.
         * The comment contains lists:
         *  - alpha
         *  - beta
         *  - gamma
         *
         * And example pseudo-code:
         *
         * for i in 1..n:
         *    do stuff
         *
         * End comment.
         */
        // minimal runtime content
        Map<String, Integer> local = new HashMap<>();
        local.put("a", 1);
        local.put("b", 2);
        local.getOrDefault("c", 3);
    }

    // region: nested classes
    public static class InnerOne {
        private int id;
        private String label;

        public InnerOne() { this(0, ""); }
        public InnerOne(int id, String label) { this.id = id; this.label = label; }

        public int getId() { return id; }
        public void setId(int id) { this.id = id; }

        public String getLabel() { return label; }
        public void setLabel(String label) { this.label = label; }

        @Override
        public String toString() {
            return "InnerOne{id=" + id + ", label='" + label + "'}";
        }

        // long method with nested loops
        public int compute(int factor) {
            int acc = 0;
            for (int i=0;i<factor;i++){
                for (int j=0;j<3;j++){
                    acc += (i * j) + id;
                }
            }
            return acc;
        }
    }

    private class PrivateInner {
        private final Deque<String> deque = new ArrayDeque<>();
        public void pushAll(String... items) {
            for (String s: items) if (s != null) deque.push(s);
        }
        public String popOne() { return deque.isEmpty() ? null : deque.pop(); }
    }

    protected static final class StaticFinalInner {
        static final BigInteger STATIC_BI = new BigInteger("12345678901234567890");
        public static String constantString() { return "CONST"; }
    }

    // anonymous inner classes in fields
    public final Comparator<String> weirdComparator = new Comparator<String>() {
        @Override
        public int compare(String a, String b) {
            if (a==b) return 0;
            if (a==null) return -1;
            if (b==null) return 1;
            return a.length() - b.length();
        }
    };

    // enum with many variants and methods
    public enum BigEnum {
        ALPHA(1), BETA(2), GAMMA(3), DELTA(4), EPSILON(5), ZETA(6), ETA(7), THETA(8), IOTA(9), KAPPA(10);

        private final int code;
        BigEnum(int c) { this.code = c; }
        public int code() { return code; }
        public boolean isEven() { return (code % 2) == 0; }
        public static BigEnum fromCode(int c) {
            for (BigEnum e : values()) if (e.code == c) return e;
            return null;
        }
    }

    // interface inside class
    public interface IoThing {
        void write(OutputStream os) throws IOException;
        default void writeString(OutputStream os, String s) throws IOException {
            if (s == null) s = "";
            os.write(s.getBytes("UTF-8"));
        }
    }

    // Synthetic generics and wildcards to create more lines
    public <T extends Comparable<? super T>> List<T> sortList(Collection<T> c) {
        List<T> list = new ArrayList<>(c);
        Collections.sort(list);
        return list;
    }

    // intentionally large method with comments and whitespace
    public void giantMethodWithBranches(int x, String[] args) {
        // Start of method
        if (x < 0) {
            // negative path
            System.err.println("negative: " + x);
            return;
        } else if (x == 0) {
            // zero path
            System.out.println("zero");
        } else {
            // positive path
            for (int i=0;i<x;i++) {
                // nested conditional
                if (i % 2 == 0) {
                    // even
                    System.out.println("even " + i);
                } else {
                    // odd
                    System.out.println("odd " + i);
                }
            }
        }

        // another long block of fake processing
        List<String> collected = new LinkedList<>();
        if (args != null) {
            for (String a : args) {
                if (a == null) continue;
                if (a.trim().isEmpty()) continue;
                collected.add(a.trim());
            }
        }

        // mutate collected into a map
        Map<String, Integer> counts = new TreeMap<>();
        for (String s : collected) {
            counts.put(s, counts.getOrDefault(s, 0) + 1);
        }

        // print counts
        counts.forEach((k,v) -> System.out.println(k + " => " + v));

        // final no-op
        if (collected.size() == 0) {
            // intentionally empty to provide diff-friendly blank block
        }
    }

    // multiple overloaded main-like entrypoints to confuse scanners
    public static void main(String[] args) {
        LongTestFile instance = new LongTestFile();
        instance.runSmokeTest();
    }

    public static void pseudoMain(String[] args, boolean verbose) {
        LongTestFile lt = new LongTestFile();
        if (verbose) lt.runSmokeTest();
        else lt.methodWithHugeCommentBlock();
    }

    // smoke test method with many asserts and checks
    public void runSmokeTest() {
        assertVersion();
        testCollections();
        testIOMock();
        testConcurrency();
    }

    private void assertVersion() {
        if (VERSION == null || VERSION.isEmpty()) {
            throw new IllegalStateException("Missing version");
        }
    }

    private void testCollections() {
        List<String> list = new ArrayList<>();
        for (int i=0;i<100;i++) list.add("v"+i);
        if (list.size() != 100) throw new AssertionError("bad size");
        Collections.shuffle(list);
        Map<String,Integer> map = new HashMap<>();
        for (String s: list) map.put(s, s.length());
    }

    private void testIOMock() {
        try {
            complicatedIOOperation(new File(System.getProperty("java.io.tmpdir")), "guess");
        } catch (IOException e) {
            // ignore in tests
        }
    }

    private void testConcurrency() {
        ExecutorService es = Executors.newFixedThreadPool(4);
        List<Future<Integer>> futures = new ArrayList<>();
        for (int i=0;i<8;i++){
            final int idx = i;
            futures.add(es.submit(() -> {
                Thread.sleep(5);
                return idx * idx;
            }));
        }
        try {
            for (Future<Integer> f : futures) {
                f.get(250, TimeUnit.MILLISECONDS);
            }
        } catch (Exception ex) {
            // ignore
        } finally {
            es.shutdownNow();
        }
    }

    // long region of repeated methods to increase file length
    public String repeatHello(int times) {
        StringBuilder sb = new StringBuilder();
        for (int i=0;i<times;i++) sb.append("hello");
        return sb.toString();
    }

    public String repeatHelloWithDelimiter(int times, String delim) {
        StringBuilder sb = new StringBuilder();
        for (int i=0;i<times;i++){
            if (i>0) sb.append(delim);
            sb.append("hello");
        }
        return sb.toString();
    }

    public List<String> generateLabels(int n) {
        List<String> labels = new ArrayList<>();
        for (int i=0;i<n;i++) labels.add(String.format("label-%03d", i));
        return labels;
    }

    public char[] faultyCharArray(String in) {
        if (in == null) return new char[0];
        char[] out = new char[in.length() * 2];
        for (int i=0;i<in.length();i++){
            out[i*2] = in.charAt(i);
            // leave gaps to simulate bugs
        }
        return out;
    }

    // deliberately large inline string literal to create a large diff footprint if changed
    private static final String LARGE_LOREM = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. "
            + "Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, "
            + "quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute "
            + "irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. "
            + "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

    // example method that demonstrates string slicing and edge cases
    public String safeSlice(String s, int start, int end) {
        if (s == null) return "";
        int a = Math.max(0, Math.min(s.length(), start));
        int b = Math.max(a, Math.min(s.length(), end));
        return s.substring(a, b);
    }

    // more nested classes and interfaces
    public static class Factory {
        public static LongTestFile createWithSeed(String seed) {
            return new LongTestFile(seed);
        }
        public static InnerOne makeInner(int id, String label) {
            return new InnerOne(id, label);
        }
    }

    // data holder record-like inner class (pre java-14 safe style)
    public static final class RecordLike {
        private final String a;
        private final int b;
        public RecordLike(String a, int b) { this.a = a; this.b = b; }
        public String a() { return a; }
        public int b() { return b; }
        @Override public String toString() { return "RecordLike{a=" + a + ",b=" + b + "}"; }
    }

    // contrived example of method overloading to produce diff-hotspots
    public String overloadMe(Number n) { return "Number:" + n; }
    public String overloadMe(Integer n) { return "Integer:" + n; }
    public String overloadMe(Double n) { return "Double:" + n; }
    public String overloadMe(Object n) { return "Object:" + n; }

    // many differently-styled comments and spacing to test comment-only diffs
    // single-line comment A
    // single-line comment B

    /**
     * javadoc-style comment block C
     * with multiple lines
     */
    // trailing spaces and tabs below follow:    	

    // region: intentionally long trailing region with many small methods
    public int chain1(int x) { return chain2(x) + 1; }
    public int chain2(int x) { return chain3(x) + 1; }
    public int chain3(int x) { return chain4(x) + 1; }
    public int chain4(int x) { return chain5(x) + 1; }
    public int chain5(int x) { return x * 2; }

    // method that uses recursion with deep depth to introduce stack-like edits
    public int factorial(int n) {
        if (n <= 1) return 1;
        return n * factorial(n - 1);
    }

    // a long doc comment that contains code-like lines to confuse diffs
    /*
     * if (x > 5) {
     *    doThing();
     * }
     *
     * The above is not code, it's a comment.
     */

    // more utility methods
    public boolean containsIgnoreCase(String src, String search) {
        if (src == null || search == null) return false;
        return src.toLowerCase(Locale.ROOT).contains(search.toLowerCase(Locale.ROOT));
    }

    public Map<String, Integer> countWords(String text) {
        if (text == null || text.isEmpty()) return Collections.emptyMap();
        String[] parts = text.split("\\W+");
        Map<String,Integer> m = new HashMap<>();
        for (String p : parts) {
            if (p.isEmpty()) continue;
            String key = p.toLowerCase(Locale.ROOT);
            m.put(key, m.getOrDefault(key, 0) + 1);
        }
        return m;
    }

    // multiple overloaded compute-like methods to create noise
    public double compute(double a, double b) { return a + b; }
    public double compute(double a, double b, double c) { return a + b + c; }
    public int compute(int a, int b) { return a * b; }
    public long compute(long a, long b) { return a - b; }

    // method with annotations and complex signature
    @Marker(value="special", level=5)
    @Multi(tags={"x","y"})
    public <T> T annotatedMethod(T input) {
        // do weird casting dance
        Object o = input;
        @SuppressWarnings("unchecked")
        T out = (T) o;
        return out;
    }

    // long section of throw-away methods to bloat file length
    public String mkString(Object... parts) {
        StringBuilder sb = new StringBuilder();
        for (Object p : parts) {
            if (sb.length() > 0) sb.append("|");
            sb.append(String.valueOf(p));
        }
        return sb.toString();
    }

    public boolean isEmpty(Collection<?> c) { return c == null || c.isEmpty(); }
    public boolean isNotEmpty(Collection<?> c) { return !isEmpty(c); }

    // utility that constructs a nested map with convoluted types
    public Map<String, Map<Integer, List<String>>> buildNested(int depth) {
        Map<String, Map<Integer, List<String>>> outer = new HashMap<>();
        for (int i=0;i<depth;i++){
            Map<Integer, List<String>> inner = new HashMap<>();
            for (int j=0;j<depth;j++){
                List<String> list = new ArrayList<>();
                for (int k=0;k<depth;k++){
                    list.add(String.format("n-%d-%d-%d", i,j,k));
                }
                inner.put(j, list);
            }
            outer.put("o-"+i, inner);
        }
        return outer;
    }

    // many small string constants to make the file noisy
    private static final String S1 = "alpha";
    private static final String S2 = "beta";
    private static final String S3 = "gamma";
    private static final String S4 = "delta";
    private static final String S5 = "epsilon";
    private static final String S6 = "zeta";
    private static final String S7 = "eta";

    // method with intentional formatting oddities and trailing whitespace
    public String weirdFormatting(String s) {
        if (s == null) return "null";
        String    tidy = s.trim();
        if (tidy.length() == 0)  return "empty";
        return tidy;
    }

    // a method that returns an extremely long string (constructed)
    public String makeHugeString(int n) {
        StringBuilder sb = new StringBuilder();
        for (int i=0;i<n;i++){
            sb.append(LARGE_LOREM);
            if (i % 10 == 0) sb.append(" **BREAK** ");
        }
        return sb.toString();
    }

    // method containing multiple small try-catches to create multiple diff hotspots
    public void manyTryCatches() {
        try {
            int x = Integer.parseInt("123");
        } catch (NumberFormatException e) {
            // ignore
        }
        try {
            URL u = new URL("http://example.com");
            u.openConnection();
        } catch (MalformedURLException mue) {
            // ignore
        } catch (IOException ioe) {
            // ignore
        } finally {
            // no-op
        }
    }

    // method with synchronized block and wait/notify stubs
    public void syncWaitNotify(Object lock) throws InterruptedException {
        synchronized (lock) {
            lock.notifyAll();
            lock.wait(1);
        }
    }

    // method for generating random map using streams to add diversity
    public Map<Integer,String> randomMap(int n) {
        Random r = new Random(12345);
        return IntStream.range(0, n).boxed()
                .collect(Collectors.toMap(i -> i, i -> "v" + r.nextInt(1000)));
    }

    // a long method containing many local classes and shadows
    public void methodWithLocalClasses() {
        class LocalA {
            int a = 1;
            String s() { return "A"; }
        }
        class LocalB extends LocalA {
            @Override String s() { return "B"; }
        }
        LocalA la = new LocalA();
        LocalB lb = new LocalB();
        System.out.println(la.s() + lb.s());
    }

    // method that intentionally uses obscure library classes to create noise
    public BigDecimal computeBigDecimal(BigInteger integer, double mult) {
        if (integer == null) integer = BigInteger.ZERO;
        BigDecimal bd = new BigDecimal(integer);
        bd = bd.multiply(BigDecimal.valueOf(mult));
        return bd;
    }

    // end region marker comment (useful for diffs that insert lines)
    // --- END OF CORE CLASS CONTENT ---

    // Extra trailing whitespace lines follow to make whitespace diffs obvious.


}

