# Ch 3 (pages 67 - 80)

The algorithm of and lsm-tree out of sstables is essentially what is used in LevelDB and RocksDB.

Terms:
- `SSTables` sorted string tables
- `LSM-Tree` log structured merge tree - the basic idea is that keeping a cascade of sstables that are background merged
  - supports range queries
  - key value db
  - high throughput
- `SSTable` and `memtable` terms introduced in Google's Bigtable paper
- `Bloom Filters` memory efficient structure for approximating a set in order to tell you if a key exists
- compaction
  - `size tiered` 
  - `leveled`

Sorted structures:
- `B-Trees` on disk structure
- `Red Black Trees` and `AVL Trees` in memory structures
  - sometimes called `memtable`