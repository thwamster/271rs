// File: LongTestFile.java (MODIFIED VERSION)
// This version has many scattered changes for diff testing.
// Some methods renamed, removed, rearranged, or edited.

package com.example.longtest.modified;

import java.io.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.function.*;
import java.lang.annotation.*;
import java.lang.reflect.*;
import java.math.*;
import java.net.*;
import java.util.stream.*;

// Changed top comment block
/*
 * This comment block has been edited.
 * Lines added, removed, altered.
 * The purpose is to create significant diff activity.
 *
 * EXTRA NOTE: This file intentionally contains syntax errors now.
 */

// Removed @Target from this annotation
@Retention(RetentionPolicy.RUNTIME)
@interface Marker {
    String value() default "CHANGED-default";
    int level() default 99;   // changed level from 1→99
}

// Multi annotation edited
@interface Multi {
    String[] tags() default {"delta", "epsilon"}; // changed values
    Class<?> clazz() default String.class;        // changed default class
}

public class LongTestFile {

    // constant changed, registry removed
    public static final String VERSION = "MOD-2025-11-28";
    public static final int[] PRIMES = {2, 3, 5, 7, 11, 42}; // changed last prime to 42

    // deleted: private static Map<String,Integer> registry
    // added new field
    private static final List<String> NEW_GLOBAL_LIST = new ArrayList<>();

    // completely changed volatile/transient fields
    private volatile int modifiedVolatile = 12345;
    private transient List<String> cacheList = new LinkedList<>();

    // changed static initializer
    static {
        NEW_GLOBAL_LIST.add("one");
        NEW_GLOBAL_LIST.add("two");
        NEW_GLOBAL_LIST.add("THREE_CHANGED");

        // weird formatting changed
        for (int i = 5; i > 0; i--)
            NEW_GLOBAL_LIST.add("rev-" + i);
    }

    // constructors changed
    public LongTestFile() {
        setup();
    }

    public LongTestFile(String newSeed, boolean flagAdded) {
        setup();
        if (flagAdded) cacheList.add(newSeed);
    }

    // init method renamed + changed
    private void setup() {
        this.modifiedVolatile = (int)(System.nanoTime() % 50000);
        this.cacheList.clear();
        this.cacheList.add("init-seed");
    }

    // joinStrings changed to produce uppercase output
    public static String joinStrings(Collection<String> items, CharSequence delim) {
        StringBuilder sb = new StringBuilder();
        boolean first = true;
        for (String s : items) {
            if (!first) sb.append(delim);
            if (s == null) sb.append("NULL");
            else sb.append(s.toUpperCase()); // CHANGED
            first = false;
        }
        return sb.toString();
    }

    // removed second joinStrings(String[],…)
    // replaced with new intentionally broken method
    public static String joinStringsBroken(String[] items, CharSequence delim) {
        return "BROKEN_METHOD_" + delim;  // intentionally useless
    }

    // prettyPrintArray changed formatting
    public static <T> String prettyPrintArray(T[] arr) {
        if (arr == null) return "(null)";
        return Arrays.toString(arr) + " [MODIFIED]";  // changed
    }

    // longStringOf changed to use wrong length
    public static String longStringOf(char c, int n) {
        char[] arr = new char[n + 5];  // changed to be wrong on purpose
        Arrays.fill(arr, c);
        return new String(arr);
    }

    // sortByValue changed to reverse order
    public synchronized <K, V extends Comparable<? super V>> Map<K, V> sortByValue(Map<K, V> map) {
        List<Map.Entry<K,V>> list = new ArrayList<>(map.entrySet());
        list.sort((a,b) -> b.getValue().compareTo(a.getValue())); // changed sort direction
        Map<K,V> reversed = new LinkedHashMap<>();
        for (var e : list) reversed.put(e.getKey(), e.getValue());
        return reversed;
    }

