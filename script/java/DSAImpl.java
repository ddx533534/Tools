import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.FileOutputStream;
import java.io.IOException;
import java.io.ObjectInputStream;
import java.io.ObjectOutputStream;
import java.security.KeyPair;
import java.security.KeyPairGenerator;
import java.security.PrivateKey;
import java.security.PublicKey;
import java.security.Signature;

/**
 * DSA 算法 - 数字签名和认证，为非对称加密算法 - 私钥和公钥。
 */
public class DSAImpl {

    public static void main(String[] args) {
        generateKeys();
    }

    public static void generateKeys() {
        try {
            // 1.获取密钥对生成器，标明加密算法
            KeyPairGenerator keyGenerator = KeyPairGenerator.getInstance("DSA");
            // 2.密钥对生成器初始化，标明密钥大小
            keyGenerator.initialize(512);
            // 3.获取密钥对
            KeyPair keyPair = keyGenerator.generateKeyPair();
            // 4.获取私钥
            PrivateKey privateKey = keyPair.getPrivate();
            // 5.获取公钥
            PublicKey publicKey = keyPair.getPublic();

            ObjectOutputStream privateKeyOutputStream = new ObjectOutputStream(
                    new FileOutputStream("./script/data/privateKey.key"));
            privateKeyOutputStream.writeObject(privateKey);
            privateKeyOutputStream.close();

            ObjectOutputStream publicKeyOutputStream = new ObjectOutputStream(
                    new FileOutputStream("./script/data/publicKey.key"));
            publicKeyOutputStream.writeObject(publicKey);
            publicKeyOutputStream.close();

            System.out.println("私钥为：" + privateKey + "\nalgorithm:" + privateKey.getAlgorithm() + "\nformat:"
                    + privateKey.getFormat() + "\nencoded:" + privateKey.getEncoded());
            System.out.println("公钥为：" + publicKey + "\nalgorithm:" + publicKey.getAlgorithm() + "\nformat:"
                    + publicKey.getFormat() + "\nencoded:" + publicKey.getEncoded());
        } catch (Exception e) {
            // TODO Auto-generated catch block
            e.printStackTrace();
        }
    }

    /**
     * 私钥进行数字签名
     */
    public static void signature(String info) {
        try {
            ObjectInputStream objectInputStream = new ObjectInputStream(
                    new FileInputStream("./script/data/privateKey.key"));
            PrivateKey privateKey = (PrivateKey) objectInputStream.readObject();
            objectInputStream.close();

            Signature signature = Signature.getInstance("DSA");
            signature.initSign(privateKey);
            signature.update(info.getBytes());

        } catch (Exception e) {
            // TODO Auto-generated catch block
            e.printStackTrace();
        }
    }
}