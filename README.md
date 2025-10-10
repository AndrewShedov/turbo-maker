[![Discord](https://img.shields.io/discord/1006372235172384849?style=for-the-badge&logo=discord&logoColor=white&labelColor=black&color=%23f3f3f3&label=)](https://discord.gg/ENB7RbxVZE)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge&logo=5865F2&logoColor=black&labelColor=black&color=%23f3f3f3)](https://github.com/AndrewShedov/turbo-maker/blob/main/LICENSE)

# turbo-maker

### The crate is in the development and testing stage.<br>

**Superfast**, **multithreaded** document generator for **MongoDB**, operating through **CLI**.<br>
Generates **millions of documents** at **maximum speed**, utilizing **all CPU threads**.<br>

###  Suitable for

- Creating big collections (exceeding **500,000,000 documents**)
- Generating synthetic data
- Stress testing MongoDB
- Performance benchmarking

### Features

1. **Multithreading** — each thread inserts documents in parallel.
2. **Specify the number of threads** for data generation to adjust CPU load, **or set it to** <code>max</code> to utilize all available threads.
3. Document distribution across threads considering the remainder.
4. Precise <code>created_at</code>/<code>updated_at</code> handling with <code>time_step_ms</code>.
5. <code>Batch</code> inserts for enhanced performance.
6. Progress bar in the console with percentage, speed, and statistics, along with other informative logs:

<img src="https://raw.githubusercontent.com/AndrewShedov/turbo-maker/refs/heads/main/assets/gif.gif" width="590" /><br>
Generation of **1,000,000 documents** in **2 seconds**, filled with the following [content](https://github.com/AndrewShedov/turbo-maker/blob/main/config%20examples/lite/turbo-maker.config.toml).<br>
PC configuration: Intel i5-12600K, 80GB DDR4 RAM, Samsung 980 PRO 1TB SSD.

### Technologies used

- Tokio
- std::sync::atomic
- sysinfo
- clap / serde / toml

The generator works and fully performs its task of multithreaded document insertion. However, additional data generation functions (random text, numbers, etc.) are still under development.

### Installation & Usage

1. Install crate:

```bash
cargo install turbo-maker
```

2. Create a config file — [turbo-maker.config.toml](https://github.com/AndrewShedov/turbo-maker/tree/main/config%20examples/common/turbo-maker.config.toml) in a convenient location.

3. Run turbo-maker with the path to your config file:

**Windows**:
```bash
turbo-maker --config-path C:\example\turbo-maker.config.toml
```

**Linux/macOS**:
```bash
turbo-maker --config-path /home/user/example/turbo-maker.config.toml
```

### Explanation of the file structure — turbo-maker.config.js.

### [settings]

Required fields must be specified:

```toml
[settings]
uri = "mongodb://127.0.0.1:27017"
db = "crystal"
collection = "posts"
number_threads = "max"
number_documents = 1_000_000
batch_size = 10_000
time_step_ms = 20
```

**number_threads**

Accepts either a <code>string</code> or a <code>number</code> and sets the number of CPU threads used.
- for value <code>"max"</code>, all threads are used.
- if the <code>number</code> exceeds the actual thread count, all threads are used.

**number_documents**

Accepts a <code>number</code>, specifying how many documents to generate.

**batch_size**

Accepts a <code>number</code> of documents per batch inserted into the database.

- the larger the batchSize, the fewer requests MongoDB makes, leading to faster insertions.
- however, a very large batchSize can increase memory consumption.
- the optimal value depends on your computer performance and the number of documents being inserted.

**time_step_ms**

Accepts <code>number</code> and sets the time interval between documents.

- With the value of <code>0</code> a large number of documents will have the same date of creation, due to a high generation rate, especially in multithreaded mode.

### [document_fields]

```toml
[document_fields]
complex_string = {function = "generate_long_string", length = 100}
text = "example" 
created_at = "custom_created_at"
updated_at = "custom_updated_at"
```

All fields in this section are optional. If there are no fields, empty documents will be created in the quantity specified in the field -<code>number_documents</code>, the documents will contain only - <code>_id: ObjectId('68dc8e144d1d8f5e10fdbbb9')</code>.

The <code>complex_string</code> field contains the <code>generate_long_string</code> function, a built-in function created for testing the generator's speed. In <code>length = 100</code>, you can specify the number of random characters to generate.

The <code>text = "example"</code> field is custom and can have any name.

### created_at & updated_at

These are special fields that may be missing, in which case the document will not have a creation date. The time step between documents is specified using the <code>time_step_ms</code> field in the <code>[settings]</code> section.<br>

<code>created_at = "custom_created_at"</code> → custom field name.

<code>created_at = ""</code> → use the default field name created_at.

<code>updated_at</code> repeats the value <code>created_at</code>

## Comparison of Node.js and Rust version of the generator

In comparative hybrid (CPU | I/O) tests, the Rust generator demonstrated **7.87 times (687%)** higher performance compared to the [Node.js version](https://www.npmjs.com/package/turbo-maker):


<img src="https://raw.githubusercontent.com/AndrewShedov/turbo-maker/refs/heads/main/assets/screenshot_1.png" width="640" /><br>
**Rust**
 

<img src="https://raw.githubusercontent.com/AndrewShedov/turbo-maker/refs/heads/main/assets/screenshot_2.png" width="640" /><br>
**Node.js**

PC configuration: Intel i5-12600K, 80GB DDR4 RAM, Samsung 980 PRO 1TB SSD.<br>
The test generated random strings of 500 characters.
It primarily stresses the CPU but also creates I/O load.<br>
[Test code](https://github.com/AndrewShedov/turboMaker/blob/main/config%20examples/Parallel%20Computation%20Benchmark/turbo-maker.config.js) for the Node.js version.<br>
Test code for the Rust version:

```rust
pub fn generate_long_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut result = String::with_capacity(length);
    for _ in 0..length {
        let char_code = rng.gen_range(65..91); // Letters A-Z
        result.push((char_code as u8 as char).to_ascii_uppercase());
        for _ in 0..1000 {
            let _ = rng.gen::<u8>(); // Empty operation for load
        }
    }
    result
}
```
<br>

[![SHEDOV.TOP](https://img.shields.io/badge/SHEDOV.TOP-black?style=for-the-badge)](https://shedov.top/) 
[![CRYSTAL](https://img.shields.io/badge/CRYSTAL-black?style=for-the-badge)](https://crysty.ru/AndrewShedov)
[![Discord](https://img.shields.io/badge/Discord-black?style=for-the-badge&logo=discord&color=black&logoColor=white)](https://discord.gg/ENB7RbxVZE)
[![Telegram](https://img.shields.io/badge/Telegram-black?style=for-the-badge&logo=telegram&color=black&logoColor=white)](https://t.me/ShedovChannel)
[![X](https://img.shields.io/badge/%20-black?style=for-the-badge&logo=x&logoColor=white)](https://x.com/AndrewShedov)
[![VK](https://img.shields.io/badge/VK-black?style=for-the-badge&logo=vk)](https://vk.com/shedovchannel)
[![VK Video](https://img.shields.io/badge/VK%20Video-black?style=for-the-badge&logo=vk)](https://vkvideo.ru/@shedovchannel)
[![YouTube](https://img.shields.io/badge/YouTube-black?style=for-the-badge&logo=youtube)](https://www.youtube.com/@AndrewShedov)
