import java.util.HashMap;

public class test {
    public static void main(String[] args) {
        LRUCache lruCache = new LRUCache(2);
        lruCache.put(1, 1);
        lruCache.put(2, 2);
        lruCache.put(3, 3);
        lruCache.put(2, 22);
        lruCache.get(3);

    }
}

class LRUCache {

    int capacity;
    int size;
    Node head;
    Node tail;
    HashMap<Integer, Node> hashMap;

    public LRUCache(int capacity) {
        hashMap = new HashMap<>(capacity * 2);
        this.capacity = capacity;
        size = 0;
        head = null;
        tail = null;
    }

    public int get(int key) {
        if (hashMap.containsKey(key)) {
            Node n = hashMap.get(key);
            moveToHead(n);
            return n.val;
        }
        System.out.println(toString());
        return -1;
    }

    public void put(int key, int value) {
        if (!hashMap.containsKey(key)) {
            Node N = new Node(null, null, key, value);
            hashMap.put(key, N);
            insertToHead(N);
        } else {
            hashMap.get(key).val = value;
            moveToHead(hashMap.get(key));
        }
        System.out.println(toString());
    }

    public void insertToHead(Node n) {
        if (n == null) {
            return;
        }
        if (head == null) {
            head = n;
            tail = n;
        } else {
            n.next = head;
            head.pre = n;
            head = n;
        }
        increaseSize();
    }

    public void moveToHead(Node n) {
        if (n == null) {
            return;
        }
        if (n == head) {
            return;
        } else if (n == tail) {
            tail = n.pre;
            tail.next = null;
        } else {
            // 非头尾节点
            n.pre.next = n.next;
            n.next.pre = n.pre;
        }
        n.next = head;
        head.pre = n;
        n.pre = null;
        head = n;
    }

    public void deleteTail() {
        if (tail == null) {
            return;
        }
        hashMap.remove(tail.key);
        if (tail.pre == null) {
            head = null;
            tail = null;
        } else {
            Node pre = tail.pre;
            pre.next = null;
            tail = pre;
        }
    }

    public void increaseSize() {
        size++;
        if (size > capacity) {
            deleteTail();
            size--;
        }
    }

    @Override
    public String toString() {
        StringBuilder stringBuilder = new StringBuilder();
        Node n = head;
        while (n != null) {
            stringBuilder.append(n.key + ":" + n.val + "\n");
            n = n.next;
        }
        return "LRUCache{" +
                "size=" + size +
                "\n" + stringBuilder.toString() +
                "\n" + hashMap.toString() +
                '}';
    }
}

class Node {
    Node next;
    Node pre;
    int key;
    int val;

    public Node(Node next, Node pre, int key, int val) {
        this.next = next;
        this.pre = pre;
        this.key = key;
        this.val = val;
    }
}