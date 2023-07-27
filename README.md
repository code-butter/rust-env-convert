# Env Convert

A convenience library for dealing with environment variables, handling basic type conversions and default values.

## Getting values

`get_env_var` takes the environment variable name and the default value as a string if it does not exist. 
It returns an `EnvVar` that can be converted into a `Result` with the conversion type. You can then use that
result to detect if the conversion happened successfully. 

```rust
let max_connections_result: Result<u32,_> = get_env_var("MAX_DB_CONNECTIONS", "5".into_owned()).into();
let max_connections = max_connections_result.expect("MAX_DB_CONNECTIONS must be an integer");
```

## Conversions

The following conversions are implemented in this library: 

* String
* u32
* i16

To implement your own, you can implement the `From<EnvVar>` trait for `Result<YourType, EnvVarConversionError>`. 


## Versions

Until this reaches version 1, I will make an attempt to keep the API stable but no guarantees. After that this 
library will follow semantic versioning.