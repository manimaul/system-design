# TOC, Preface and Ch1 (pages 1-6):

"Data Intense" means data throughput rather over compute.

The book talks about what it indends to cover being understanding the core components and trade offs of various technology components used in system design. The book intends to cover various composition of these components.

Components:
- databases
- search indexes
- caches
- message queues

Starting in chapter 1 there are 3 poignant attributes of data intense applications pointed out. 
- Reliability
  - Systems should be fault tolerant to hardware failures and human failures.
Scalability
  - Systems should be able to be easiliy scaled to handle increased loads.
Maintainability
  - Systems should be able to be worked on productively by various and changing individuals. They should be able to be easily diagnosed, understood and observed. 

# Ch1 (pages 6-22)

This chapter briefs the 3 topic of the book Reliability, Scalability, Maintainability.

Reliability is defined as an application that functions as it should, tolerates faults, and performs well under the expansion of load and throughput. Scalability is defined as the ability of the system to cope with increased load by the system growing in a particular way. In order to design for scalibility you first need to describe load, performance and measure latency. Maintainability is defined in 3 terms Operability, Simplicity and Evolvability. Operability is making the system easy to operate. Simplicity is making it easy for new engineers to understand and removing complexity. Evolvability is making it easy for engineers to make changes.

- terms:
  - `latency` duration which it is latent, awaiting service
  - `tail latencies` = p9x percentiles
  - `median` = p50 / 50th percentile
  - `head-of-line-blocking` when a line of packets is held up by the first packet
