# URanker

URanker is a CLI tool abling to rank top-100 URLs from URL datasets.

## Usage

Build from source:

```bash
# use release flag to get better performance.
cargo build --release
```

```bash
uranker <dataset>
```

The program will generate a `report.csv` in the current directory ( ./ directory).

## Data input

Input URL data should be contained in a text file, that contains URLs separated by UNIX linefeed `\n`.

For example:

```text
https://cn.bing.com
https://google.com.hk
https://www.gnuradio.org
https://mirrors.tuna.tsinghua.edu.cn
```

By the way, a single URL pattern should no longer than 50 characters.

## Mechanisms

### Large file reading

Sometimes the file is too big to be stored in memory, so we have to read it as separate data blocks.

### Map

Once a data block has been readout, the program will pass it to a standby `Mapper`.

`Mapper` takes in data blocks to produce `key`-`value` pairs. Every unique URL will be stored as `key` with its number of occurrences will be stored as `value`.

`Mapper` then write `key`-`value` pairs one-by-one into a buffer file.

### Reduce

When all `Mapper` done their jobs, `Reducer`s show up.

`Reducer`s read KV pairs from buffer files and merge pairs containing same `key`s. `Reducer`s only care about `key`s that in their duty.

To improve performance and decrease memory occupation, each `Reducer` maintains a 100-element-long `Vec` instead of `HashMap`. `Reducer` will linearly sift through its vector on each insertion. 

`Reducer` will store its outcome into another buffer file.

### Rank

After the `Map` and `Reduce` phase, the intermediate data should shrink a lot compared to the raw dataset.

`Ranker` takes in all `Reducer`'s intermediate file and sort KV pairs by value. Then produce top-100 frequency URLs.

## Project structure and detailed introduction

### reader

Reader module copes with large file reading. 

Inspired by `BufReader`'s split() method, a structure `IterReader` was constructed and derived with `Iterator` trait. So that we can pick up URLs by merely iterating through the whole file regardless of overflowing buffer or memory.

Once `IterReader` finds a long URL (about two times larger than its buffer), she will calculate the URL's hash and replace the long URL with `uranker://<URL's Hash>` and record where the URL first appeared in the source file.

However, it is not enough by reading with `IterReader`. To cooperate with mappers on other threads, `MyReader` takes `IterReader`'s URLs, packs up them and send them to `Mapper`s through synchronic channels. 

> By the way, this is the most time-consumming phase. There should be further development taken to improve performance.

### mul_thread

At the beginning, the module was named `map_reduce`. In fact, it was actually not constructed according to `MapReduce` model that its name was replaced.

The module pack ups operations into functions to make the program easier to debug. The crate `threadpool` was used to make multiple workers while not exceeding the physical limit of mechine.

This module also takes the final URL ranking as its duty.

### ranker

This module implemented some detailed functions used in `map` and `reduce` phase.
