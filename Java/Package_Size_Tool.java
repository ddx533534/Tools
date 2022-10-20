import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.OutputStreamWriter;
import java.util.Base64;

public class Package_Size_Tool {
    public static void main(String[] args) {
        // 需要写入的文件的路径
        String filePath = "/Users/ddx/Desktop/useless/relese_10.3.0_package.json";
        algorithm_1(filePath, false, "UTF-8", 1250);
    }

    /**
     * 填补包大小算法1：填补 json 数据，提供压缩后文件大小。
     * 
     * @param FilePath        文件路径
     * @param append          是否采用追加模式写入
     * @param charsetName     写入格式
     * @param fileSize(bytes) 期望文件大小
     */
    private static void algorithm_1(String FilePath, boolean append, String charsetName, int fileSize) {
        try {
            if (fileSize <= 0) {
                return;
            }
            long line = (long) (fileSize / 0.024);
            File file = new File(FilePath);
            FileOutputStream fos = new FileOutputStream(file, append);
            OutputStreamWriter osw = new OutputStreamWriter(fos, charsetName);

            long number = Long.MIN_VALUE;
            osw.write("{\r\n");
            for (int i = 1; i <= line; i++) {
                number = number * i + i;
                String base64 = Base64.getEncoder()
                        .encodeToString(String.valueOf(number).getBytes(charsetName));
                osw.write('"' + String.valueOf(number) + '"' + ":" + '"' + base64 + '"' + ",");
                osw.write("\r\n");
            }
            osw.write("}");
            // 写入完成关闭流
            osw.close();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    /**
     * 填补包大小算法2：填补 json 数据，提供压缩前文件大小。
     * 
     * @param FilePath       文件路径
     * @param append         是否采用追加模式写入
     * @param charsetName    写入格式
     * @param fileSize(byte) 期望文件大小
     */
    private static void algorithm_2(String FilePath, boolean append, String charsetName, int fileSize) {
        try {
            File file = new File(FilePath);
            FileOutputStream fos = new FileOutputStream(file, append);
            OutputStreamWriter osw = new OutputStreamWriter(fos, charsetName);

            FileInputStream fin = null;
            long number = Long.MIN_VALUE;
            int i = 1;
            osw.write("{\r\n");
            while (true) {
                number = number * i + i;
                i++;
                String base64 = Base64.getEncoder()
                        .encodeToString(String.valueOf(number).getBytes(charsetName));
                osw.write('"' + String.valueOf(number) + '"' + ":" + '"' + base64 + '"' + ",");
                osw.write("\r\n");
                fin = new FileInputStream(file);
                if (fin.getChannel().size() > 102400) {
                    fin.close();
                    break;
                }
            }
            osw.write("}");
            // 写入完成关闭流
            osw.close();
        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}