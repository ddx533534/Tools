import org.json.JSONObject;

public class Json_Tool {
    public static void main(String[] args) {
        testLoop();
    }

    static void testLoop() {
        int LIMIT = 1000000;
        int loop = 100;
        int i = 0;
        int res = 0;
        long start = System.currentTimeMillis();
        while (true) {
            if (i > LIMIT) {
                break;
            }
            i++;
            for (int j = 0; j < loop; j++) {
                res += j;
            }
            res = 0;
        }
        long end = System.currentTimeMillis();
        System.out.println("time cost " + (end - start) + "ms");
    }

    /**
     * 判断传入的字符串是 JSONObject 还是 JSONArray
     * 
     * @param jsonStr
     */
    public static void algorithm_1(String jsonStr) {

        Object resObj = new JSONTokener(jsonStr).nextValue();// obj为需要判断的对象

        if (resObj instanceof JSONObject) {
            System.out.println("对象是JSONObject: " + resObj);
        } else if (resObj instanceof JSONArray) {
            System.out.println("对象是JSONArray: " + resObj);
        } else {
            System.out.println("Neither jsonStr nor jsonarray is jsonObj");
        }
    }

}
