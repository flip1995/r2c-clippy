# Feedback r2c

## Docs

- Button for next page instead of link at end of page
- "In a directory where you want to create your new analyzer, run:" suggests
    that the files of the analyzer are directly created in the current dir, but
    a dir is created
- No instructions for unittest.sh
- How is output.json handled on large scale? Will it be uploaded in the database
    and overridden with each run of the analyzer for every project?
- How to find the constants for the results analyzer?

## Tool

- Every programming language can be used to run the analysis -> big plus
- Offer to upload a script/program to recreate input data over the Web interface
- Use latest commit, when no commit_hash is specified in input.json. Don't know
    if this already is the case though.

## Troubleshooting

- Couldn't r2c push, because of [docker
    error](https://github.com/docker/for-linux/issues/711), this could be
    resolved by starting the docker daemon with `dockerd -s overlay`
- Couldn't connect to postgresql server:
    ```
    psql: FATAL:  password authentication failed for user "notebook_user"
    FATAL:  password authentication failed for user "notebook_user"
    ```
