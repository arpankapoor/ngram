# stats

- server specs:
  - 20 core Intel(R) Xeon(R) CPU E5-2670 v2 @ 2.50GHz
  - 128GiB RAM
  - rotating disk
  - Linux version 4.15.0-159-generic #167-Ubuntu

- unigram

  ```console
  $ /usr/bin/time -v ./target/release/freq /data/arpank/data/hi/hi.txt 1 > unigrams.txt
      Command being timed: "./target/release/freq /data/arpank/data/hi/hi.    txt 1"
      User time (seconds): 550.79
      System time (seconds): 8.57
      Percent of CPU this job got: 99%
      Elapsed (wall clock) time (h:mm:ss or m:ss): 9:19.38
      Average shared text size (kbytes): 0
      Average unshared data size (kbytes): 0
      Average stack size (kbytes): 0
      Average total size (kbytes): 0
      Maximum resident set size (kbytes): 533672
      Average resident set size (kbytes): 0
      Major (requiring I/O) page faults: 0
      Minor (reclaiming a frame) page faults: 759133
      Voluntary context switches: 1
      Involuntary context switches: 749
      Swaps: 0
      File system inputs: 0
      File system outputs: 8
      Socket messages sent: 0
      Socket messages received: 0
      Signals delivered: 0
      Page size (bytes): 4096
      Exit status: 0
  ```

- bigram

  ```console
  $ /usr/bin/time -v ./target/release/freq /data/arpank/data/hi/hi.txt 2 > bigrams.txt 
  	Command being timed: "./target/release/freq /data/arpank/data/hi/hi.txt 2"
  	User time (seconds): 1311.65
  	System time (seconds): 73.94
  	Percent of CPU this job got: 99%
  	Elapsed (wall clock) time (h:mm:ss or m:ss): 23:05.65
  	Average shared text size (kbytes): 0
  	Average unshared data size (kbytes): 0
  	Average stack size (kbytes): 0
  	Average total size (kbytes): 0
  	Maximum resident set size (kbytes): 9964644
  	Average resident set size (kbytes): 0
  	Major (requiring I/O) page faults: 0
  	Minor (reclaiming a frame) page faults: 46942292
  	Voluntary context switches: 1
  	Involuntary context switches: 3532
  	Swaps: 0
  	File system inputs: 0
  	File system outputs: 8
  	Socket messages sent: 0
  	Socket messages received: 0
  	Signals delivered: 0
  	Page size (bytes): 4096
  	Exit status: 0
  ```

- trigram

  ```console
  $ /usr/bin/time -v ./target/release/freq /data/arpank/data/hi/hi.txt 3 > trigrams.txt
        Command being timed: "./target/release/freq /data/arpank/data/hi/hi.txt 3"
        User time (seconds): 1658.44
        System time (seconds): 65.36
        Percent of CPU this job got: 99%
        Elapsed (wall clock) time (h:mm:ss or m:ss): 28:44.01
        Average shared text size (kbytes): 0
        Average unshared data size (kbytes): 0
        Average stack size (kbytes): 0
        Average total size (kbytes): 0
        Maximum resident set size (kbytes): 44378284
        Average resident set size (kbytes): 0
        Major (requiring I/O) page faults: 0
        Minor (reclaiming a frame) page faults: 26108939
        Voluntary context switches: 2
        Involuntary context switches: 2298
        Swaps: 0
        File system inputs: 80
        File system outputs: 8
        Socket messages sent: 0
        Socket messages received: 0
        Signals delivered: 0
        Page size (bytes): 4096
        Exit status: 0

  ```
