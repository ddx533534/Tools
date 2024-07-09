import java.util.Random;

public class Sort {

    public static final int SELECT_SORT_TYPE = 1;
    public static final int INSERT_SORT_TYPE = 2;

    public static void main(String[] args) {
        // int[] arr = new int[] { 3, 2, 1, 4, 9, 7, 8, 0 };
        // insertSort(arr);
        // assert isSorted(arr);
        // printArray(arr);
        Random random = new Random();
        long select_time = 0;
        long insert_time = 0;
        int[] arr1 = new int[100];
        int[] arr2 = new int[100];
        for (int i = 0; i < 1000; i++) {

            for (int j = 0; j < arr1.length; j++) {
                arr1[j] = random.nextInt(1000);
                arr2[j] = arr1[j];
            }
            select_time += time(SELECT_SORT_TYPE, arr1);
            insert_time += time(INSERT_SORT_TYPE, arr2);
        }
        System.out.println("select time: " + select_time);
        System.out.println("insert time: " + insert_time);
        return;
    }

    /**
     * 排序耗时
     * 
     * @param type 算法类型
     * @param arr  待排序数组
     * @return
     */
    private static long time(int type, int[] arr) {
        long start = System.currentTimeMillis();
        switch (type) {
            case SELECT_SORT_TYPE:
                selectSort(arr);
                break;
            case INSERT_SORT_TYPE:
                insertSort(arr);
            default:
                break;
        }
        return System.currentTimeMillis() - start;
    }

    /**
     * 选择排序
     * 每轮排序挑选一个最小的数，放到最前面，依次进行下去
     * 
     * @param arr
     */
    public static void selectSort(int[] arr) {
        if (arr == null || arr.length < 2) {
            return;
        }
        for (int i = 0; i < arr.length; i++) {
            int minIndex = i;
            for (int j = i + 1; j < arr.length; j++) {
                if (arr[minIndex] > arr[j]) {
                    minIndex = j;
                }
            }
            // do swap
            int temp = arr[i];
            arr[i] = arr[minIndex];
            arr[minIndex] = temp;
        }
    }

    /**
     * 插入排序
     * 
     * @param arr 待排序数组
     */
    public static void insertSort(int[] arr) {
        if (arr == null || arr.length < 2) {
            return;
        }
        for (int i = 1; i < arr.length; i++) {
            for (int j = i; j - 1 >= 0 && arr[j] < arr[j - 1]; j--) {
                int temp = arr[j];
                arr[j] = arr[j - 1];
                arr[j - 1] = temp;
            }
        }
    }

    public static boolean isSorted(int[] arr) {
        for (int i = 1; i < arr.length; i++) {
            if (arr[i] < arr[i - 1]) {
                return false;
            }
        }
        return true;
    }

    public static void printArray(int[] arr) {
        for (int i : arr) {
            System.out.printf("%d ", i);
        }
    }
}
