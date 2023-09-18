import java.io.File;
import java.io.FileInputStream;
import java.io.FileOutputStream;
import java.io.OutputStreamWriter;
import java.math.BigInteger;
import java.security.MessageDigest;
import java.security.NoSuchAlgorithmException;
import java.util.Base64;

/**
 * 指定包大小生成工具，采用 json 文件格式，可以提供 json 文件压缩前后两种生成算法。
 */
public class Package_Size_Tool {
    public static void main(String[] args) {
        // 需要写入的文件的路径
        String filePath = "/Users/ddx/Desktop/useless/reqid_white_list.json";
        algorithm_1(filePath, false, "UTF-8", 50);
    }

    /**
     * 填补包大小算法1：填补 json 数据，提供压缩后文件大小。
     *
     * @param FilePath     文件路径
     * @param append       是否采用追加模式写入
     * @param charsetName  写入格式
     * @param fileSize(kb) 期望文件大小
     */
    private static void algorithm_1(String FilePath, boolean append, String charsetName, int fileSize) {
        try {
            if (fileSize <= 0) {
                return;
            }
            long line = (long) (fileSize / 0.0478);
            File file = new File(FilePath);
            FileOutputStream fos = new FileOutputStream(file, append);
            OutputStreamWriter osw = new OutputStreamWriter(fos, charsetName);

            long curTime = System.currentTimeMillis();
            osw.write("{\r\n");
            for (int i = 1; i <= line; i++) {
                String key = getMD5(String.valueOf(i + curTime).getBytes());
                String value = Base64.getEncoder()
                        .encodeToString(key.getBytes(charsetName));
                osw.write('"' + key + '"' + ":" + '"' + value + '"' + ",");
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
            int i = 1;
            long curTime = System.currentTimeMillis();

            osw.write("{\r\n");
            while (true) {
                String key = getMD5(String.valueOf(i + curTime).getBytes());
                String value = Base64.getEncoder()
                        .encodeToString(key.getBytes(charsetName));
                osw.write('"' + key + '"' + ":" + '"' + value + '"' + ",");
                osw.write("\r\n");
                i++;
                fin = new FileInputStream(file);
                if (fin.getChannel().size() > fileSize) {
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

    /**
     * 采用md5 数字签名算法生成key，减少key的碰撞性。
     * 
     * @param bytes 源字节数组
     * @return 十六制进的字符串
     */
    private static String getMD5(byte[] bytes) {
        try {
            MessageDigest md = MessageDigest.getInstance("MD5");
            // 计算md5函数
            md.update(bytes);
            // digest()最后确定返回md5 hash值，返回值为8为字符串。因为md5
            return new BigInteger(1, md.digest()).toString(16);
        } catch (NoSuchAlgorithmException e) {
            e.printStackTrace();
        }
        return "";
    }
}