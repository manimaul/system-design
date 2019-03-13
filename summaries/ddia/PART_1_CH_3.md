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


# Ch 3 (pages 80 - 90)

Comparing characteristics of database index strategies. 

- BTrees - faster reads (typically)


- LSMTrees - faster writes (typically)
    - Advantages
        - lower write amplification
        - more compact
    - Disadvantages
        - compaction process leads to more unpredictable average response times
        - misconfiguration can lead to compaction process not keeping up with writes

- RTrees - multi-dimensional index for ranged queries

- FTS and fuzzy indexes
    - these are lightly covered and not described in detail

- In memory databases
    - can use data models that are difficult to implement on disk

### Transaction Processing or Analytic?

ACID - atomicity, consistency, isolation, durability


# Ch 3 (pages 90 - )

This section covers the characteristics of keeping everything in memory. In this case the index still must reside in memory. The advantage isn't so much 
the RAM speed as much as the format of the data.

- ACID : atomicity, consistency, isolation, durability
- OLTP - online transaction processing
- OLAP - online analytics processing

Transaction processing OLTP and Analytic systems OLAP are compared and contrasted. 

OLTP - small number of records per query
customer facing representation
latest state

OLAP - large number records / aggregate data per query
event stream / bulk import
business analyst facing
historical events over time

Data warehousing

Sometimes a read only copy of the OLTP so that analysts can query w/o interrupting performance.

ETL - extract transfer load (getting data into the data warehouse)

Stars and Snowflakes - 

