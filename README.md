# URanker

URanker is a CLI tool abling to rank top-100 URLs from URL datasets.

## Data input

Input URL data should be contained in a text file, that contains URLs separated by UNIX linefeed `\n` or space ` `.

For example:

```text
cn.bing.com
google.com.hk
www.gnuradio.org mirrors.tuna.tsinghua.edu.cn
```

By the way, a single URL pattern should no longer than 50 characters.

## Mechanisms

### Large file reading

Sometimes the file is too big to be stored in memory, so we have to read it as separate data blocks.

### Map

Once a data block has been readout, the program will pass it to a standby `Mapper`.

`Mapper` takes in data blocks to produce `key`-`value` pairs. Every unique URL will be stored as `key` with its number of occurrences will be stored as `value`.

`Mapper` then write `key`-`value` pairs to a buffer file.

### Reduce

When all `Mapper` done their jobs, `Reducer`s show up.

`Reducer`s read KV pairs from buffer files and merge pairs containing same `key`s. `Reducer`s only care about `key`s that in their duty.

`Reducer` will store its outcome into another buffer file.

### Rank

After the `Map` and `Reduce` phase, the intermediate data should shrink a lot compared to the raw dataset.

`Ranker` takes in all `Reducer`'s intermediate file and sort KV pairs by value. Then produce top-100 frequency URLs.
