# Api call 

We want to list all articles related to a keyword by making a call to a news API

## :dart: Objectives

* serialization / deserialization
* make http request

## Our API

We are going to use Hacker news API described here : 

https://hn.algolia.com/api

The results are in Json

An example with Curl :

```bash
curl -i http://hn.algolia.com/api/v1/search?query=rust
```

Header Content-Type 

```bash
HTTP/2 200
date: Wed, 05 Apr 2023 12:31:00 GMT
content-type: application/json; charset=utf-8

```

Body content

```bash
{

    hits:
    [
        {
            title: "Y Combinator",
            url: "http://ycombinator.com",
            author: "pg",
            points: 57,
            story_text: null,
            comment_text: null,
            _tags: 
            [
                "story"
            ],
            num_comments: 2,
            objectID: "1",
            _highlightResult: 
            {
                title: 
                {
                    value: "Y Combinator",
                    matchLevel: "none",
                    matchedWords: [ ]
                },
                url: 
                {
                    value: "http://ycombinator.com",
                    matchLevel: "none",
                    matchedWords: [ ]
                },
                author: 
                {
                    value: "<em>pg</em>",
                    matchLevel: "full",
                    matchedWords: 
                    [
                        "pg"
                    ]
                }
            }
        }, [...]
    ],
    page: 0,
    nbHits: 11,
    nbPages: 1,
    hitsPerPage: 20,
    processingTimeMS: 1,
    query: "pg",
    params: "query=pg"

}
```

We need to parse the body of the HTTP response provided as Json String into our data structure in Rust 

=> This a `Deserialisation` 

## Serialisation / deserialization

[Serde](https://serde.rs/) is a framework for serializing and deserializing Rust data structures efficiently and generically.


Serde provides a derive macro to generate implementations of the Serialize and Deserialize traits for data structures defined in your crate, allowing them to be represented conveniently in all of Serde's data formats.

Add Serde dependency with derive feature : 

```bash
cargo add serde -F derive
```

Our data structure is composed of numbers of hits and a collection of articles

We need to define our data structure representing the Json Response : 

```rust
#[derive(Debug, Deserialize)]
pub struct Article {
    pub title: String,
}
```

:bulb: we don't need to map every field. Others fields are ignored.

```rust
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub nb_hits: u64,
    pub hits: Vec<Article>,
}

```

:bulb: We use a macro `rename_all` to rename deserialized string from a camel case convention to a Rust snake case convention



:books: Additional resources

* [Serde documentation](https://doc.rust-lang.org/cargo/commands/cargo-add.html) (ex: specify version, features,etc)




## Make an HTTP request

The [reqwest](https://docs.rs/reqwest/latest/reqwest/) crate provides a convenient, higher-level HTTP Client. 

We need to add our library to project :

```basH
cargo add reqwest -F json blocking
```

:bulb: Reqwest supports async and blocking requests. For now, we use a blocking client.


Example of GET Http request : 

```rust
// format valid URL
let query = format!("http://hn.algolia.com/api/v1/search?query={}", keyword);
// execute HTTP call
let http_response: Result<Result, Error> = reqwest::blocking::get(query);
// Try to deserialize if response success
match http_response {
    Ok(response) => response.json::<SearchResult>(),
    Err(e) => Err(e),
}
```
We have to deal with two types of Error :
 * error when doing HTTP call
 * error when deserialize 

When first error occurs we don't want to try deserialization
        
To avoid `spaghetti code` we can use `question mark  ?` operator

```rust
let query = format!("http://hn.algolia.com/api/v1/search?query={}", keyword);
let http_response = reqwest::blocking::get(query)?;
let search_results: SearchResult = http_response.json::<SearchResult>()?;
```

:books: Additional resources

* [Reqwest documentation](https://docs.rs/reqwest/latest/reqwest/)
* [Question mark operator](https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html)


### :pencil: Exercice : Update application to add a search command

We expect to make a search with a keyword :

```bash
cargo run -- search --keyword <whatever>
```


:bulb: Tips : 
* update Command enum 
* implement the `search_news` function in a new `search` module
* manage errors messages 
* create a function to display results as list


### :pencil: Exercice : display search results as table


Use [Cli Table](https://docs.rs/cli-table/latest/cli_table/index.html#) to display result as table


```bash
 cargo run -- search --keyword rust

+----------------------------------------------------------------------------+
| Title                                                                      |
+----------------------------------------------------------------------------+
| Why Discord is switching from Go to Rust                                   |
+----------------------------------------------------------------------------+
| A half-hour to learn Rust                                                  |
+----------------------------------------------------------------------------+

```


## :clap: Congrats

you can connect your app to any API 

Check a solution with unit tests [here](./solution/src/main.rs) 

## :pencil: Summary

What you learned 

* serialization and deserialization with serde
* make http call with reqwest
* use question mark to manage Errors


