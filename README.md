# Couchbase Inspector
Various tools to help you figure out whats going on between your app
and Couchbase Server. Work in progress, it will eat your laundry.

# Usage
The only thing that really works for now is sniffing N1QL queries from
the Java SDK to a Couchbase Server running on localhost.

### Installation
You need to have rust installed, use [rustup](https://www.rustup.rs/) 
if you haven't.

Also, make sure `libpcap` is available.

```
$ git clone https://github.com/daschl/couchbase-inspector.git
$ cd couchbase-inspector
```

Build the binary.

```
$ cargo build --release
```

Run the binary. Make sure it points to the right interface and also
you might need to use `sudo` since you are sniffing on interfaces.

I got this java code running to fire some queries:

```java
public static void main(String[] args) throws Exception {
    CouchbaseCluster cluster = CouchbaseCluster.create();
    Bucket bucket = cluster.openBucket("beer-sample");
    Random rand = new Random();
    while(true) {
        bucket.query(N1qlQuery.simple("SELECT * FROM `beer-sample` LIMIT " + rand.nextInt(1000)));
        Thread.sleep(500);
    }
}
```

And running it looks like this:

```
$ sudo target/release/couchbase-inspector -i lo0
Password:
{"statement":"SELECT * FROM `beer-sample` LIMIT 927","timeout":"75000ms"} took 11ms
{"statement":"SELECT * FROM `beer-sample` LIMIT 595","timeout":"75000ms"} took 11ms
{"statement":"SELECT * FROM `beer-sample` LIMIT 93","timeout":"75000ms"} took 9ms
{"statement":"SELECT * FROM `beer-sample` LIMIT 924","timeout":"75000ms"} took 9ms
{"statement":"SELECT * FROM `beer-sample` LIMIT 474","timeout":"75000ms"} took 10ms
{"statement":"SELECT * FROM `beer-sample` LIMIT 940","timeout":"75000ms"} took 10ms
{"statement":"SELECT * FROM `beer-sample` LIMIT 767","timeout":"75000ms"} took 9ms
{"statement":"SELECT * FROM `beer-sample` LIMIT 671","timeout":"75000ms"} took 10ms
{"statement":"SELECT * FROM `beer-sample` LIMIT 430","timeout":"75000ms"} took 11ms
```