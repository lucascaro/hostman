# Hostman

[![Build Status](https://travis-ci.com/lucascaro/hostman.svg?branch=master)](https://travis-ci.com/lucascaro/hostman)
[![Crates.io](https://img.shields.io/crates/v/hostman.svg)](https://crates.io/crates/hostman)

Hostman is a command line manager for `/etc/hosts`.

## Installation

### using `cargo`

```shell
cargo install hostman
```

### Manual install

Download the [latest release for your architecture from github](https://github.com/lucascaro/hostman/releases/latest) and put it in a directory in your path.

### Install script

You can use the [trust install script](https://github.com/japaric/trust) to install this tool:

```shell
curl -LSfs https://japaric.github.io/trust/install.sh | \
    sh -s -- --git lucascaro/hostman
```

## Usage

Run the tool to get a usage summary:

```shell
hostman
```

### `hostman show`

Use this command to show your current hosts file.

### `hostman check`

Use this command to check if a host is in your hosts file:

```shell
$ hostman check localhost
# localhost is used to configure the loopback interface
127.0.0.1  localhost
::1  localhost
```

### `hostman add`

Add a new host to your hosts file.

```shell
hostman add <ip> <names> [comment]...
```

### `hostman remove`

Remove a host from your hosts file.

```shell
hostman remove <host>
```

### `hostman disable`

Disable (comment out) a host from your hosts file.

```shell
hostman disable <host>
```

### `hostman enable`

Enable a commented out host from your hosts file.

```shell
hostman enable <host>
```

### `hostman update`

Update the cli to the latest version.

```shell
hostman update
```