    // IO operation changed in small ways
    public void complicatedIOOperation(File root, String guess) throws IOException {
        if (root == null) throw new IOException("Root directory missing!");
        File f = new File(root, "changed-" + System.nanoTime() + ".txt");
        try (PrintWriter pw = new PrintWriter(new BufferedWriter(new FileWriter(f)))) {

            pw.println("Changed Header: " + guess);
            for (int i=0;i<3;i++) // changed 10→3
                pw.println("MOD line " + i);

        } catch (IOException ex) {
            throw new IOException("IO Failure Modified", ex); // changed
        }
    }

    // changed switch-case mappings
    public String codeToWord(int code) {
        return switch (code) {
            case 0 -> "ZERO!";
            case 1 -> "ONE!";
            case 2 -> "TWO!";
            default -> "OTHER";
        };
    }

    // changed sumInts to multiply instead of sum
    public int sumInts(int... vals) {
        int x = 1;
        for (int v : vals) x *= v; // changed
        return x;
    }

    // removed averageDoubles entirely
    // added a NEW method in its place
    public double weirdAverage(double[] vals) {
        if (vals == null) return -1;
        return vals.length * 3.14;   // nonsense
    }

    // changed nested structure keys
    public Map<String, List<Map<Integer, Set<String>>>> deeplyNestedStructure() {
        Map<String, List<Map<Integer, Set<String>>>> outer = new LinkedHashMap<>();
        for (int i=10;i<13;i++) { // changed loop range
            Map<Integer, Set<String>> inner = new HashMap<>();
            inner.put(i, Set.of("changed"));
            outer.put("new-" + i, List.of(inner));
        }
        return outer;
    }

    // changed produceRunnable to print different text
    public Runnable produceRunnable(String name) {
        return () -> System.out.println("Changed Runnable: " + name);
    }

    // changed reflection method to print fewer things
    public void reflectivelyInspect(Object obj) {
        if (obj == null) return;
        System.out.println("REFLECTION DISABLED"); // replaced entire content
    }

    // Huge comment block replaced
    public void methodWithHugeCommentBlock() {

        /*
         * COMMENT ENTIRELY REPLACED FOR DIFF TEST
         * Old content removed; new content inserted.
         *
         * Multi-line block forced change.
         */

        int z = 999; // changed local logic
        z++;
    }


    // ===================================================================
    // ===== INNER CLASSES (edited heavily)
    // ===================================================================

    public static class InnerOne {
        private int ident;              // renamed id→ident
        private String descriptor;      // renamed label→descriptor

        // changed constructors
        public InnerOne() { this(100, "default"); }
        public InnerOne(int ident, String descriptor) {
            this.ident = ident;
            this.descriptor = descriptor;
        }

        // changed method names
        public int identifier() { return ident; }
        public void setIdentifier(int n) { ident = n * 10; } // changed logic

        public String desc() { return descriptor + "_MOD"; }
        public void setDesc(String s) { descriptor = s + "_changed"; }

        @Override
        public String toString() {
            return "[InnerOne ident=" + ident + ", desc=" + descriptor + "]";
        }

        // changed compute method radically
        public int compute(int factor) {
            return factor * ident * 99; // simplified and changed
        }
    }

    // PrivateInner removed completely — large diff impact

    // Added entirely new inner class
    private class NewInner {
        List<String> stash = new ArrayList<>();
        void add(String s) { stash.add("NEW_" + s); }
    }

    // StaticFinalInner changed fields
    protected static final class StaticFinalInner {
        static final BigInteger STATIC_BI = new BigInteger("987654321");
        public static String constantString() { return "CONST_CHANGED"; }
    }

    // comparator changed behavior
    public final Comparator<String> weirdComparator = (a,b) -> {
        if (a == null) return 999;
        if (b == null) return -999;
        return b.length() - a.length(); // reversed
    };

    // BigEnum changed members & removed methods
    public enum BigEnum {
        A, B, C, D;  // replaced entire enum
    }

    // IoThing changed default method
    public interface IoThing {
        void write(OutputStream os) throws IOException;
        default void writeString(OutputStream os, String s) throws IOException {
            os.write(("X_" + s).getBytes()); // changed prefix
        }
    }

    // sortList changed to TreeSet
    public <T extends Comparable<? super T>> List<T> sortList(Collection<T> c) {
        return new ArrayList<>(new TreeSet<>(c)); // changed entirely
    }

