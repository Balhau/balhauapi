# BalhauAPI

## Introduction 

This component consists of a set of webservices implemented in Rust that will feed some of applications that will be deployed on the [raspberry kubernetes cluster](https://github.com/balhau/kuber). 

## Why Rust?

Being a Java developer why Rust? Aside from the obvious reason that it is always a good thing to learn a new programming language, the main reason for Rust for this particular problem (serve as a backend layer for programs running on a raspberry arquitecture) is their performance characteristics. Rust programs run natively. There is no garbage collector and no VirtualMachine overhead. Rust programs have a small memory footprint. A nice feature of Rust is their orthogonality. You can program low level because you are talking directly to the machine you can integrate easily native libraries with the [FFI](https://doc.rust-lang.org/book/first-edition/ffi.html) but you can program high level also. You got a nice and modern [build system](https://github.com/rust-lang/cargo) which helps you to develop easily your libs and programs. 

Could we develop the backend in python, java, nodejs, ruby or go? Definitely but all of them have some things that keep me away.

> * **Java** -- While a very performant language it is a little bloated and also has a high memory footprint. It is very distant from the operating system and for low level programming (aka hacking) you need extra work with JNI or JNA implementation, which is not a very nice way of deal with FFI
> * **Python** -- While a very nice programming language it is not so performant as java or rust. You cannot get the benefits of multicore processing as transparently as java or rust. Aside from this it is a lovely programming language to work with. It comes on 2nd place on my personal ranking for this problem in particular
> * **Javascript/NodeJS** -- While it is very convenient to work with nodejs due to the proximity with the front end development, there are some points which don't seduce me. First of all I don't like much of the philosophy of the community which is too much disrusptive. Backwards compatibility is not in their dictionary and for backend services this is not a thing that I like. It is also not the most performant language and was designed to solve a particular set of problems and not so generic as I want for this particular problem.

> * **Ruby** -- Honestly I never worked enough with ruby to have a proper opinion. But the arbritary overloading of operators don't seduce me also. And honestely I'm not so curious about it as I should (sorry my fault)

For more information on performance information you can [check this](http://benchmarksgame.alioth.debian.org/u64q/rust.html).


## Services

In the near (hopefully) future this component will replace some tolling done previously in python and, eventually, java.

* We got a previous python pip package that was a microservice responsible to automate some tasks defined as REST API. As example of services were the download of media from youtube, by using youtube-dl as the backend. This is being ported to rust (just a process launcher exposed as a service) 
* We got some content we need to ingest. Some tooling on ereader interaction and bookmarks management. This is being developed here also



## Automation

Some automation routines were implemented as a web service call. For instance to reboot some
network host that is registered under remotes in the configuration yaml file

    ---
    configs:
      remotes:
          - name: The Remote Name
            host: <hostname> //--> 192.168.1.169:122
            user: <ssh_user>
            pass: <ssh_password>
      webserver:
        binding_host: 0.0.0.0
        port: 5000
        log: false
        env: prod
      database:
        hostname: 192.168.1.14
        schema: postgres
        port: 5432
        user: <posgres_user>
        pass: <posgres_password>
      folders:
        mediafolder: /tmp/whynot


You can do something like this
    
    curl -H "Content-Type: application/json" -X POST -d '{ "host" : "pi1" }' http://localhost:5000/automation/reboot