# adler

| 版本   | 作用                               |
| ------ | ---------------------------------- |
| v1.0.2 | 用来计算一段字节流的Alder-32校验和 |

关于Adler校验和的内容，可以参考[Adler-32校验算法](https://blog.csdn.net/liujiayu2/article/details/51685481)这篇博客。它比CRC（循环冗余）算法更快，但安全性略低。

使用这个库时，新建一个`Alder32`实例，然后通过`write_slice`方法向其中写入`&[u8]`格式的数据。若有计算复杂类型的校验和的需求，请不要使用官方文档中的写法（因为那种写法要求`std`环境）。可以改用`parity-scale-codec`实现从复杂类型到字节流的转化。

```rust
pub fn test_adler() {
    let mut adler = Adler32::new();
    let a = Test(String::from("Adler-32 test."));
    let a_encoded = a.using_encoded(|ref slice| String::from_utf8_lossy(slice).into_owned());
    adler.write_slice(a_encoded.as_bytes());
    println!("Adler32 checksum of {:?} is {:?}", a, adler.checksum());
}
```