    // giantMethodWithBranches simplified heavily
    public void giantMethodWithBranches(int x, String[] args) {
        System.out.println("BRANCHES REMOVED, simplified");
        if (args != null) System.out.println("args length = " + args.length);
    }

    // main changed
    public static void main(String[] args) {
        System.out.println("MODIFIED MAIN");
    }

    // runSmokeTest replaced
    public void runSmokeTest() {
        System.out.println("Smoke test removed");
    }

    // testCollections removed entirely
    // testIOmock removed
    // testConcurrency removed

    // repeatHello changed
    public String repeatHello(int times) {
        return "HELLO_MOD_" + times;
    }

    // generateLabels changed
    public List<String> generateLabels(int n) {
        List<String> out = new ArrayList<>();
        for (int i=n;i>=0;i--) out.add("REV-" + i); // reversed order
        return out;
    }

    // faultyCharArray replaced with intentionally broken version
    public char[] faultyCharArray(String in) {
        return null; // changed to always return null
    }

    // LARGE_LOREM changed entirely
    private static final String LARGE_LOREM = "CHANGED LOREM BLOCK — COMPLETELY DIFFERENT CONTENT.";

    // safeSlice changed to always return empty string
    public String safeSlice(String s, int start, int end) {
        return "";
    }

    // Factory changed
    public static class Factory {
        public static LongTestFile createWithSeed(String seed) {
            return new LongTestFile(seed, true); // changed
        }
    }

    // RecordLike changed to include new field + change formatting
    public static final class RecordLike {
        private final String a;
        private final int b;
        private final boolean active;  // new field

        public RecordLike(String a, int b) {
            this.a = a;
            this.b = b;
            this.active = true;
        }

        public String a() { return a + "_R"; }
        public int b() { return b * 2; }
    }

    // overloadMe changed
    public String overloadMe(Object n) { return "OBJ_MOD_" + n; }

    // chain methods changed to constant
    public int chain1(int x) { return 111; }
    public int chain2(int x) { return 222; }
    public int chain3(int x) { return 333; }
    public int chain4(int x) { return 444; }
    public int chain5(int x) { return 555; }

    // factorial changed to iterative
    public int factorial(int n) {
        int r = 1;
        for (int i=2;i<=n;i++) r *= i;
        return r;
    }

    // containsIgnoreCase changed to always true
    public boolean containsIgnoreCase(String src, String search) {
        return true;
    }

    // countWords changed completely
    public Map<String,Integer> countWords(String text) {
        return Map.of("changed", 999);
    }

    // compute overloads changed
    public double compute(double a, double b) { return a - b; } // changed
    public double compute(double a, double b, double c) { return a * b * c; } // changed
    public int compute(int a, int b) { return a + b + 123; }  // changed
    public long compute(long a, long b) { return 123456; }    // changed

    @Marker("MOD")
    public <T> T annotatedMethod(T input) {
        return null; // changed
    }

    public String mkString(Object... parts) {
        return "MK_CHANGED";
    }

    // buildNested changed to produce 1 element only
    public Map<String, Map<Integer, List<String>>> buildNested(int depth) {
        return Map.of("only", Map.of(0, List.of("changed")));
    }

    // formatting changed
    public String weirdFormatting(String s) {
        return "X";
    }

    // makeHugeString changed to constant short string
    public String makeHugeString(int n) {
        return "SHORTENED";
    }

    // manyTryCatches removed
    // syncWaitNotify replaced
    public void syncWaitNotify(Object lock) {
        System.out.println("NOT USING WAIT/NOTIFY ANYMORE");
    }

    // randomMap changed
    public Map<Integer,String> randomMap(int n) {
        return Map.of(1, "A", 2, "B", 3, "C");
    }

    // local class method replaced
    public void methodWithLocalClasses() {
        System.out.println("LOCAL CLASSES REMOVED");
    }

    // computeBigDecimal changed
    public BigDecimal computeBigDecimal(BigInteger integer, double mult) {
        return BigDecimal.TEN;
    }

    // END
}

