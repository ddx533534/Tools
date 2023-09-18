import org.json.JSONObject;

public class Json_Tool {
    public static void main(String[] args) {
        // 需要判断的字符串
        String str = "";
        algorithm_1(str);
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
