## Env Merge

Tool for automatically filling your `.env` file from a `.env.template`

#### Example

`.env.template`
```
# Template for your env
EXAMPLE=value
TEST_VAR=123 #comment
```

Run using cargo:
```sh
$ cargo run -- --t .env.template
```

This will create a `.env` with the content:
```
EXAMPLE=value
TEST_VAR=123
```

Now when something is added to the template
`.env.template`
```
...
NEW_VAR=my_value
```

Your .env adds that value

You can also pass multiple templates:
```sh
$ cargo run -- --t .env.template,.env.overwrite.template
```
note: Templates to the right overwrite values passed before

Feel free to compile and add this as part of your build process to keep envs up to date 🎉
