# mark-dir

Mark directories. Just like bookmarks.

<!-- TOC -->

* [Usage](#usage)
    * [Help](#help)
    * [Mark a directory](#mark-a-directory)
    * [Get a marked directory](#get-a-marked-directory)
    * [Remove a marked directory](#remove-a-marked-directory)
    * [Reset the config](#reset-the-config)

<!-- TOC -->

## Usage

### Help

Execute `mark-dir --help` to get a list and explanation of all available commands.

```shell
mark-dir --help
```

### Mark a directory

```shell
mark-dir <dir> <key>...
```

> **Note:** If the key already exists, it will be overwritten.

#### Examples

```shell
mark-dir ~/projects p
```

```shell
mark-dir ~/projects/mark-dir p md
```

```shell
mark-dir ~/projects/mark-dir/src p md s
```

### Get a marked directory

```shell
mark-dir -g <key>...
mark-dir --get <key>...
```

#### Examples

```shell
mark-dir --get p   # prints ~/projects
```

```shell
mark-dir -g p md   # prints ~/projects/mark-dir
```

```shell
mark-dir -g p md s   # prints ~/projects/mark-dir/src
```

### Remove a marked directory

```shell
mark-dir -r <key>...
mark-dir --remove <key>...
```

#### Examples

```shell
mark-dir --remove p   # removes ~/projects for p
```

```shell
mark-dir -r p md   # removes ~/projects/mark-dir for p md
```

```shell
mark-dir -r p md s   # removes ~/projects/mark-dir/src for p md s
```

### Reset the config

```shell
mark-dir --reset-config
```

> **Note:** This will also reset all marked directories.

#### Examples

```shell
mark-dir --reset-config   # resets the config file
```
